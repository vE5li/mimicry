extern crate common;
extern crate sfml;

mod renderer;

use common::*;
use self::renderer::SFMLRenderer;
use sfml::window::Event;

fn from_sfml_key(key: sfml::window::Key) -> Option<Key> {
    match key {
        sfml::window::Key::SPACE => return Some(Key::Space),
        sfml::window::Key::T => return Some(Key::T),
        _other => return None,
    }
}

fn main() {

    // get from command line
    let window_title = "mimicry emulator";
    let window_size = FloatVector::from(1800.0, 900.0);
    let vertical_synch = true;

    let mut device = Device::new(window_size);
    let mut renderer = SFMLRenderer::new(window_title, window_size, vertical_synch, "/usr/share/tortoise/mimicry/emulator/assets/monaco.ttf");

    loop {
        while let Some(event) = renderer.event() {
            match event {

                Event::Closed => return,

                Event::Resized { width, height } => {
                    device.resize(FloatVector::from(width as f32, height as f32));
                    renderer.resize(width as f32, height as f32);
                },

                Event::KeyPressed { code, shift: _, ctrl: _, alt: _, system: _ } => {
                    if let Some(key) = from_sfml_key(code) {
                        device.handle_key_input(key);
                    }
                }

                _other => { },
            }
        }

        device.render(&mut renderer);
    }
}
