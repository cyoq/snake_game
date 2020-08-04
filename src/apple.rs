use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::Rng;

use crate::consts::*;
use crate::snake::Snake;

pub struct Apple {
    pub x: u32,
    pub y: u32,
    pub grid_size: u32,
}

impl Apple {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.x * self.grid_size) as f64,
            (self.y * self.grid_size) as f64,
            self.grid_size as f64,
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(red, square, transform, gl);
        });
    }

    pub fn update(&mut self, snake: &Snake) {
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen::<u32>() % GRID;
            let y = rng.gen::<u32>() % GRID;
            if !snake.does_collide(&(x as i32, y as i32)) {
                self.x = x;
                self.y = y;
                break;
            }
        }
    }
}
