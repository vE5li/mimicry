use common::*;
use sfml::graphics::{ RenderTarget, RenderWindow, Font, View, FloatRect, RectangleShape, Text, Shape, Transformable, VertexArray, PrimitiveType };
use sfml::window::{ Event, Style, Key };
use sfml::system::Vector2f;
use sfml::SfBox;

fn to_sfml_vector(vector: FloatVector) -> Vector2f {
    return Vector2f::new(vector.x, vector.y);
}

fn to_sfml_color(color: Color) -> sfml::graphics::Color {
    return sfml::graphics::Color::rgb(color.red, color.green, color.blue);
}

pub struct SFMLRenderer {
    window: RenderWindow,
    font: SfBox<Font>,
}

impl SFMLRenderer {

    pub fn new(window_title: &str, window_size: FloatVector, vertical_sync: bool, font_file: &str) -> Self {

        let mut window = RenderWindow::new((window_size.x as u32, window_size.y as u32), window_title, Style::RESIZE, &Default::default());
        window.set_vertical_sync_enabled(vertical_sync);
        let font = Font::from_file(font_file).unwrap();

        return Self {
            window: window,
            font: font,
        };
    }

    pub fn event(&mut self) -> Option<Event> {
        return self.window.poll_event();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let view = View::from_rect(&FloatRect::new(0.0, 0.0, width, height));
        self.window.set_view(&view);
    }
}

impl Renderer for SFMLRenderer {

    fn clear(&mut self, color: Color) {
        self.window.clear(to_sfml_color(color));
    }

    fn display(&mut self) {
        self.window.display();
    }

    fn draw_rectangle(&mut self, position: FloatVector, size: FloatVector, color: Color) {
        let mut rectangle = RectangleShape::new();
        rectangle.set_size(to_sfml_vector(size));
        rectangle.set_position(to_sfml_vector(position));
        rectangle.set_fill_color(to_sfml_color(color));
        self.window.draw(&rectangle);
    }

    fn draw_text(&mut self, text: &str, position: FloatVector, color: Color, size: u32) {
        let mut text = Text::new(text, &self.font, size);
        text.set_position(to_sfml_vector(position));
        text.set_fill_color(to_sfml_color(color));
        self.window.draw(&text);
    }

    fn draw_text_right(&mut self, text: &str, position: FloatVector, color: Color, size: u32) {
        let mut text = Text::new(text, &self.font, size);
        let text_width = text.local_bounds().width;
        let text_position = position - FloatVector::with_x(text_width);
        text.set_position(to_sfml_vector(text_position));
        text.set_fill_color(to_sfml_color(color));
        self.window.draw(&text);
    }

    fn draw_line_segment(&mut self, position: FloatVector, vertices: &Vec<Vertex>) {
        let mut line_segment = VertexArray::default();
        line_segment.set_primitive_type(PrimitiveType::LINE_STRIP);

        for vertex in vertices {
            let vertex_color = to_sfml_color(vertex.color);
            let vertex_position = to_sfml_vector(position + vertex.position);
            line_segment.append(&sfml::graphics::Vertex::with_pos_color(vertex_position, vertex_color));
        }

        self.window.draw(&line_segment);
    }
}
