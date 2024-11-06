use data_provider::Data;
use std::env::Args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = Data::new_offline()?;
    println!("{:#?}", data);

    Ok(())
}

fn parse_args(mut args: Args) {
    args.next();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            // to add or delete quotes and words
            "word" => (),
            "quote" => (),
            _ => (),
        }
    }
}
