use macroquad::shapes;
use macroquad::text::{self, TextDimensions};

use super::{Letter, Style};

pub fn print_letter(
    style: &Style,
    letter: &Letter,
    x: f32,
    y: f32,
    cursor_index: usize,
) -> TextDimensions {
    let font_size = *style.font_size.lock().unwrap();

    let dimensions = text::measure_text(&letter.letter.to_string(), None, font_size as u16, 1.0);

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

    let x = x + p_x + o_x;
    let y = y + p_y + o_y;

    let letter_color = *letter.color.lock().unwrap();

    text::draw_text(&letter.letter.to_string(), x, y, font_size, letter_color);

    let error_color = *style.theme.error.lock().unwrap();

    if letter_color == error_color {
        shapes::draw_line(
            x,
            y + 0.2 * font_size,
            x + dimensions.width,
            y + 0.2 * font_size,
            0.05 * font_size,
            error_color,
        );
    }

    let text_color = *style.theme.text.lock().unwrap();

    if letter.id == cursor_index {
        text::draw_text("|", x - dimensions.width / 2.0, y, font_size, text_color);
    }

    dimensions
}

pub fn print_letters(style: &Style, letters: &[&Letter], x: f32, y: f32, cursor_index: usize) {
    let mut offset_x = 0.0;
    let font_size = *style.font_size.lock().unwrap() as u16;
    let offset_y = text::measure_text(
        &letters
            .iter()
            .fold(String::new(), |acc, l| acc + &l.letter.to_string()),
        None,
        font_size,
        1.0,
    )
    .offset_y;

    for letter in letters {
        offset_x += print_letter(style, letter, x + offset_x, y + offset_y, cursor_index).width;
    }
}

pub fn print_letters_wrap(style: &Style, letters: &[Letter], cursor_index: usize) -> Vec<usize> {
    let mut line_breaks = vec![];

    let mut lines = 0.0;

    let p_x = match &style.padding_x {
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

        let font_size = *style.font_size.lock().unwrap();
        if text::measure_text(&(line_merged + &word_merged), None, font_size as u16, 1.0).width
            > style.width.get() - 2.0 * p_x
        {
            print_letters(
                style,
                &line,
                style.x.get(),
                style.y.get() + lines * font_size,
                cursor_index,
            );
            lines += 1.0;
            line.drain(..);
            line_breaks.push(i - word.len())
        }

        line.append(&mut word);
        line.push(letter);
        word.drain(..);
    }

    line.append(&mut word);
    line.pop();

    let font_size = *style.font_size.lock().unwrap();
    print_letters(
        style,
        &line,
        style.x.get(),
        style.y.get() + lines * font_size,
        cursor_index,
    );

    line_breaks
}

pub fn print_text(style: &Style, text: &str, x: f32, y: f32) {
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

    let font_size = *style.font_size.lock().unwrap();

    let TextDimensions { offset_y, .. } =
        macroquad::text::measure_text(text, None, font_size as u16, 1.0);

    let text_color = *style.theme.text.lock().unwrap();
    text::draw_text(
        text,
        x + p_x + o_x,
        y + p_y + o_y + offset_y,
        font_size,
        text_color,
    );
}
