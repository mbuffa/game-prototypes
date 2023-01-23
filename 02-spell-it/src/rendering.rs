use macroquad::prelude::*;

use crate::utils;

const COLOR_SPELLBOOK: Color = YELLOW;
const SPELL_FONT_SIZE: f32 = 24.0;
const SPELL_X_PADDING: f32 = 10f32;
const COLOR_SPELL_DAMAGE: Color = RED;
const COLOR_SPELL_HEALING: Color = BLUE;
const COLOR_TEXT_SHADOW: Color = DARKGRAY;

const COLOR_PLAYER_HEALTH_BACKGROUND: Color = BLACK;
const COLOR_PLAYER_HEALTH_FOREGROUND: Color = RED;

const DESCRIPTION_FONT_SIZE: f32 = 24f32;
const COLOR_DESCRIPTION_FOREGROUND: Color = WHITE;

const INPUT_FONT_SIZE: f32 = 24f32;
const COLOR_INPUT_BACKGROUND: Color = BLACK;
const COLOR_INPUT_FOREGROUND: Color = WHITE;

const MONSTER_WIDTH: f32 = 40f32;
const MONSTER_HEIGHT: f32 = 100f32;

use crate::game_state::GameState;
use crate::world::World;

// fn center_x() -> f32 { screen_width() /2f32 }
// fn center_y() -> f32 { screen_height() /2f32 }

fn grid_12_width() -> f32 {
    screen_width() / 12f32
}
fn grid_12_height() -> f32 {
    screen_height() / 12f32
}
fn grid_24_width() -> f32 {
    screen_width() / 24f32
}
fn grid_24_height() -> f32 {
    screen_height() / 24f32
}
fn grid_48_width() -> f32 {
    screen_width() / 48f32
}
fn grid_48_height() -> f32 {
    screen_height() / 48f32
}

pub fn draw_ui_debug(game_state: &GameState) {
    draw_rectangle(0f32, 0f32, 150f32, 200f32, BLACK);
    draw_text(
        &format!("{}", macroquad::time::get_fps()),
        0f32,
        50f32,
        64f32,
        WHITE,
    );
    draw_text(
        &format!("is_over: {:?}", game_state.is_over()),
        0f32,
        100f32,
        16f32,
        WHITE,
    );
    draw_text(
        &format!("is_won: {:?}", game_state.is_won()),
        0f32,
        120f32,
        16f32,
        WHITE,
    );
}

pub fn draw_ui(world: &World, game_state: &GameState, input: &String) {
    draw_spellbook(&world);
    draw_health(&game_state);
    draw_description(&game_state);
    draw_input(&input);
}

fn draw_spellbook(world: &World) {
    let pos = (grid_24_width(), grid_24_height());

    // Background
    draw_rectangle(
        pos.0,
        pos.1,
        grid_12_width() * 3f32,
        grid_12_height() * 3f32,
        COLOR_SPELLBOOK,
    );

    // Spells
    world
        .get_spell_types()
        .iter()
        .enumerate()
        .for_each(|(i, (k, v))| {
            let color = match v.get_type() {
                crate::world::spell::SpellEffectType::Damage => COLOR_SPELL_DAMAGE,
                crate::world::spell::SpellEffectType::Healing => COLOR_SPELL_HEALING,
            };

            utils::draw_text_with_shadow(
                k,
                pos.0 + SPELL_X_PADDING,
                (pos.1 + grid_12_height() / 2f32) + grid_12_height() / 2f32 * i as f32,
                SPELL_FONT_SIZE,
                color,
                0.5f32,
                COLOR_TEXT_SHADOW,
            );
        });
}

fn draw_health(game_state: &GameState) {
    let remaining_health = format!("{}", game_state.get_scene().get_player().get_health());

    draw_rectangle(
        grid_24_width(),
        grid_12_height() * 4f32,
        grid_24_width(),
        grid_12_height() * 3f32 - grid_24_height(),
        COLOR_PLAYER_HEALTH_BACKGROUND,
    );

    // crate::debug!("{}", remaining_health);

    draw_text(
        &remaining_health,
        grid_24_width(),
        grid_12_height() * 5f32,
        24.0,
        GREEN,
    );
}

fn draw_description(game_state: &GameState) {
    let description = match game_state.get_scene().get_current_stage() {
        Some(stage) => stage.get_description(),
        None => "",
    };

    // Background
    draw_rectangle(
        grid_12_width() * 4f32,
        grid_24_height() * 14f32 - grid_48_height(),
        grid_12_width() * 7f32,
        grid_12_height() * 2f32,
        BLACK,
    );

    // Text
    draw_text(
        description,
        grid_12_width() * 4f32 + grid_48_width(),
        grid_24_height() * 14f32 + grid_48_height(),
        DESCRIPTION_FONT_SIZE,
        COLOR_DESCRIPTION_FOREGROUND,
    );
}

fn draw_input(input: &String) {
    draw_rectangle(
        grid_12_width() * 4f32,
        grid_12_height() * 9f32,
        grid_12_width() * 7f32,
        grid_12_height() * 2f32,
        COLOR_INPUT_BACKGROUND,
    );

    draw_text(
        input,
        grid_12_width() * 4f32 + grid_48_width(),
        grid_12_height() * 9f32 + grid_24_height(),
        INPUT_FONT_SIZE,
        COLOR_INPUT_FOREGROUND,
    );
}

pub fn draw_scene(world: &World, game_state: &GameState, input: &String) {
    let scene_pos = (grid_12_width() * 4f32, grid_24_height() * 1f32);
    let scene_width = grid_12_width() * 7f32;
    let scene_height = grid_12_height() * 6f32;

    let scene_center = (
        scene_pos.0 + scene_width / 2f32,
        scene_pos.1 + scene_height / 2f32,
    );

    // Background
    draw_rectangle(scene_pos.0, scene_pos.1, scene_width, scene_height, BLACK);

    // Walls rendering

    // Enemies rendering
    let enemy_pos = (scene_pos.0, scene_center.1 - MONSTER_HEIGHT / 2f32);

    match game_state.get_scene().get_current_stage() {
        Some(stage) => {
            let enemies = stage.get_enemies();
            let party_size = enemies.len();
            let party_width = party_size as f32 * MONSTER_WIDTH;
            let party_width = (party_size as f32 * MONSTER_WIDTH)
                + ((party_size as f32 - 1f32) * grid_48_width());

            enemies.iter().enumerate().for_each(|(i, e)| {
                if e.is_alive() {
                    let x = scene_center.0 - (party_width / 2f32)
                        + ((i as f32) * (MONSTER_WIDTH + grid_48_width()));

                    draw_rectangle(
                        x,
                        enemy_pos.1,
                        MONSTER_WIDTH,
                        MONSTER_HEIGHT,
                        match e.get_enemy_type() {
                            crate::game_state::EnemyType::Goblin(_, _, _, _) => PINK,
                            crate::game_state::EnemyType::Orc(_, _, _, _) => GREEN,
                            crate::game_state::EnemyType::Succubus(_, _, _, _) => RED,
                        },
                    );

                    draw_text(
                        &format!("{}", e.get_health()),
                        x + grid_48_width() / 2f32,
                        enemy_pos.1 - grid_48_height() / 2f32,
                        24.0,
                        WHITE,
                    );
                }
            })
        }
        None => {}
    }
}
