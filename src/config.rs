use once_cell::sync::Lazy;
use ratatui::style::Color;

pub struct Theme {
    pub background: Color,
    pub border: Color,
    pub text: Color,
    pub selection: Color,
    pub selection_text: Color,
    pub selection_match: Color,
    pub prompt: Color,
    pub placeholder: Color,
}

pub static THEME: Lazy<Theme> = Lazy::new(|| Theme {
    background: Color::Rgb(32, 19, 30),
    border: Color::Rgb(125, 61, 82),
    text: Color::Rgb(242, 242, 242),
    selection: Color::Rgb(70, 41, 65),
    selection_text: Color::Rgb(242, 242, 242),
    selection_match: Color::Rgb(142, 64, 87),
    prompt: Color::Rgb(88, 110, 117),
    placeholder: Color::Rgb(88, 110, 117),
});
