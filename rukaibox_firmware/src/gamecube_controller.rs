//! Excellent writeup on the GC controller protocol:
//! https://jefflongo.dev/posts/gc-controller-reverse-engineering-part-1/

use cortex_m::delay::Delay;
use embedded_hal::digital::InputPin;
use pio::{Instruction, InstructionOperands, Program, ProgramWithDefines, SideSet, Wrap};
// This board has the same pinout as a pico so the pico bsp is handy.
use rp_pico::{
    self as bsp,
    pac::{PIO0, RESETS},
};
use rp2040_hal::{
    Timer,
    clocks::ClocksManager,
    gpio::{FunctionNull, FunctionPio0, Pin, PullDown, bank0::Gpio28},
    pio::{PIOExt, Running, Rx, SM0, ShiftDirection, StateMachine, Tx},
};

use bsp::hal::clocks::Clock;

pub struct ConsolePio {
    data_pin: Pin<Gpio28, FunctionPio0, PullDown>,
    tx: Tx<(PIO0, SM0)>,
    rx: Rx<(PIO0, SM0)>,
    sm: StateMachine<(PIO0, SM0), Running>,
}

impl ConsolePio {
    pub fn new(
        data_pin: Pin<Gpio28, FunctionNull, PullDown>,
        pio0: PIO0,
        resets: &mut RESETS,
        clocks: ClocksManager,
    ) -> ConsolePio {
        let data_pin: Pin<_, FunctionPio0, PullDown> = data_pin.into_function();
        let data_pin_num = data_pin.id().num;

        //     let program = pio_proc::pio_asm!(
        //         "
        // .program joybus

        // .define public T1 10
        // .define public T2 20
        // .define public T3 10

        // ; Autopush with 8 bit ISR threshold
        // public read:
        //     set pindirs 0                   ; Set pin to input
        // read_loop:
        //     wait 0 pin 0 [T1 + T2 / 2 - 1]  ; Wait for falling edge, then wait until halfway through the 2uS which represents the bit value
        //     in pins, 1                      ; Read bit value
        //     wait 1 pin 0                    ; Done reading, so make sure we wait for the line to go high again before restarting the loop
        //     jmp read_loop

        // ; 9 bit OSR threshold, no autopull because it interferes with !osre
        // public write:
        //     set pindirs 1           ; Set pin to output
        // write_loop:
        //     set pins, 1             ; Set line high for at least 1uS to end pulse
        //     pull ifempty block      ; Fetch next byte into OSR if we are done with the current one
        //     out x, 1                ; Get bit
        //     jmp !osre write_bit     ; If we aren't on the 9th bit, just write the bit
        //     jmp x!=y write_stop_bit ; If we are on the 9th bit and it's a 1 that indicates stop bit so write it
        //     pull ifempty block      ; If we are on the 9th bit and it's a 0 then we should skip to the next byte
        //     out x, 1                ; Get first bit of the next byte
        //     jmp write_bit_fast      ; Write it, skipping some of the delays because we spent so much time checking the 9th bit
        // write_bit:
        //     nop [3]
        // write_bit_fast:
        //     nop [T3 - 9]
        //     set pins, 0 [T1 - 1]    ; Pulse always starts with low for 1uS
        //     mov pins, x [T2 - 2]    ; Set line according to bit value for 2uS
        //     jmp write_loop
        // write_stop_bit:
        //     nop [T3 - 6]
        //     set pins, 0 [T1 - 1]
        //     set pins, 1 [T2 - 2]
        //     jmp read
        // "
        //     );

        // pio proc macro is broken with cargo bin deps nightly feature.
        // work around this by manually creating program.
        let raw_program: [u16; 32] = [
            //     .wrap_target
            0xe080, //  0: set    pindirs, 0
            0x3320, //  1: wait   0 pin, 0               [19]
            0x4001, //  2: in     pins, 1
            0x20a0, //  3: wait   1 pin, 0
            0x0001, //  4: jmp    1
            0xe081, //  5: set    pindirs, 1
            0xe001, //  6: set    pins, 1
            0x80e0, //  7: pull   ifempty block
            0x6021, //  8: out    x, 1
            0x00ee, //  9: jmp    !osre, 14
            0x00b3, // 10: jmp    x != y, 19
            0x80e0, // 11: pull   ifempty block
            0x6021, // 12: out    x, 1
            0x000f, // 13: jmp    15
            0xa342, // 14: nop                           [3]
            0xa142, // 15: nop                           [1]
            0xe900, // 16: set    pins, 0                [9]
            0xb201, // 17: mov    pins, x                [18]
            0x0006, // 18: jmp    6
            0xa442, // 19: nop                           [4]
            0xe900, // 20: set    pins, 0                [9]
            0xf201, // 21: set    pins, 1                [18]
            0x0000, // 22: jmp    0
            //     .wrap
            0x0000, // padding
            0x0000, // padding
            0x0000, // padding
            0x0000, // padding
            0x0000, // padding
            0x0000, // padding
            0x0000, // padding
            0x0000, // padding
            0x0000, // padding
        ];

        let program = ProgramWithDefines {
            program: Program {
                code: raw_program.into(),
                origin: Some(0),
                wrap: Wrap {
                    source: 22,
                    target: 0,
                },
                side_set: SideSet::default(),
            },
            public_defines: (),
        };

        let (mut pio, sm0, _, _, _) = pio0.split(resets);
        let installed = pio
        .install(&program.program)
        .unwrap()
        // TODO: do we need this or does rp2040_hal derive it for us?
        //.set_wrap()
        ;

        let bitrate = 250000;
        let cycles_per_bit = 10 + 20 + 10;
        let divisor = clocks.system_clock.freq().to_Hz() as f32 / (cycles_per_bit * bitrate) as f32;

        let (sm, rx, tx) = rp2040_hal::pio::PIOBuilder::from_installed_program(installed)
            .out_pins(data_pin_num, 1)
            .set_pins(data_pin_num, 1)
            .in_pin_base(data_pin_num)
            // out shift
            .out_shift_direction(ShiftDirection::Left)
            .autopull(false)
            .pull_threshold(9)
            // in shift
            .in_shift_direction(ShiftDirection::Left)
            .autopush(true)
            .push_threshold(8)
            .clock_divisor(divisor)
            .build(sm0);
        let sm = sm.start();

        ConsolePio {
            tx,
            rx,
            sm,
            data_pin,
        }
    }
}

pub struct GamecubeController {
    pio: ConsolePio,
}

impl GamecubeController {
    pub fn try_new(
        mut pio: ConsolePio,
        timer: &Timer,
        delay: &mut Delay,
    ) -> Result<GamecubeController, ()> {
        pio.sm.exec_instruction(Instruction {
            operands: InstructionOperands::JMP {
                condition: pio::JmpCondition::Always,
                address: 0,
            },
            delay: 0,
            side_set: None,
        });

        let mut controller = GamecubeController { pio };

        match controller.recv(timer).map(GamecubeCommand::from) {
            Some(GamecubeCommand::Reset) | Some(GamecubeCommand::Probe) => {
                delay.delay_us(4);
                controller.send(&[9, 0, 3]);
            }
            Some(GamecubeCommand::Recalibrate) | Some(GamecubeCommand::Origin) => {
                delay.delay_us(4);
                // set perfect deadzone, we have no analog sticks
                // Apparently gc adapter ignores this though and uses the first poll response instead.
                controller.send(&[
                    0,   // butons1
                    1,   // butons2
                    128, // stick x
                    128, // stick y
                    128, // cstick x
                    128, // cstick y
                    0,   // left trigger
                    0,   // right trigger
                    0,   // reserved
                    0,   // reserved
                ]);
            }
            Some(GamecubeCommand::Poll) => {
                let report = [
                    0,   // butons1
                    1,   // butons2
                    128, // stick x
                    128, // stick y
                    128, // cstick x
                    128, // cstick y
                    0,   // left trigger
                    0,   // right trigger
                ];
                controller.respond_to_poll(timer, delay, &report);
            }
            Some(GamecubeCommand::Unknown) => {
                delay.delay_us(130);
                controller.restart_sm_for_read();
            }
            None => return Err(()),
        }

        Ok(controller)
    }

    pub fn wait_for_poll_start(&mut self, timer: &Timer, delay: &mut Delay) {
        loop {
            match self.recv(timer).map(GamecubeCommand::from) {
                Some(GamecubeCommand::Reset) | Some(GamecubeCommand::Probe) => {
                    delay.delay_us(4);
                    self.send(&[9, 0, 3]);
                }
                Some(GamecubeCommand::Recalibrate) | Some(GamecubeCommand::Origin) => {
                    delay.delay_us(4);
                    // set perfect deadzone, we have no analog sticks
                    // Apparently gc adapter ignores this though and uses the first poll response instead.
                    self.send(&[
                        0,   // butons1
                        1,   // butons2
                        128, // stick x
                        128, // stick y
                        128, // cstick x
                        128, // cstick y
                        0,   // left trigger
                        0,   // right trigger
                        0,   // reserved
                        0,   // reserved
                    ]);
                }
                Some(GamecubeCommand::Poll) => {
                    return;
                }
                Some(GamecubeCommand::Unknown) | None => {
                    delay.delay_us(130);
                    self.restart_sm_for_read();
                }
            }
        }
    }

    pub fn restart_sm_for_read(&mut self) {
        self.pio.sm.clear_fifos(); // TODO: this should probably occur inside the restart
        self.pio.sm.restart();
    }

    pub fn restart_sm_for_write(&mut self) {
        self.pio.sm.clear_fifos(); // TODO: this should probably occur inside the restart
        self.pio.sm.restart();
        self.pio.sm.exec_instruction(Instruction {
            operands: InstructionOperands::JMP {
                condition: pio::JmpCondition::Always,
                address: 5,
            },
            delay: 0,
            side_set: None,
        });
    }

    // pub fn restart_sm(pio: &mut ConsolePio, address: u8) {
    //     let (sm0, installed) = pio.sm.uninit(pio.rx, pio.tx);
    //     let data_pin_num = pio.data_pin.id().num;
    //     let bitrate = 250000;
    //     let cycles_per_bit = 10 + 20 + 10;
    //     let divisor = 1.0;
    //     //let divisor = clocks.system_clock.freq().to_Hz() as f32 / (cycles_per_bit * bitrate) as f32;
    //     let (sm, rx, tx) = rp2040_hal::pio::PIOBuilder::from_installed_program(installed)
    //         .out_pins(pio.data_pin.id().num, 1)
    //         .set_pins(data_pin_num, 1)
    //         .in_pin_base(data_pin_num)
    //         // out shift
    //         .out_shift_direction(ShiftDirection::Left)
    //         .autopull(false)
    //         .pull_threshold(9)
    //         // in shift
    //         .in_shift_direction(ShiftDirection::Left)
    //         .autopush(true)
    //         .push_threshold(8)
    //         .clock_divisor(divisor)
    //         .build(sm0);
    //     let sm = sm.start();

    //     pio.sm.exec_instruction(Instruction {
    //         operands: InstructionOperands::JMP {
    //             condition: pio::JmpCondition::Always,
    //             address: address,
    //         },
    //         delay: 0,
    //         side_set: None,
    //     });
    //     pio.sm = sm;
    // }

    pub fn respond_to_poll(&mut self, timer: &Timer, delay: &mut Delay, report: &[u8]) {
        // TODO: optimization to read the values 40us just before they are sent off
        delay.delay_us(40);

        self.recv(timer);
        self.recv(timer);
        delay.delay_us(4);

        self.send(report);
    }

    pub fn recv(&mut self, timer: &Timer) -> Option<u8> {
        let instant = timer.get_counter();

        loop {
            match self.pio.rx.read() {
                Some(value) => return Some(value as u8),
                None => {
                    if timer
                        .get_counter()
                        .checked_duration_since(instant)
                        .unwrap()
                        .ticks()
                        // TODO: high value used for testing
                        > 2000000
                    {
                        return None;
                    }
                }
            }
        }
    }

    pub fn send(&mut self, values: &[u8]) {
        // wait for line to be high
        while self.pio.data_pin.as_input().is_low().unwrap() {}

        self.restart_sm_for_write();

        for (i, value) in values.iter().enumerate() {
            let stop = if i == values.len() - 1 { 1 } else { 0 };
            let word = ((*value as u32) << 24) | (stop as u32) << 23;

            while self.pio.tx.is_full() {}
            self.pio.tx.write(word);
        }
    }
}

enum GamecubeCommand {
    Probe = 0x00,
    Poll = 0x40,
    Origin = 0x41,
    Recalibrate = 0x42,
    Reset = 0xFF,
    Unknown,
}

impl GamecubeCommand {
    fn from(value: u8) -> Self {
        match value {
            0x00 => GamecubeCommand::Probe,
            0xFF => GamecubeCommand::Reset,
            0x41 => GamecubeCommand::Origin,
            0x42 => GamecubeCommand::Recalibrate,
            0x40 => GamecubeCommand::Poll,
            _ => GamecubeCommand::Unknown,
        }
    }
}
