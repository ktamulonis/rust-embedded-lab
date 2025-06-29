#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

use rp_pico::hal::{pac, clocks::init_clocks_and_plls, gpio::Pins, sio::Sio, watchdog::Watchdog};
use rp_pico::hal;
use rp_pico::XOSC_CRYSTAL_FREQ;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(peripherals.SIO);

    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // Use onboard LED (GPIO 25)
    let mut led_pin = pins.led.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        delay(500_000);
        led_pin.set_low().unwrap();
        delay(500_000);
    }
}

// Very simple busy-wait delay
fn delay(count: u32) {
    for _ in 0..count {
        cortex_m::asm::nop();
    }
}

