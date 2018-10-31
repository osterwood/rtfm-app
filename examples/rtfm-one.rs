//! An application with one task
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(extern_crate_item_prelude)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate panic_halt;
extern crate stm32f042_hal as hal;
extern crate stm32f0;

#[macro_use(exception)]
extern crate cortex_m_rt as rt;

use hal::prelude::*;
use hal::gpio::{Output, PushPull};
use hal::gpio::gpioc::PC13;

use rtfm::{app, Threshold};
use rt::ExceptionFrame;
use cortex_m::peripheral::syst::SystClkSource;
use stm32f0::stm32f0x2;


app! {
    device: stm32f0x2,

    // Here data resources are declared
    //
    // Data resources are static variables that are safe to share across tasks
    resources: {
        // Declaration of resources looks exactly like declaration of static
        // variables
        static ON: bool = false;
        static LED: PC13<Output<PushPull>>;
    },

    // Here tasks are declared
    //
    // Each task corresponds to an interrupt or an exception. Every time the
    // interrupt or exception becomes *pending* the corresponding task handler
    // will be executed.
    tasks: {
        // Here we declare that we'll use the SYS_TICK exception as a task
        SYS_TICK: {
            // Path to the task handler
            path: sys_tick,

            // These are the resources this task has access to.
            //
            // The resources listed here must also appear in `app.resources`
            resources: [ON, LED],
        },
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

fn init(mut p: init::Peripherals, r: init::Resources) -> init::LateResources {
    // `init` can modify all the `resources` declared in `app!`
    r.ON;

    let mut rcc = p.device.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let mut gpioc = p.device.GPIOC.split();

    // configure the system timer to generate one interrupt every second
    p.core.SYST.set_clock_source(SystClkSource::Core);
    p.core.SYST.set_reload(8_000_000); // 1s
    p.core.SYST.enable_interrupt();
    p.core.SYST.enable_counter();

    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_low();

    init::LateResources {
        LED: led
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

// This is the task handler of the SYS_TICK exception
//
// `_t` is the preemption threshold token. We won't use it in this program.
//
// `r` is the set of resources this task has access to. `SYS_TICK::Resources`
// has one field per resource declared in `app!`.
#[allow(unsafe_code)]
fn sys_tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
    // toggle state
    *r.ON = !*r.ON;

    if *r.ON {
        r.LED.set_low();
    } else {
        r.LED.set_high();
    }
}