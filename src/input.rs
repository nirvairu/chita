use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
/*
for event in event_pump.poll_iter() {
    match event {
        Event::Quit{..} => {
            break 'sys_loop
        },

        Event::KeyDown{keycode: Some(key), ..} => {
            if let Some(k) = key2btn(key) {
                chip8.update_key(k, true);
            }
        }
        Event::KeyUp{keycode: Some(key), ..} => {
            if let Some(k) = key2btn(key) {
                chip8.update_key(k, false);
            }
        },
        _ => ()
    }
}
*/
pub struct InputHandler {
    event: sdl2::EventPump,
}

impl InputHandler {

    pub fn new(context: &sdl2::Sdl) -> Self {
        Self { event: context.event_pump().unwrap() }
    }

    pub fn process_inputs_terminate(&mut self, keypad: &mut [bool; 16]) -> bool {
        for event in self.event.poll_iter() {
            match event {
                Event::Quit{..} => {
                    return true;
                },
        
                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(k) = to_keypad(key) {
                        keypad[k] = true;
                    }
                }
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = to_keypad(key) {
                        keypad[k] = false;
                    }
                },
                _ => ()
            }
        }
        return false;
    }
}

fn to_keypad (key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 =>    Some(0x1),
        Keycode::Num2 =>    Some(0x2),
        Keycode::Num3 =>    Some(0x3),
        Keycode::Num4 =>    Some(0xC),
        Keycode::Q =>       Some(0x4),
        Keycode::W =>       Some(0x5),
        Keycode::E =>       Some(0x6),
        Keycode::R =>       Some(0xD),
        Keycode::A =>       Some(0x7),
        Keycode::S =>       Some(0x8),
        Keycode::D =>       Some(0x9),
        Keycode::F =>       Some(0xE),
        Keycode::Z =>       Some(0xA),
        Keycode::X =>       Some(0x0),
        Keycode::C =>       Some(0xB),
        Keycode::V =>       Some(0xF),
        _ =>                None,
    }
}
