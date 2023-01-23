use macroquad::input::{is_key_pressed, KeyCode};

use macroquad::color::Color;
use macroquad::text::draw_text;

pub fn generate_identifier(prefix: &str) -> String {
    [
        prefix.to_owned(),
        macroquad::rand::rand().to_string(),
        macroquad::rand::rand().to_string(),
        macroquad::rand::rand().to_string(),
        macroquad::rand::rand().to_string(),
    ]
    .join("-")
}

pub fn draw_text_with_shadow(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    shadow_offset: f32,
    shadow_color: Color,
) {
    // Shadow
    draw_text(
        text,
        x + shadow_offset,
        y + shadow_offset,
        font_size,
        shadow_color,
    );

    // Text
    draw_text(text, x, y, font_size, color);
}

pub fn is_any_letter_pressed() -> Option<char> {
    if is_key_pressed(KeyCode::A) {
        return Some('a');
    } else if is_key_pressed(KeyCode::B) {
        return Some('b');
    } else if is_key_pressed(KeyCode::C) {
        return Some('c');
    } else if is_key_pressed(KeyCode::D) {
        return Some('d');
    } else if is_key_pressed(KeyCode::E) {
        return Some('e');
    } else if is_key_pressed(KeyCode::F) {
        return Some('f');
    } else if is_key_pressed(KeyCode::G) {
        return Some('g');
    } else if is_key_pressed(KeyCode::H) {
        return Some('h');
    } else if is_key_pressed(KeyCode::I) {
        return Some('i');
    } else if is_key_pressed(KeyCode::J) {
        return Some('j');
    } else if is_key_pressed(KeyCode::K) {
        return Some('k');
    } else if is_key_pressed(KeyCode::L) {
        return Some('l');
    } else if is_key_pressed(KeyCode::M) {
        return Some('m');
    } else if is_key_pressed(KeyCode::N) {
        return Some('n');
    } else if is_key_pressed(KeyCode::O) {
        return Some('o');
    } else if is_key_pressed(KeyCode::P) {
        return Some('p');
    } else if is_key_pressed(KeyCode::Q) {
        return Some('q');
    } else if is_key_pressed(KeyCode::R) {
        return Some('r');
    } else if is_key_pressed(KeyCode::S) {
        return Some('s');
    } else if is_key_pressed(KeyCode::T) {
        return Some('t');
    } else if is_key_pressed(KeyCode::U) {
        return Some('u');
    } else if is_key_pressed(KeyCode::V) {
        return Some('v');
    } else if is_key_pressed(KeyCode::W) {
        return Some('w');
    } else if is_key_pressed(KeyCode::X) {
        return Some('x');
    } else if is_key_pressed(KeyCode::Y) {
        return Some('y');
    } else if is_key_pressed(KeyCode::Z) {
        return Some('z');
    } else if is_key_pressed(KeyCode::Space) {
        return Some(' ');
    }

    None
}
