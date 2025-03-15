use macroquad::prelude::*;

// static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

static BACKGROUND_COLOR: Color = Color::new(0.3, 0.0, 0.0, 1.0);

#[macroquad::main("My game")]
async fn main() {
    loop {
        clear_background(BACKGROUND_COLOR);
        next_frame().await
    }
}