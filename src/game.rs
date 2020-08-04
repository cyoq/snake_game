use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs};

use crate::apple::Apple;
use crate::direction::Direction;
use crate::game_state::GameState;
use crate::score::Score;
use crate::snake::Snake;

pub struct Game<'a> {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub apple: Apple,
    pub score: Score<'a>,
    pub game_state: GameState,
}

impl<'a> Game<'a> {
    pub fn render(&mut self, args: &RenderArgs) {
        let black: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(black, gl);
        });

        self.snake.render(&mut self.gl, args);

        self.apple.render(&mut self.gl, args);
    }

    pub fn update(&mut self) {
        self.game_state = self.snake.update();
        self.score.update();

        let head_pos = *self.snake.body.front().expect("Body should have entries");

        if head_pos.0 == self.apple.x as i32 && head_pos.1 == self.apple.y as i32 {
            self.apple.update(&self.snake);
            self.score.increment();
            println!("Score is: {}", self.score.current);

            let tail = *(self.snake.body.back().expect("Body should have entries"));
            self.snake.body.push_back((tail.0, tail.1));
        }

        if self.game_state == GameState::Over {
            self.score.zero();
            println!("Your best score is: {}", self.score.best);
        }
    }

    pub fn pressed(&mut self, btns: &Button) {
        let last_dir = self.snake.direction;

        self.snake.direction = match btns {
            &Button::Keyboard(Key::Up) | &Button::Keyboard(Key::W) if last_dir != Direction::South => Direction::North,
            &Button::Keyboard(Key::Down) | &Button::Keyboard(Key::S) if last_dir != Direction::North => Direction::South,
            &Button::Keyboard(Key::Left) | &Button::Keyboard(Key::A) if last_dir != Direction::East => Direction::West,
            &Button::Keyboard(Key::Right) | &Button::Keyboard(Key::D) if last_dir != Direction::West => Direction::East,
            _ => self.snake.direction
        }
    }
}
