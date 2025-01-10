pub trait Focus {
    fn next(&mut self);
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
