use std::env;
use std::fs::File;
use std::io::Read;

mod chip8;
mod display;
mod input;
mod audio;

use audio::AudioHandler;
use display::Display;
use chip8::Chip8;
use input::InputHandler;
//use sdl2::keyboard::Keycode;
//use sdl2::event::Event;

const TICKS_PER_FRAME: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() !=2 {
        print!("Usage chitta path/to/rom");
    }
    let mut rom = File::open(&args[1]).expect("Unable to open open rom");
    let mut buffer: Vec<u8> = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_program(&buffer);

    
    //SDL
    let context = sdl2::init().unwrap();
    let mut display = Display::new(&context);
    let mut inputhandler = InputHandler::new(&context);
    let audiohandler = AudioHandler::new(&context);

    // Main Loop
    loop {

        if inputhandler.process_inputs_terminate(chip8.get_keys()) {
            break;
        }

        for _ in 0..TICKS_PER_FRAME {
            chip8.do_iteration();
        };

        // Beep if timeres are proper
        if chip8.timer_advance() {
            audiohandler.start_beep();
        } else {
            audiohandler.stop_beep();
        }
        display.draw(chip8.get_display());
    }
}
