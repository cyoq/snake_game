pub mod score;
pub mod game_state;
pub mod consts;
pub mod apple;
pub mod game;
pub mod direction;
pub mod snake;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderEvent, UpdateEvent, ButtonEvent, ButtonState};
use piston::window::WindowSettings;
use std::collections::linked_list::LinkedList;
use std::iter::FromIterator;
use crate::snake::Snake;
use crate::direction::Direction;
use crate::game::Game;
use crate::consts::*;
use crate::apple::Apple;
use crate::game_state::GameState;
use crate::score::Score;

fn create_game<'a>(opengl: OpenGL) -> Game<'a> {
    Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter(vec![(3, 7), (3, 8), (3, 9)].into_iter()),
            grid_size: SQUARE_SIZE,
            direction: Direction::North
        },
        apple: Apple {
            x: (SCREEN_WIDTH / SQUARE_SIZE) / 2 as u32,
            y: (SCREEN_WIDTH / SQUARE_SIZE) / 2 as u32,
            grid_size: SQUARE_SIZE,
        },
        score: Score::new(),
        game_state: GameState::Run
    }
}

fn main() {

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Snake game", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut game: Game = create_game(opengl);

    let mut events = Events::new(EventSettings::new().ups(8));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_) = e.update_args() {
            if game.game_state == GameState::Over {
                game = create_game(opengl);
            }
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
