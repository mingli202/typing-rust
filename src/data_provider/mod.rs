use rand::prelude::*;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Quote {
    pub source: String,
    pub quote: String,
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

    pub fn new_offline(
        words_path: Option<String>,
        quotes_path: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let words = if let Some(p) = words_path {
            fs::read_to_string(p)?
        } else {
            include_str!("../data/words.txt").to_string()
        }
        .split('\n')
        .map(|w| w.to_string())
        .collect();

        let quotes = if let Some(p) = quotes_path {
            fs::read_to_string(p)?
        } else {
            include_str!("../data/quotes.txt").to_string()
        }
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
        .filter(|q| !q.quote.is_empty())
        .collect();

        Ok(Data { words, quotes })
    }

    pub fn new_online(words_file: String) -> Result<Self, Box<dyn Error>> {
        let words = fs::read_to_string(words_file)?
            .split('\n')
            .map(|w| w.to_string())
            .collect();

        Ok(Data {
            words,
            quotes: vec![],
        })
    }

    pub fn get_random_word(&self) -> &str {
        let mut rng = rand::thread_rng();
        self.words.choose(&mut rng).unwrap()
    }

    pub fn get_random_quote(&self) -> &Quote {
        let mut rng = rand::thread_rng();
        self.quotes.choose(&mut rng).unwrap()
    }

    pub fn get_n_random_words(&self, n: usize) -> Vec<&String> {
        let mut rng = rand::thread_rng();

        let mut v = Vec::with_capacity(n);

        let mut last = -1;
        let mut ind = -1;

        for _ in 0..n {
            while ind == last {
                ind = rng.gen_range(0..self.words.len()) as i32;
            }

            v.push(&self.words[ind as usize]);

            last = ind;
        }

        v
    }

    pub fn get_n_random_quotes(&self, n: usize) -> Vec<&Quote> {
        let mut rng = rand::thread_rng();

        let mut v = Vec::with_capacity(n);

        let mut last = -1;
        let mut ind = -1;

        for _ in 0..n {
            while ind == last {
                ind = rng.gen_range(0..self.quotes.len()) as i32;
            }

            v.push(&self.quotes[ind as usize]);

            last = ind;
        }

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn number_of_source() {
        let data = Data::new_offline(None, None).unwrap();
        assert_eq!(3001, data.words.len());
        assert_eq!(
            33,
            data.quotes
                .iter()
                .map(|q| q.source.clone())
                .collect::<HashSet<String>>()
                .len()
        )
    }

    #[test]
    fn random_words_and_quotes() {
        let data = Data::new_offline(None, None).unwrap();
        let random_words = data.get_n_random_words(10);
        let random_quotes = data.get_n_random_quotes(10);

        assert_eq!(10, random_words.len());
        assert_eq!(10, random_quotes.len());

        let mut last = String::new();

        for word in random_words {
            if last == *word {
                panic!("Repeating word");
            }
            last = (*word).clone();
        }

        for quote in random_quotes {
            if last == *quote.quote {
                panic!("Repeating quote");
            }
            last = quote.quote.clone();
        }
    }
}
