use macroquad::prelude::*;

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

static BACKGROUND_COLOR: Color = Color::new(0.3, 0.0, 0.0, 1.0);

#[macroquad::main(get_config)]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BACKGROUND_COLOR);

        if is_key_down(KeyCode::Right) {
            x += 10.0;
        }

        if is_key_down(KeyCode::Left) {
            x -= 10.0;
        }

        if is_key_down(KeyCode::Up) {
            y -= 10.0;
        }

        if is_key_down(KeyCode::Down) {
            y += 10.0;
        }

        draw_circle(x, y, 16.0, YELLOW);

        next_frame().await
    }
}

fn get_config() -> Conf {
    Conf {
        fullscreen: false,
        high_dpi: true,
        icon: None,
        platform: Default::default(),
        window_title: PACKAGE_NAME.into(),
        window_width: 1024,
        window_height: 680,
        sample_count: Default::default(),
        window_resizable: false
    }
}