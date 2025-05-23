use macroquad::window::Conf;
use std::error::Error;
use typing_test::app::App;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    let (data, config) = typing_test::parse_args(std::env::args())?;
    let mut app = App::new(data, config).await;
    app.main_loop().await?;

    Ok(())
}

fn window_conf() -> Conf {
    let default = Conf::default();

    let height = 700.0;

    Conf {
        window_title: "Typing Test".to_string(),
        fullscreen: false,
        window_width: (height * 1.61) as i32,
        window_height: height as i32,
        window_resizable: true,
        high_dpi: true,
        ..default
    }
}
