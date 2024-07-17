mod default_font;
mod small;
mod standard;

pub fn get_font(font_name: &str) -> [&str; 344] {
    match font_name {
        "standard" => standard::STANDARD,
        "small" => small::SMALL,
        "default" => default_font::DEFAULT,
        _ => default_font::DEFAULT,
    }
}
