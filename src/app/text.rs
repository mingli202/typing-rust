use macroquad::color::Color;
use macroquad::text::{self, TextDimensions, TextParams};
use macroquad::{shapes, window};

use super::{Letter, Style};

pub fn print_letter(
    style: &Style,
    letter: &Letter,
    x: f32,
    y: f32,
    cursor_index: usize,
) -> TextDimensions {
    let dimensions = text::measure_text(
        &letter.letter.to_string(),
        None,
        *style.font_size.borrow() as u16,
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

    let x = x + p_x + o_x;
    let y = y + p_y + o_y;

    text::draw_text(
        &letter.letter.to_string(),
        x,
        y,
        *style.font_size.borrow(),
        *letter.color.borrow(),
    );

    if *letter.color.borrow() == *style.theme.error.borrow() {
        shapes::draw_line(
            x,
            y + 0.2 * *style.font_size.borrow(),
            x + dimensions.width,
            y + 0.2 * *style.font_size.borrow(),
            0.05 * *style.font_size.borrow(),
            *letter.color.borrow(),
        );
    }

    if letter.id == cursor_index {
        text::draw_text(
            "|",
            x - dimensions.width / 2.0,
            y,
            *style.font_size.borrow(),
            *style.theme.text.borrow(),
        );
    }

    dimensions
}

pub fn print_letters(style: &Style, letters: &[&Letter], x: f32, y: f32, cursor_index: usize) {
    let mut offset_x = 0.0;
    let offset_y = text::measure_text(
        &letters
            .iter()
            .fold(String::new(), |acc, l| acc + &l.letter.to_string()),
        None,
        *style.font_size.borrow() as u16,
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

        if text::measure_text(
            &(line_merged + &word_merged),
            None,
            *style.font_size.borrow() as u16,
            1.0,
        )
        .width
            > style.width.get() - 2.0 * p_x
        {
            print_letters(
                style,
                &line,
                style.x.get(),
                style.y.get() + lines * *style.font_size.borrow(),
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

    print_letters(
        style,
        &line,
        style.x.get(),
        style.y.get() + lines * *style.font_size.borrow(),
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

    let fsize = *style.font_size.borrow();
    if style.wrap {
        let wt = WrappedText::new(
            text,
            style.width.get(),
            TextParams {
                font_size: fsize as u16,
                ..TextParams::default()
            },
        );

        wt.print(style, x, y);
    } else {
        let TextDimensions { offset_y, .. } =
            macroquad::text::measure_text(text, None, fsize as u16, 1.0);

        text::draw_text(
            text,
            x + p_x + o_x,
            y + p_y + o_y + offset_y,
            fsize,
            *style.theme.text.borrow(),
        );
    }
}

pub struct WrappedText<'a> {
    text: Vec<String>,
    height: f32,
    width: f32,
    max_width: f32,
    text_params: TextParams<'a>,
}

impl<'a> WrappedText<'a> {
    pub fn new(text: &str, max_width: f32, text_params: TextParams<'a>) -> Self {
        let mut lines: Vec<Vec<String>> = vec![];

        let mut line: Vec<String> = vec![];

        let mut w = 0.0;

        for word in text.split_whitespace() {
            let l = line.join(" ") + " " + word;

            let TextDimensions { width, .. } =
                text::measure_text(&l, None, text_params.font_size, text_params.font_scale);

            if width > max_width {
                let _w = text::measure_text(
                    &line.join(" ")[..],
                    None,
                    text_params.font_size,
                    text_params.font_scale,
                )
                .width;

                if _w > w {
                    w = _w;
                }

                lines.push(line.clone());
                line = vec![word.to_string()];
            } else {
                line.push(word.to_string());
            }
        }
        lines.push(line);

        let text = lines.iter().map(|l| l.join(" ")).collect::<Vec<String>>();

        let last = text.last().unwrap();
        let TextDimensions { height: h_last, .. } =
            text::measure_text(last, None, text_params.font_size, text_params.font_scale);
        let height = (text.len() - 1) as f32 * text_params.font_size as f32 + h_last;

        WrappedText {
            height,
            text,
            max_width,
            width: w,
            text_params,
        }
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }
    pub fn get_width(&self) -> f32 {
        self.width
    }
    pub fn get_max_width(&self) -> f32 {
        self.max_width
    }

    pub fn print(&self, style: &Style, x: f32, y: f32) {
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

        for (i, line) in self.text.iter().enumerate() {
            let TextDimensions {
                offset_y, height, ..
            } = macroquad::text::measure_text(&line[..], None, self.text_params.font_size, 1.0);

            text::draw_text(
                &line[..],
                x + p_x + o_x,
                y + p_y + o_y + offset_y + i as f32 * self.text_params.font_size as f32,
                *style.font_size.borrow(),
                *style.theme.text.borrow(),
            );
        }
    }
}
