#![no_std]
#![no_main]

fn main() -> Result<(), &'static str> {
    // App logic goes here
    Ok(())
}

#[riscv_rt::entry]
fn entry() -> ! {
    device::entry(main);
}

mod device {
    use core::{
        fmt,
        panic::PanicInfo,
        sync::atomic::{self, Ordering},
    };

    use hifive1::{
        hal::{prelude::*, DeviceResources},
        pin, pins, sprintln, Led,
    };

    pub fn entry<T, E>(f: impl FnOnce() -> Result<T, E>) -> !
    where
        T: fmt::Debug,
        E: fmt::Display,
    {
        let device = unsafe { DeviceResources::steal() };
        let pins = device.pins;

        // Configure the device
        let clocks = hifive1::clock::configure(
            device.peripherals.PRCI,
            device.peripherals.AONCLK,
            320.mhz().into(),
        );

        hifive1::stdout::configure(
            device.peripherals.UART0,
            pin!(pins, uart0_tx),
            pin!(pins, uart0_rx),
            115_200.bps(),
            clocks,
        );

        let (red, green, blue) = pins!(pins, (led_red, led_green, led_blue));
        let (mut red, mut green, mut blue) = hifive1::rgb(red, green, blue);

        // Execute the given function
        Runner {
            failure: &mut red,
            success: &mut green,
            in_progress: &mut blue,
        }
        .exec(f);

        loop {
            atomic::compiler_fence(Ordering::SeqCst);
        }
    }

    #[inline(never)]
    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        // NOTE: This is actually not valid, the device API is designed around singletons,
        // expecting there to only ever be a single instance
        let device = unsafe { DeviceResources::steal() };
        let pins = device.pins;
        let (red, green, blue) = pins!(pins, (led_red, led_green, led_blue));
        let (mut red, mut green, mut blue) = hifive1::rgb(red, green, blue);

        Runner {
            failure: &mut red,
            success: &mut green,
            in_progress: &mut blue,
        }
        .failure();

        sprintln!("panic: {}", info);

        loop {
            atomic::compiler_fence(Ordering::SeqCst);
        }
    }

    struct Runner<'a> {
        failure: &'a mut dyn Led,
        success: &'a mut dyn Led,
        in_progress: &'a mut dyn Led,
    }

    impl<'a> Runner<'a> {
        fn in_progress(&mut self) {
            self.failure.off();
            self.success.off();

            self.in_progress.on();
        }

        fn success(&mut self) {
            self.failure.off();
            self.in_progress.off();

            self.success.on();
        }

        fn failure(&mut self) {
            self.success.off();
            self.in_progress.off();

            self.failure.on();
        }

        pub fn exec<T, E>(&mut self, f: impl FnOnce() -> Result<T, E>)
        where
            T: fmt::Debug,
            E: fmt::Display,
        {
            self.in_progress();

            match f() {
                Ok(t) => {
                    self.success();
                    sprintln!("ok: {:?}", t);
                }
                Err(e) => {
                    self.failure();
                    sprintln!("err: {}", e);
                }
            }
        }
    }
}
