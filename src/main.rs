use macroquad::prelude::*;

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

static BACKGROUND_COLOR: Color = Color::new(0.3, 0.0, 0.0, 1.0);

const MOVEMENT_SPEED: f32 = 200.0;
const CIRCLE_RADIUS: f32 = 16.0;

#[macroquad::main(get_config)]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        let delta_time = get_frame_time();

        clear_background(BACKGROUND_COLOR);

        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }

        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }

        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }

        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }

        x = clamp(x, CIRCLE_RADIUS, screen_width() - CIRCLE_RADIUS);
        y = clamp(y, CIRCLE_RADIUS, screen_height() - CIRCLE_RADIUS);

        draw_circle(x, y, CIRCLE_RADIUS, YELLOW);

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