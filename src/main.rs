pub mod objects {
    use sfml::graphics::{CircleShape, RenderWindow, RenderTarget, Transformable, Shape, Color, RectangleShape};
    use sfml::system::Vector2f;

    use crate::object_handler::{TDraw};

    pub struct Circle {
        size: f32,
        position: Vector2f,
    }

    impl TDraw for Circle {
        fn draw(&self, window: &mut RenderWindow) {
            let mut tmp_circle = CircleShape::new(self.size, 30);

            tmp_circle.set_position(self.position);
            tmp_circle.set_fill_color(Color::WHITE);

            window.draw(&tmp_circle);
        }
    }

    impl Circle {
        pub fn new() -> Self {
            Self {
                size: 32.0,
                position: Vector2f::new(32.0, 32.0),
            }
        }
    }

    pub struct Square {
        size: f32,
        position: Vector2f,
    }

    impl TDraw for Square {
        fn draw(&self, window: &mut RenderWindow) {
            let mut tmp_square = RectangleShape::new();

            tmp_square.set_size(Vector2f::new(self.size, self.size));
            tmp_square.set_position(self.position);
            tmp_square.set_fill_color(Color::WHITE);

            window.draw(&tmp_square);
        }
    }

    impl Square {
        pub fn new() -> Self {
            Self {
                size: 32.0,
                position: Vector2f::new(512.0, 32.0),
            }
        }
    }
}

use sfml::{
    graphics::{Color, RenderWindow, RenderTarget},
    window::{Style, Event, Key},
};

use crate::object_handler::ObjectHandler;
use crate::objects::Circle;
use crate::objects::Square;

fn main() {
    let mut window = RenderWindow::new(
        (1920, 1080),
        "SFML window",
        Style::CLOSE,
        &Default::default()
    );

    let mut obj = ObjectHandler::new();
    obj.import_object(Box::new(Square::new()));
    obj.import_object(Box::new(Square::new()));
    obj.import_object(Box::new(Circle::new()));
    obj.import_object(Box::new(Circle::new()));
    //obj.import_object(RectangleShape::new_init(Vector2f::new(32.0, 32.0)));

    while window.is_open() {

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        obj.draw_all(&mut window);
        window.display();
    }
}

pub mod object_handler {
    use sfml::{
        graphics::{RenderWindow},
        window::{Event},
    };

    use crate::objects::*;

    pub trait TDraw {
        fn draw(&self, window: &mut RenderWindow);

        //fn import(&self, data: Vec<String>);
        //fn export(&self) -> Vec<String>;
    }

    pub trait TEvent {
        fn event(&self, event: &Event);
    }

    pub struct ObjectHandler {
        object_store: Vec<Box<dyn TDraw>>,
    }

    impl ObjectHandler {
        pub fn new() -> Self {
            Self{
                object_store: Vec::new(),
            }
        }

        pub fn draw_all(&self, window: &mut RenderWindow) {
            for object in self.object_store.iter() {
                object.draw(window);
            }
        }

        /*pub fn export_all(&mut self) -> Vec<Vec<String>> {
            let mut tmp_vec: Vec<Vec<String>> = Vec::new();

            for object in self.object_store.iter() {
                tmp_vec.push(object.export());
            }

            tmp_vec
        }*/

        pub fn import_object(&mut self, data: Box<dyn TDraw>) -> u32  {
            self.object_store.push(data);
            self.object_store.len() as u32
        }

        pub fn get_object_reference(&mut self, index: u32) -> &mut Box<dyn TDraw> {
            &mut self.object_store[index as usize]
        }
    }
}
