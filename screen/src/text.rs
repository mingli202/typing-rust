use macroquad::text::{self, TextDimensions};

use crate::component::textbox::Letter;
use crate::component::Style;

pub fn print_letter(style: &Style, letter: &Letter, x: f32, y: f32) -> TextDimensions {
    let dimensions = text::measure_text(
        &letter.letter.to_string(),
        None,
        style.font_size as u16,
        1.0,
    );

    let p_x = match &style.padding_x {
        Some(p) => p.get(),
        _ => 0.0,
    };

    let o_x = match &style.offset_x {
        Some(p) => p.get(),
        _ => 0.0,
    };

    let p_y = match &style.padding_y {
        Some(p) => p.get(),
        _ => 0.0,
    };

    let o_y = match &style.offset_y {
        Some(p) => p.get(),
        _ => 0.0,
    };

    text::draw_text(
        &letter.letter.to_string(),
        x + p_x + o_x,
        y + p_y + o_y,
        style.font_size,
        *style.theme.text.borrow(),
    );

    dimensions
}

pub fn print_text(style: &Style, letters: &[&Letter], x: f32, y: f32) {
    let mut offset_x = 0.0;
    let mut offset_y = text::measure_text(
        &letters
            .iter()
            .fold(String::new(), |acc, l| acc + &l.letter.to_string()),
        None,
        style.font_size as u16,
        1.0,
    )
    .offset_y;

    for letter in letters {
        offset_x += print_letter(style, letter, x + offset_x, y + offset_y).width;
    }
}

pub fn print_text_wrap(style: &Style, letters: &[Letter]) {
    let mut lines = 0.0;

    let p_x = match &style.padding_x {
        Some(p) => p.get(),
        _ => 0.0,
    };

    let p_y = match &style.padding_y {
        Some(p) => p.get(),
        _ => 0.0,
    };

    let mut line: Vec<&Letter> = vec![];
    let mut word: Vec<&Letter> = vec![];

    for (i, letter) in letters.iter().enumerate() {
        if letter.letter != ' ' {
            word.push(letter);
            if i != letters.len() - 1 {
                continue;
            }
        }

        let line_merged: String = line
            .iter()
            .fold(String::new(), |acc, l| acc + &l.letter.to_string());

        let word_merged: String = word
            .iter()
            .fold(String::new(), |acc, l| acc + &l.letter.to_string());

        if text::measure_text(
            &(line_merged + &word_merged),
            None,
            style.font_size as u16,
            1.0,
        )
        .width
            > style.width.get() - 2.0 * p_x
        {
            print_text(
                style,
                &line,
                style.x.get(),
                style.y.get() + lines * style.font_size,
            );
            lines += 1.0;
            line.drain(..);
        }

        line.append(&mut word);
        line.push(letter);
        word.drain(..);

        if lines * style.font_size > style.y.get() + style.height.get() - p_y {
            return;
        }
    }

    line.append(&mut word);
    line.pop();

    print_text(
        style,
        &line,
        style.x.get(),
        style.y.get() + lines * style.font_size,
    );
}
