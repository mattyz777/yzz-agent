pub enum Action {
    Quit,
    InputChar(char),
    Backspace,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    NewLine,
    Enter,
}