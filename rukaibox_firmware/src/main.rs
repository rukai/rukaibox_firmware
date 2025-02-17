#![no_std]
#![no_main]

mod config;
mod input;
mod profile;
mod socd;

use cortex_m::delay::Delay;
use input::ButtonInput;
use joybus_pio::{GamecubeController, JoybusPio};
// set the panic handler
use bsp::entry;
use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;
// This board has the same pinout as a pico so the pico bsp is handy.
use bsp::hal::{
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use profile::MapProfile;
use rp_pico as bsp;
use rp2040_hal::{
    Timer,
    gpio::{FunctionSio, Pin, PullDown, SioOutput, bank0::Gpio25},
    rom_data::reset_to_usb_boot,
};
use rukaibox_config::Config;

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

    let mut input = ButtonInput {
        left_index: pins.gpio2.into_pull_up_input().into_dyn_pin(),
        left_middle: pins.gpio3.into_pull_up_input().into_dyn_pin(),
        left_ring: pins.gpio4.into_pull_up_input().into_dyn_pin(),
        left_pinky: pins.gpio5.into_pull_up_input().into_dyn_pin(),

        left_middle_2: pins.gpio1.into_pull_up_input().into_dyn_pin(),

        left_thumb_left: pins.gpio6.into_pull_up_input().into_dyn_pin(),
        left_thumb_right: pins.gpio7.into_pull_up_input().into_dyn_pin(),

        right_index: pins.gpio26.into_pull_up_input().into_dyn_pin(),
        right_middle: pins.gpio21.into_pull_up_input().into_dyn_pin(),
        right_ring: pins.gpio19.into_pull_up_input().into_dyn_pin(),
        right_pinky: pins.gpio17.into_pull_up_input().into_dyn_pin(),

        right_index_2: pins.gpio27.into_pull_up_input().into_dyn_pin(),
        right_middle_2: pins.gpio22.into_pull_up_input().into_dyn_pin(),
        right_ring_2: pins.gpio20.into_pull_up_input().into_dyn_pin(),
        right_pinky_2: pins.gpio18.into_pull_up_input().into_dyn_pin(),

        right_thumb_left: pins.gpio13.into_pull_up_input().into_dyn_pin(),
        right_thumb_right: pins.gpio16.into_pull_up_input().into_dyn_pin(),
        right_thumb_up: pins.gpio12.into_pull_up_input().into_dyn_pin(),
        right_thumb_down: pins.gpio15.into_pull_up_input().into_dyn_pin(),
        right_thumb_middle: pins.gpio14.into_pull_up_input().into_dyn_pin(),

        start,
    };

    let Ok(config) = config::load() else {
        // Failed to parse config, set 5s blinky for diagnostics
        loop {
            led_pin.set_high().unwrap();
            delay.delay_ms(5000);
            if input.start.is_low().unwrap_or(true) {
                reset_to_usb_boot(0, 0);
            }

            led_pin.set_low().unwrap();
            delay.delay_ms(5000);
            if input.start.is_low().unwrap_or(true) {
                reset_to_usb_boot(0, 0);
            }
        }
    };

    let profile = MapProfile::new(config.profiles.first().unwrap());

    let pio = JoybusPio::new(pins.gpio28, pac.PIO0, &mut pac.RESETS, clocks);
    match GamecubeController::try_new(pio, &timer, &mut delay) {
        Ok(gamecube_controller) => {
            run_gamecube_loop(
                led_pin,
                gamecube_controller,
                &timer,
                &mut delay,
                profile,
                input,
                &config,
            );
        }
        Err(_pio) => {
            run_pc_loop(led_pin, &mut delay, input);
        }
    }
}

fn run_gamecube_loop(
    mut led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
    mut gamecube_controller: GamecubeController,
    timer: &Timer,
    delay: &mut Delay,
    mut profile: MapProfile,
    mut input: ButtonInput,
    config: &Config,
) -> ! {
    let mut counter = 0u32;
    loop {
        counter += 1;
        if counter % 10 < 5 {
            led_pin.set_high().unwrap();
        } else {
            led_pin.set_low().unwrap();
        }

        gamecube_controller.wait_for_poll_start(timer, delay);
        let input_results = input.get_pin_state();
        profile.change_profile(&input_results, config);
        let report = profile.map_to_gamecube(&input_results);
        gamecube_controller.respond_to_poll(timer, delay, report);
    }
}

fn run_pc_loop(
    mut led_pin: Pin<Gpio25, FunctionSio<SioOutput>, PullDown>,
    delay: &mut Delay,
    mut input: ButtonInput,
) -> ! {
    let mut blink = true;
    loop {
        // slowly blink pins to show gamecube not detected
        blink = !blink;
        if blink {
            led_pin.set_high().unwrap();
        } else {
            led_pin.set_low().unwrap();
        }

        // we are probably connected to a PC so allow flashing via start button
        if input.start.is_low().unwrap_or(true) {
            reset_to_usb_boot(0, 0);
        }

        // Reattempt gamecube connection.
        // TODO: sounded like a cool idea, but ran into problems and not sure I want it anyway.
        // match GamecubeController::try_new(pio, timer, delay) {
        //     Ok(gamecube_controller) => {
        //         run_gamecube_loop(led_pin, gamecube_controller, timer, delay, mapping);
        //     }
        //     Err(_pio) => pio = _pio,
        // }

        delay.delay_ms(1000);
    }
}
