pub trait Focus {
    fn next(&mut self);
}

#[derive(PartialEq)]
pub enum TypingTestFocus {
    RestartButton,
    NextButton,
    ThemeButton,
    TypingBox,
    Nothing,
}

impl Focus for TypingTestFocus {
    fn next(&mut self) {
        match self {
            TypingTestFocus::Nothing => *self = TypingTestFocus::NextButton,
            TypingTestFocus::NextButton => *self = TypingTestFocus::RestartButton,
            TypingTestFocus::RestartButton => *self = TypingTestFocus::ThemeButton,
            TypingTestFocus::ThemeButton => *self = TypingTestFocus::NextButton,
            TypingTestFocus::TypingBox => *self = TypingTestFocus::NextButton,
        }
    }
}

#[derive(PartialEq)]
pub enum EndscreenFocus {
    NextButton,
    QuitButton,
    RestartButton,
    Nothing,
}

impl Focus for EndscreenFocus {
    fn next(&mut self) {
        match self {
            EndscreenFocus::Nothing => *self = EndscreenFocus::NextButton,
            EndscreenFocus::NextButton => *self = EndscreenFocus::RestartButton,
            EndscreenFocus::RestartButton => *self = EndscreenFocus::QuitButton,
            EndscreenFocus::QuitButton => *self = EndscreenFocus::NextButton,
        }
    }
}
