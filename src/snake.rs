use std::collections::LinkedList;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::consts::GRID;
use crate::direction::Direction;
use crate::game_state::GameState;

type SnakePart = (i32, i32);

pub struct Snake {
    pub body: LinkedList<SnakePart>,
    pub grid_size: u32,
    pub direction: Direction,
}

impl Snake {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let white: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * self.grid_size as i32) as f64,
                    (y * self.grid_size as i32) as f64,
                    self.grid_size as f64,
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|s| {
                    graphics::rectangle(white, s, transform, gl);
                });
        });
    }

    pub fn update(&mut self) -> GameState {
        let mut new_head = (self.body.front().expect("Body should be full")).clone();

        match self.direction {
            Direction::North => new_head.1 -= 1,
            Direction::South => new_head.1 += 1,
            Direction::West => new_head.0 -= 1,
            Direction::East => new_head.0 += 1,
        }

        self.check_going_out(&mut new_head);

        self.body.push_front(new_head);

        self.body.pop_back();

        self.check_self_touching()
    }

    pub fn does_collide(&self, object: &(i32, i32)) -> bool {
        self.body.iter().any(|part| part.0 == object.0 && part.1 == object.1)
    }

    fn check_self_touching(&self) -> GameState {
        let head = self.body.front().expect("Body should have entries");

        let mut cloned = self.body.clone();
        cloned.pop_front();

        for part in cloned.iter() {
            if part.0 == head.0 && part.1 == head.1 {
                return GameState::Over;
            }
        }

        GameState::Run
    }

    fn check_going_out(&mut self, head: &mut SnakePart) {
        if head.0 >= GRID as i32 || head.1 >= GRID as i32 {
            head.0 %= GRID as i32;
            head.1 %= GRID as i32;
        } else if head.0 <= -1 {
            head.0 = GRID as i32;
        } else if head.1 <= -1 {
            head.1 = GRID as i32;
        }
    }
}
