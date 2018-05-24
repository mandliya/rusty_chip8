use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct KeyPad {
    pub keys: [bool; 16],
    events: sdl2::EventPump,
}

impl KeyPad {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        KeyPad {
            keys: [false; 16],
            events: sdl_context.event_pump()
                .expect("Fatal Error: Failed to get SDL2 event pump"),
        }
    }

    pub fn poll(&mut self) -> Result<(), ()> {
        for event in self.events.poll_iter() {
             match event {
                Event::Quit{ .. }|
                    Event::KeyDown{ keycode: Some(Keycode::Escape), .. } => {
                    return Err(());
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => self.keys[0xF] ^= true,

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => self.keys[4] = true,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => self.keys[6] = true,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => self.keys[8] = true,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => self.keys[2] = true,
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => self.keys[5] = true,

                Event::KeyUp { keycode: Some(Keycode::Left), .. } => self.keys[4] = false,
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => self.keys[6] = false,
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => self.keys[8] = false,
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => self.keys[2] = false,
                Event::KeyUp { keycode: Some(Keycode::Return), .. } => self.keys[5] = false,
                 _ => {}
             }
        }
        Ok(())
    }
}