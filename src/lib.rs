mod utils;

use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;


#[wasm_bindgen]
pub struct Gesturizer {
    pressed: bool,
    pos: Position,
    prev: Position,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Position {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Position {
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
}

#[wasm_bindgen]
impl Gesturizer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Gesturizer {
        Gesturizer { pressed: false, pos: Position {x: 0.0, y: 0.0}, prev: Position {x: 0.0, y: 0.0} }
    }

    pub fn get_mouse(&self) -> Position {
        self.pos
    }

    pub fn mouse_move(&mut self, x: f32, y: f32) {
        if self.pressed {
            let dx = x - self.prev.x;
            let dy = y - self.prev.y;
            self.pos.x += dx;
            self.pos.y += dy;
        }
        self.prev.x = x;
        self.prev.y = y;
    }

    pub fn mouse_down(&mut self) {
        self.pressed = true
    }

    pub fn mouse_up(&mut self) {
        self.pressed = false
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(n: &str) {
    alert(&format!("Hello, {n}!"));
}

#[wasm_bindgen]
pub fn setup() {
    set_panic_hook();
}