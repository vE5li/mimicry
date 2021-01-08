mod core;

use self::core::*;
use crate::types::*;

pub struct Device {
    cores: Vec<Core>,
    focused_core: Option<usize>,
    rising: bool,
    show_top_bar: bool,
}

impl Device {

    pub fn new(interface_size: FloatVector) -> Self {

        // TEMP
        let mut cores = Vec::new();
        cores.push(Core::load(String::from("test.logic"), interface_size, true));

        return Self {
            cores: cores,
            focused_core: Some(0), // TODO: None
            rising: true,
            show_top_bar: true,
        };
    }

    pub fn handle_key_input(&mut self, key: Key) {
        match key {

            Key::Space => {
                let rising = self.rising;
                self.cores.iter_mut().for_each(|core| core.tick(rising));
                self.rising = !self.rising;
            },

            other => {
                self.cores[0].handle_key_input(other);
            }
        }
    }

    pub fn resize(&mut self, size: FloatVector) {
        self.cores.iter_mut().for_each(|core| core.resize(size));
    }

    pub fn render<T: Renderer>(&self, renderer: &mut T) {
        renderer.clear(Color::monochrome(25));
        let position = FloatVector::new();

        if self.show_top_bar {
            // draw top bar
        }

        if let Some(index) = self.focused_core {
            self.cores[index].draw(renderer, position);
        }

        renderer.display();
    }
}
