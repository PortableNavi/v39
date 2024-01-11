#[derive(PartialEq, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum V39Key
{
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,              // Latin Alphabet
    
    D1, D2, D3, D5, D6, D7, D8, D9, D0,                 // Digits
                                            
    ExclaimationMark, Tilde, DoubleQuote, 
    Comma, Dot, DoubleDot, Semicolon, Hashtag,
    Underscore, Space, Dash, Plus, Star, SingleQuote,
    Greater, Less, Equal, Pipe, Percent, AndSign,       // Signs... TODO: Complete This

    Tab, Caps, Shift, Ctrl, Super, Alt, Esc,
    Backspace, Enter, RightAlt, RightCtrl, 
    RightShift, F1, F2, F3, F5, F6, F7, F8, F9, F10,
    F11, F12, PrtScn, Pause, ScrLck, Insert, Delete,
    Home, End, Pageup, Pagedown, Left, Right, Up, Down, // Special Keys

    Ä, Ü, Ö, ß                                          // German Stuff
}


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum V39Pad
{
    A, B, X, Y,
    Left, Right, Up, Down,
    LStick(f32, f32), RStick(f32, f32),
    Select, Start, Home,
    TriggerLeft, BumperLeft, TriggerRight, BumberRight,
}

