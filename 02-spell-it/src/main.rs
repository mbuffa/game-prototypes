use game_master::GameMaster;
use macroquad::prelude::*;

mod utils;
mod world;
mod combat;
mod game_state;
mod game_master;
mod rendering;
mod systems;

use game_state::GameState;
use world::World;

const TRANSITION_TIMER: f32 = 6f32;

enum State {
    Intro,
    Playing,
    GameOver
}

struct GameContext {
    master: GameMaster,
    state: State,
    in_transition: bool,
    transition_time: f32
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            master: GameMaster::new(),
            state: State::Playing,
            in_transition: false,
            transition_time: TRANSITION_TIMER
        }
    }

    pub fn init(&mut self) {
        self.master.start_session();
    }

    pub fn update(&mut self, since_last_frame: &f32) {
        if self.in_transition {
            if self.transition_time <= 0f32 {
                self.reset();
            } else {
                self.transition_time -= since_last_frame;
            }
        }

        let(world, game_state, input) = self.master.update(since_last_frame);

        draw(&self.transition_time, &world, &game_state, &input);

        if game_state.is_over() || game_state.is_won() {
            self.in_transition = true;
        }
    }

    pub fn reset(&mut self) {
        self.master = GameMaster::new();
        self.master.start_session();
        self.in_transition = false;
        self.transition_time = TRANSITION_TIMER;
    }
}

#[macroquad::main("Spell It")]
async fn main() {
    let mut since_last_frame: f32;

    let mut context = GameContext::new();
    context.init();

    loop {
        since_last_frame = macroquad::time::get_frame_time();
        context.update(&since_last_frame);
        next_frame().await
    }
}

fn draw(context: &f32, world: &World, game_state: &GameState, input: &String) {    
    if game_state.is_won() {
        rendering::draw_victory_screen(context);
    } else if game_state.is_over() {
        rendering::draw_defeat_screen(context);
    } else {
        rendering::draw_game_screen(&world, &game_state, &input);
    }

    // rendering::draw_ui_debug(&game_state);
}
