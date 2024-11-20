use macroquad::window::Conf;
use std::error::Error;
use typing_test::screen::Screen;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = typing_test::parse_args(std::env::args())?;
    let mut screen = Screen::new(data);
    screen.main_loop().await?;

    Ok(())
}

fn window_conf() -> Conf {
    let default = Conf::default();

    Conf {
        window_title: "Typing Test".to_string(),
        fullscreen: false,
        window_width: (600.0 * 1.61) as i32,
        window_height: 600,
        window_resizable: false,
        high_dpi: true,
        ..default
    }
}
