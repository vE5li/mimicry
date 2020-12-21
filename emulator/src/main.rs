extern crate sfml;

mod core;

use self::core::Core;
use sfml::graphics::{ RenderTarget, RenderWindow, Color, Font, View, FloatRect };
use sfml::window::{ Event, Style, Key };
use sfml::system::Vector2f;

fn main() {

    let font = Font::from_file("/home/mimicry/emulator/assets/monaco.ttf").unwrap();
    let mut window = RenderWindow::new((1800, 900), "emulator", Style::RESIZE, &Default::default());
    window.set_vertical_sync_enabled(true);

    let mut rising = true;
    let mut cores = Vec::new();
    cores.push(Core::load(String::from("test.logic"), &font, Vector2f::new(1800.0, 900.0), true));

    loop {
        while let Some(event) = window.poll_event() {
            match event {

                Event::Closed => return,

                Event::Resized { width, height } => {
                    let view = View::from_rect(&FloatRect::new(0.0, 0.0, width as f32, height as f32));
                    window.set_view(&view);
                    cores[0].update_size(Vector2f::new(width as f32, height as f32));
                },

                Event::KeyPressed { code, shift: _, ctrl: _, alt: _, system: _ } => {
                    if code == Key::Space {
                        cores.iter_mut().for_each(|core| core.tick(rising));
                        rising = !rising;
                    } else {
                        cores[0].handle_key_input(code);
                    }
                }

                _other => { },
            }
        }

        window.clear(Color::rgb(25, 25, 25));
        cores[0].draw(&mut window);
        window.display();
    }
}
