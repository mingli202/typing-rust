pub mod data_provider;
use data_provider::Data;
pub mod screen;
use std::env::Args;
use std::error::Error;

pub fn parse_args(mut args: Args) -> Result<Data, Box<dyn Error>> {
    args.next();

    let mut words_file = String::from("src/data/words.txt");
    let mut quotes_file = String::from("src/data/quotes.txt");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--words" | "-w" => {
                if let Some(f) = args.next() {
                    words_file = f;
                };
            }
            "--quotes" | "-q" => {
                if let Some(f) = args.next() {
                    quotes_file = f;
                }
            }
            //"--online" => return Data::new_online(words_file),
            "--help" | "-h" => help(),
            _ => (),
        }
    }

    let data = Data::new_offline(words_file, quotes_file)?;

    Ok(data)
}

fn help() {
    let help = [
        "Usage: typing_test [-w <file>] [(-q <file>)|-o]\n",
        "Test your typing speed from built-in words and quotes, or provide your own words and quotes. You can also choose to get quotes from scrapping popular quotes on the Internet.\n",
        "Options:",
        "   -w, --words <file>      Provide your own words file.",
        "                           Words are separated by a new line character.",
        "   -q, --quotes <file>     Provide your own quotes file.",
        "                           Sources are sepparated by 2 new line characters.",
        "                           The first line is the source name, and each quote belonging",
        "                           to that source is separated by a new line character.",
        //"   -o, --online            Get quotes from the web. Must be connected to the internet.",
        "   -h, --help              Print this help.",
    ]
    .join("\n");

    println!("{}", help);
    std::process::exit(1);
}
