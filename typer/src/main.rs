use macroquad::window::Conf;
use screen::Screen;
use std::error::Error;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = typer::parse_args(std::env::args())?;
    let screen = Screen::new(data, None, None, None);
    screen.main_loop().await?;

    Ok(())
}

fn window_conf() -> Conf {
    let default = Conf::default();

    Conf {
        window_title: "Typing Test".to_string(),
        fullscreen: false,
        window_width: 1000,
        window_height: 600,
        high_dpi: true,
        ..default
    }
}
