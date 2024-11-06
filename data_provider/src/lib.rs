use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Quote {
    source: String,
    quote: String,
}

#[derive(Debug)]
pub struct Data {
    words: Vec<String>,
    quotes: Vec<Quote>,
}

impl Data {
    pub fn get_words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn get_quotes(&self) -> &Vec<Quote> {
        &self.quotes
    }

    pub fn new_offline() -> Result<Self, Box<dyn Error>> {
        let cwd = std::env::current_dir()?
            .to_str()
            .unwrap()
            .split("typing_test")
            .collect::<Vec<&str>>()[0]
            .to_string();

        let words = fs::read_to_string(cwd.clone() + "/typing_test/data_provider/data/words.txt")?
            .split('\n')
            .map(|w| w.to_string())
            .collect();

        let quotes =
            fs::read_to_string(cwd.clone() + "/typing_test/data_provider/data/quotes.txt")?
                .split("\n\n")
                .flat_map(|entry| {
                    let mut entry: Vec<String> = entry.split('\n').map(|s| s.to_string()).collect();
                    let source = entry[0].clone();
                    let mut v = vec![];

                    while let Some(quote) = entry.pop() {
                        if quote != source {
                            v.push(Quote {
                                source: source.clone(),
                                quote,
                            });
                        }
                    }
                    v
                })
                .collect();

        Ok(Data { words, quotes })
    }

    pub fn new_online() -> Self {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn number_of_source() {
        let data = Data::new_offline().unwrap();
        assert_eq!(3001, data.words.len());
        assert_eq!(
            28,
            data.quotes
                .iter()
                .map(|q| q.source.clone())
                .collect::<HashSet<String>>()
                .len()
        )
    }
}
