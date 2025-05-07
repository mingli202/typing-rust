#![allow(unused, non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Phonetic {
    text: Option<String>,
    audio: Option<String>,
    sourceUrl: Option<String>,
    license: Option<License>,
}

#[derive(Deserialize, Debug)]
pub struct License {
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Definition {
    definition: String,
    example: Option<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Meaning {
    partOfSpeech: String,
    definitions: Vec<Definition>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Res {
    word: String,
    phonetic: Option<String>,
    phonetics: Vec<Phonetic>,
    origin: Option<String>,
    meanings: Vec<Meaning>,
    license: Option<License>,
    sourceUrls: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct NotFound {
    title: String,
    message: String,
    resolution: String,
}
