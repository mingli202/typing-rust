# typing-rust

Try it with `cargo install typing_test` and run `typing_test`

Fast and minimal typing test binary crate written in rust
- graphics library for rendering words and quotes
- display typing speed
- can be timer, by number of words, or quote (akin to monkeytype)
- offline default words and quotes (done)
- online web scraping for more quotes and words
- has themes, default is gruvbox

Tech Stack:
- macroquad for graphics library and keyboard input

Special thanks to Monkeytype for inspiring me to build this and providing the default words.

[![Typing demo](https://img.youtube.com/vi/qujeessD6Mg/0.jpg)](https://youtu.be/qujeessD6Mg)

Layout todos
- Composable layout
- Not reliant on gui, similar to clay
- Width/height that fit content or grow to fill everything
- Flex layout to align items
- Flex items: center, between, start, end
- Padding/gap
