use macroquad::window::Conf;
use std::error::Error;
use typing_test::app::App;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    let (data, config) = typing_test::parse_args(std::env::args())?;
    let app = App::new(data, config);
    app.run().await;

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
