use std::fmt;

// Standard text colors
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

//  RGB color with 24-bit color depth (16.7 million colors)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RgbColor { r, g, b }
    }
}

impl Color {
    //  ANSI color code
    // foreground color
    fn fg_code(&self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::BrightBlack => 90,
            Color::BrightRed => 91,
            Color::BrightGreen => 92,
            Color::BrightYellow => 93,
            Color::BrightBlue => 94,
            Color::BrightMagenta => 95,
            Color::BrightCyan => 96,
            Color::BrightWhite => 97,
        }
    }

    // background color
    fn bg_code(&self) -> u8 {
        match self {
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
            Color::BrightBlack => 100,
            Color::BrightRed => 101,
            Color::BrightGreen => 102,
            Color::BrightYellow => 103,
            Color::BrightBlue => 104,
            Color::BrightMagenta => 105,
            Color::BrightCyan => 106,
            Color::BrightWhite => 107,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    fg_rgb_color: Option<RgbColor>,
    bg_rgb_color: Option<RgbColor>,
    bold: bool,
    italic: bool,
    underline: bool,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg_color: None,
            bg_color: None,
            fg_rgb_color: None,
            bg_rgb_color: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

impl Style {
    pub fn new() -> Self {
        Style::default()
    }

    // note: look into a better implementation
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self.fg_rgb_color = None; // Clear RGB color when setting standard color
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self.bg_rgb_color = None;
        self
    }

    // New methods for RGB colors
    pub fn fg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.fg_rgb_color = Some(RgbColor::new(r, g, b));
        self.fg_color = None; // Clear standard color when setting RGB color
        self
    }

    pub fn bg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.bg_rgb_color = Some(RgbColor::new(r, g, b));
        self.bg_color = None;
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    fn format_prefix(&self) -> String {
        let mut codes = Vec::new();

        // Standard foreground color
        if let Some(fg) = self.fg_color {
            codes.push(fg.fg_code().to_string());
        }

        // RGB foreground color
        if let Some(rgb) = self.fg_rgb_color {
            codes.push(format!("38;2;{};{};{}", rgb.r, rgb.g, rgb.b));
        }

        // Standard background color
        if let Some(bg) = self.bg_color {
            codes.push(bg.bg_code().to_string());
        }

        // RGB background color
        if let Some(rgb) = self.bg_rgb_color {
            codes.push(format!("48;2;{};{};{}", rgb.r, rgb.g, rgb.b));
        }

        if self.bold {
            codes.push("1".to_string());
        }

        if self.italic {
            codes.push("3".to_string());
        }

        if self.underline {
            codes.push("4".to_string());
        }

        if codes.is_empty() {
            return String::new();
        }

        format!("\x1b[{}m", codes.join(";"))
    }

    fn format_suffix() -> &'static str {
        "\x1b[0m"
    }

    pub fn paint<T: AsRef<str>>(&self, text: T) -> ColoredString {
        ColoredString {
            text: text.as_ref().to_string(),
            style: *self,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColoredString {
    pub text: String, // Made public to allow easy access
    style: Style,
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = self.style.format_prefix();
        if prefix.is_empty() {
            write!(f, "{}", self.text)
        } else {
            write!(f, "{}{}{}", prefix, self.text, Style::format_suffix())
        }
    }
}

// Standard color helper functions
pub fn red<T: AsRef<str>>(text: T) -> ColoredString {
    Style::new().fg(Color::Red).paint(text)
}

pub fn green<T: AsRef<str>>(text: T) -> ColoredString {
    Style::new().fg(Color::Green).paint(text)
}

pub fn blue<T: AsRef<str>>(text: T) -> ColoredString {
    Style::new().fg(Color::Blue).paint(text)
}

pub fn yellow<T: AsRef<str>>(text: T) -> ColoredString {
    Style::new().fg(Color::Yellow).paint(text)
}

pub fn magenta<T: AsRef<str>>(text: T) -> ColoredString {
    Style::new().fg(Color::Magenta).paint(text)
}

pub fn cyan<T: AsRef<str>>(text: T) -> ColoredString {
    Style::new().fg(Color::Cyan).paint(text)
}

pub fn white<T: AsRef<str>>(text: T) -> ColoredString {
    Style::new().fg(Color::White).paint(text)
}

pub fn black<T: AsRef<str>>(text: T) -> ColoredString {
    Style::new().fg(Color::Black).paint(text)
}

// RGB color helper functions
pub fn rgb<T: AsRef<str>>(r: u8, g: u8, b: u8, text: T) -> ColoredString {
    Style::new().fg_rgb(r, g, b).paint(text)
}

pub fn on_rgb<T: AsRef<str>>(r: u8, g: u8, b: u8, text: T) -> ColoredString {
    Style::new().bg_rgb(r, g, b).paint(text)
}

pub trait Colorize {
    fn red(&self) -> ColoredString;
    fn green(&self) -> ColoredString;
    fn blue(&self) -> ColoredString;
    fn yellow(&self) -> ColoredString;
    fn magenta(&self) -> ColoredString;
    fn cyan(&self) -> ColoredString;
    fn white(&self) -> ColoredString;
    fn black(&self) -> ColoredString;
    fn bold(&self) -> ColoredString;
    fn italic(&self) -> ColoredString;
    fn underline(&self) -> ColoredString;
    fn color(&self, color: Color) -> ColoredString;
    fn bg_color(&self, color: Color) -> ColoredString;
    
    fn rgb(&self, r: u8, g: u8, b: u8) -> ColoredString;
    fn on_rgb(&self, r: u8, g: u8, b: u8) -> ColoredString;
}

impl<T: AsRef<str>> Colorize for T {
    fn red(&self) -> ColoredString {
        red(self)
    }

    fn green(&self) -> ColoredString {
        green(self)
    }

    fn blue(&self) -> ColoredString {
        blue(self)
    }

    fn yellow(&self) -> ColoredString {
        yellow(self)
    }

    fn magenta(&self) -> ColoredString {
        magenta(self)
    }

    fn cyan(&self) -> ColoredString {
        cyan(self)
    }

    fn white(&self) -> ColoredString {
        white(self)
    }

    fn black(&self) -> ColoredString {
        black(self)
    }

    fn bold(&self) -> ColoredString {
        Style::new().bold().paint(self)
    }

    fn italic(&self) -> ColoredString {
        Style::new().italic().paint(self)
    }

    fn underline(&self) -> ColoredString {
        Style::new().underline().paint(self)
    }

    fn color(&self, color: Color) -> ColoredString {
        Style::new().fg(color).paint(self)
    }

    fn bg_color(&self, color: Color) -> ColoredString {
        Style::new().bg(color).paint(self)
    }
    
    fn rgb(&self, r: u8, g: u8, b: u8) -> ColoredString {
        Style::new().fg_rgb(r, g, b).paint(self)
    }
    
    fn on_rgb(&self, r: u8, g: u8, b: u8) -> ColoredString {
        Style::new().bg_rgb(r, g, b).paint(self)
    }
}

impl AsRef<str> for ColoredString {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_text() {
        let colored = red("This is red text");
        assert_eq!(colored.to_string(), "\x1b[31mThis is red text\x1b[0m");
    }

    #[test]
    fn test_colorize_trait() {
        let colored = "Blue text".blue();
        assert_eq!(colored.to_string(), "\x1b[34mBlue text\x1b[0m");
    }

    #[test]
    fn test_combined_styles() {
        let styled = Style::new()
            .fg(Color::Green)
            .bg(Color::Black)
            .bold()
            .paint("Bold green text on black background");
        assert_eq!(
            styled.to_string(),
            "\x1b[32;40;1mBold green text on black background\x1b[0m"
        );
    }
    
    #[test]
    fn test_rgb_color() {
        let colored = rgb(255, 100, 50, "RGB text");
        assert_eq!(colored.to_string(), "\x1b[38;2;255;100;50mRGB text\x1b[0m");
    }
    
    #[test]
    fn test_rgb_trait_method() {
        let colored = "RGB trait".rgb(50, 100, 255);
        assert_eq!(colored.to_string(), "\x1b[38;2;50;100;255mRGB trait\x1b[0m");
    }
    
    #[test]
    fn test_bg_rgb_color() {
        let colored = on_rgb(50, 100, 255, "Background RGB");
        assert_eq!(colored.to_string(), "\x1b[48;2;50;100;255mBackground RGB\x1b[0m");
    }
    
    #[test]
    fn test_complex_rgb_styling() {
        let styled = Style::new()
            .fg_rgb(255, 50, 50)
            .bg_rgb(20, 20, 50)
            .bold()
            .paint("Complex RGB styling");
        assert_eq!(
            styled.to_string(),
            "\x1b[38;2;255;50;50;48;2;20;20;50;1mComplex RGB styling\x1b[0m"
        );
    }
}