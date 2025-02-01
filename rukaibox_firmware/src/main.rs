#![no_std]
#![no_main]

mod gamecube_controller;
mod input;
mod project_plus;
mod socd;

use gamecube_controller::{ConsolePio, GamecubeController};
use input::ButtonInput;
// set the panic handler
use panic_halt as _;

use bsp::entry;
use embedded_hal::digital::{InputPin, OutputPin};

use project_plus::ProjectPlusMapping;
// This board has the same pinout as a pico so the pico bsp is handy.
use rp_pico as bsp;
use rp2040_hal::{Timer, rom_data::reset_to_usb_boot};

use bsp::hal::{
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();
    let mut start = pins.gpio0.into_pull_up_input().into_dyn_pin();

    // TODO: not sure why this is needed
    delay.delay_ms(10);

    if start.is_low().unwrap_or(true) {
        reset_to_usb_boot(0, 0);
    }

    for _ in 0..10 {
        led_pin.set_high().unwrap();
        delay.delay_ms(100);
        led_pin.set_low().unwrap();
        delay.delay_ms(100);
    }

    let input = ButtonInput {
        left_hand_index: pins.gpio2.into_pull_up_input().into_dyn_pin(),
        left_hand_middle: pins.gpio3.into_pull_up_input().into_dyn_pin(),
        left_hand_ring: pins.gpio4.into_pull_up_input().into_dyn_pin(),
        left_hand_pinky: pins.gpio5.into_pull_up_input().into_dyn_pin(),

        left_hand_middle_2: pins.gpio17.into_pull_up_input().into_dyn_pin(),

        left_hand_thumb_left: pins.gpio6.into_pull_up_input().into_dyn_pin(),
        left_hand_thumb_right: pins.gpio7.into_pull_up_input().into_dyn_pin(),

        right_hand_index: pins.gpio26.into_pull_up_input().into_dyn_pin(),
        right_hand_middle: pins.gpio21.into_pull_up_input().into_dyn_pin(),
        right_hand_ring: pins.gpio19.into_pull_up_input().into_dyn_pin(),
        right_hand_pinky: pins.gpio1.into_pull_up_input().into_dyn_pin(),

        right_hand_index_2: pins.gpio27.into_pull_up_input().into_dyn_pin(),
        right_hand_middle_2: pins.gpio22.into_pull_up_input().into_dyn_pin(),
        right_hand_ring_2: pins.gpio20.into_pull_up_input().into_dyn_pin(),
        right_hand_pinky_2: pins.gpio18.into_pull_up_input().into_dyn_pin(),

        right_hand_thumb_left: pins.gpio13.into_pull_up_input().into_dyn_pin(),
        right_hand_thumb_right: pins.gpio16.into_pull_up_input().into_dyn_pin(),
        right_hand_thumb_up: pins.gpio12.into_pull_up_input().into_dyn_pin(),
        right_hand_thumb_down: pins.gpio15.into_pull_up_input().into_dyn_pin(),
        right_hand_thumb_middle: pins.gpio14.into_pull_up_input().into_dyn_pin(),

        start,
    };

    let mut mapping = ProjectPlusMapping::new(input);

    let mut counter = 0u32;
    let pio = ConsolePio::new(pins.gpio28, pac.PIO0, &mut pac.RESETS, clocks);
    if let Ok(mut gamecube_controller) = GamecubeController::try_new(pio, &timer, &mut delay) {
        loop {
            counter += 1;
            if counter % 10 < 5 {
                led_pin.set_high().unwrap();
            } else {
                led_pin.set_low().unwrap();
            }

            gamecube_controller.wait_for_poll_start(&timer, &mut delay);
            let report = mapping.poll();
            gamecube_controller.respond_to_poll(&timer, &mut delay, report);
        }
    } else {
        loop {
            // slowly blink pins to show gamecube not detected
            led_pin.set_high().unwrap();
            delay.delay_ms(2000);
            led_pin.set_low().unwrap();
            delay.delay_ms(2000);

            // we are probably connected to a PC so allow flashing via start button
            if mapping.input.start.is_low().unwrap_or(true) {
                reset_to_usb_boot(0, 0);
            }
        }
    }
}
