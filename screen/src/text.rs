use macroquad::text::{self, TextDimensions};

use crate::component::Style;

pub fn print_text(style: &Style, text: &str, x: f32, y: f32) {
    let TextDimensions { offset_y, .. } =
        text::measure_text(text, None, style.font_size as u16, 1.0);

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
        text,
        x + p_x + o_x,
        y + p_y + o_y + offset_y,
        style.font_size,
        *style.theme.text.borrow(),
    );
}

pub fn print_text_wrap(style: &Style, text: &str) {
    let mut i = -1;
    let mut k: i32 = 0;
    let mut last = i;
    let mut lines = 0;

    let p_x = match &style.padding_x {
        Some(p) => p.get(),
        _ => 0.0,
    };

    let p_y = match &style.padding_y {
        Some(p) => p.get(),
        _ => 0.0,
    };

    for word in text.split(' ') {
        i += word.len() as i32 + 1;

        let y = style.y.get() + lines as f32 * style.font_size;
        if y > style.y.get() + style.height.get() - p_y {
            return;
        }

        let TextDimensions { width, .. } = text::measure_text(
            &text[k as usize..i as usize],
            None,
            style.font_size as u16,
            1.0,
        );

        if width > style.width.get() - 2.0 * p_x {
            print_text(style, &text[k as usize..last as usize], style.x.get(), y);

            lines += 1;
            k = last + 1;
        }

        last = i;
    }

    let y = style.y.get() + lines as f32 * style.font_size;

    print_text(style, &text[k as usize..last as usize], style.x.get(), y);
}
