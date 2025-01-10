pub mod data_provider;
pub mod screen;
use self::app::ThemeName;
use data_provider::Data;

use serde::{Deserialize, Serialize};

use std::env::{self, Args};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn parse_args(mut args: Args) -> Result<(Data, Config), Box<dyn Error>> {
    args.next();

    let mut words_file = None;
    let mut quotes_file = None;

    let mut config_fname: PathBuf = [env::var("HOME").unwrap().as_str(), ".typing_test.toml"]
        .iter()
        .collect();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--words" | "-w" => {
                if let Some(f) = args.next() {
                    words_file = Some(f);
                };
            }
            "--quotes" | "-q" => {
                if let Some(f) = args.next() {
                    quotes_file = Some(f);
                }
            }
            //"--online" => return Data::new_online(words_file),
            "--help" | "-h" => help(),
            "--config" | "-c" => {
                if let Some(f) = args.next() {
                    config_fname = PathBuf::from(f);
                }
            }
            _ => (),
        }
    }

    let data = Data::new_offline(words_file, quotes_file)?;

    Ok((data, Config::new(config_fname.as_path())))
}

fn help() {
    let help = [
        "Usage: typing_test [-w <file>] [-q <file>] [-c <file>]\n",
        "Test your typing speed from built-in words and quotes, or provide your own words and quotes. You can also choose to get quotes from scrapping popular quotes on the Internet (not implemented yet).\n",
        "Controls:",
        "   <Tab>                   Cycle forward between buttons.",
        "   <Shift-Tab>             Cycle backward between buttons.",
        "   <Enter>                 Click selected button. Alternatively, you can use your mouse to click on buttons.",
        "   <Super>=                Increase font size.",
        "   <Super>-                Decrease font size.",
        "   <Super>0                Reset font size.\n",
        "Options:",
        "   -w, --words <file>      Provide your own words file.",
        "                           Words are separated by a new line character.",
        "   -q, --quotes <file>     Provide your own quotes file.",
        "                           Sources are separated by 2 new line characters.",
        "                           The first line is the source name, and each quote belonging",
        "                           to that source is separated by a new line character.",
        "                           See built-in quotes at the repository.",
        //"   -o, --online            Get quotes from the web. Must be connected to the internet.",
        "   -c, --config <file>     Provide custom config file. Default is ~/.typing_test.toml",
        "   -h, --help              Print this help.",
    ]
    .join("\n");

    println!("{}", help);
    std::process::exit(1);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThemeParams {
    bg_color: u32,
    text_color: u32,
    error_color: u32,
    ghost_color: u32,
}

// TODO: extra themes need to be able to be selected
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub config_file: PathBuf,

    pub theme: ThemeName,
    pub font_size: f32,
    // extra_themes: HashMap<String, ThemeParams>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_file: [env::var("HOME").unwrap().as_str(), ".typing_test.toml"]
                .iter()
                .collect(),
            theme: ThemeName::default(),
            font_size: 30.0,
        }
    }
}

impl Config {
    pub fn new(filename: &Path) -> Self {
        match fs::read_to_string(filename) {
            Ok(s) => {
                let mut config = match toml::from_str::<Config>(&s) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("Config Error, using defaults. {}", e);
                        Config::default()
                    }
                };
                config.config_file = filename.to_path_buf();
                config
            }
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        let config_fname: PathBuf =
                            [env::var("HOME").unwrap().as_str(), ".typing_test.toml"]
                                .iter()
                                .collect();

                        if let Err(e) =
                            fs::write(config_fname, toml::to_string(&Config::default()).unwrap())
                        {
                            println!("Can't create default config file. {}", e);
                        };
                    }
                    _ => {
                        println!("Can't read config file, using defaults. {}", e.kind());
                    }
                }
                Config::default()
            }
        }
    }

    pub fn update_file(&self) {
        if let Err(e) = fs::write(&self.config_file, toml::to_string(&self).unwrap()) {
            println!("Can't write to config file. {}", e);
        };
    }
}
