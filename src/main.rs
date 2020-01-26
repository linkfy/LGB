//Utils
use std::{thread, time};
use std::io::{self, Read};

/*
 _ __   __ _ _ __ __ _ _ __ ___   ___| |_ ___ _ __ ___
| '_ \ / _` | '__/ _` | '_ ` _ \ / _ | __/ _ | '__/ __|
| |_) | (_| | | | (_| | | | | | |  __| ||  __| |  \__ \
| .__/ \__,_|_|  \__,_|_| |_| |_|\___|\__\___|_|  |___/
|_| 
 */

// WINDOW  PARAMETERS:
extern crate minifb;
use minifb:: {Key, Window, WindowOptions, Scale, ScaleMode};

const HEIGHT: usize = 160;
const WIDTH: usize = 144;
const BACKGROUND: u32 = 0x8AAB19;
// END WINDOW

// SYSTEM PARAMETERS
mod instructions;
mod cpu;
mod rom;
mod ram;

use cpu::Cpu;
use rom::Rom;
use ram:: Ram;

const CLOCK_SPEED: i32 = 4194304;
const FRAME_RATE: i32 = 60;
// END SYSTEM PARAMETERS


/*
 _ __ ___   __ _(_)_ __
| '_ ` _ \ / _` | | '_ \
| | | | | | (_| | | | | |
|_| |_| |_|\__,_|_|_| |_|
 */
fn main() {
    
    //Create a new CPU
    let mut cpu = Cpu::new();
    //Create a RAM
    let mut ram = Ram::new();
    //Load a ROM
    let mut rom = Rom::load("ROMS/dmg_boot.bin".to_string());
    //Create the window parameters
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Linkfy GB Emulator", WIDTH, HEIGHT, WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X4,
        scale_mode: ScaleMode::AspectRatioStretch,
    })
    .unwrap_or_else( |e| {
        panic!("{}", e);
    });


    /*
          _           _               _
__      _(_)_ __   __| | _____      _| | ___   ___  _ __
\ \ /\ / | | '_ \ / _` |/ _ \ \ /\ / | |/ _ \ / _ \| '_ \
 \ V  V /| | | | | (_| | (_) \ V  V /| | (_) | (_) | |_) |
  \_/\_/ |_|_| |_|\__,_|\___/ \_/\_/ |_|\___/ \___/| .__/
                                                   |_|
    */

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        //We need to calculate the cycles to execute each frame from 0 to cycles_per_frame
        let mut cycles = 0;
        let cycles_per_frame = CLOCK_SPEED / FRAME_RATE;
        
        // Get the actual time and the time that a single frame needs
        let start_time = time::Instant::now();
        let frame_time = time::Duration::new(0, 16600000); // 1/60 secs = 60 FPS
        
        //Do the corresponding cycles per frame
        while cycles <= cycles_per_frame {
            let mut read_enter = String::new();
            let stdin = io::stdin().read_line(&mut read_enter);
            println!("Cycle {}/{}", cycles, cycles_per_frame);

            
            cycles += cpu.execute_instruction(&mut rom);
            cpu.registers.print();
                
        }

        let elapsed_time = start_time.elapsed();
        if elapsed_time < frame_time {
            println!("Update");
            //Update the screen each Frame
            let remaining_time = frame_time - elapsed_time;
            thread::sleep(remaining_time);
        }
        //Example to Window image on screen
        setBackground(&mut buffer, &mut window);
        
    }

/*
 _           _
| |_ ___ ___| |_ ___
| __/ _ / __| __/ __|
| ||  __\__ | |_\__ \
 \__\___|___/\__|___/ parameters
*/
    cpu.registers.print();
    cpu.registers.set("a", 0xFF);
    cpu.registers.print();
    cpu.registers.set("ab", 0xF00F);
    cpu.registers.print();
    cpu.registers.print();
    
    
}

/*
 _           _
| |_ ___ ___| |_ ___
| __/ _ / __| __/ __|
| ||  __\__ | |_\__ \
 \__\___|___/\__|___/ functions
*/
fn setBackground(buffer: &mut Vec<u32>, window: &mut Window) {
    //Example buffer data each loop
    for i in buffer.iter_mut() {
        *i = BACKGROUND;
    }

    //WE NEED TO PROGRAM SOMETHING TO UPDATE ONLY 60HZ. 60 Times per Second
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    
}