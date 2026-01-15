//! Color configuration for log formatting.
//!
//! This module provides fine-grained control over colors for all log components,
//! matching the zylog Go library's color system.

use log::Level;
use owo_colors::{OwoColorize, Stream};
use serde::{Deserialize, Serialize};

/// Color attribute for terminal output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorAttribute {
    /// No color (transparent/reset)
    Reset,

    // Standard colors
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    // Bright/high-intensity colors
    HiBlack, // Grey
    HiRed,
    HiGreen,
    HiYellow,
    HiBlue,
    HiMagenta,
    HiCyan,
    HiWhite,
}

impl Default for ColorAttribute {
    fn default() -> Self {
        Self::Reset
    }
}

impl ColorAttribute {
    /// Apply this color to a string using owo-colors
    pub(crate) fn apply(&self, text: &str, stream: Stream) -> String {
        match self {
            Self::Reset => text.to_string(),
            Self::Black => text.if_supports_color(stream, |x| x.black()).to_string(),
            Self::Red => text.if_supports_color(stream, |x| x.red()).to_string(),
            Self::Green => text.if_supports_color(stream, |x| x.green()).to_string(),
            Self::Yellow => text.if_supports_color(stream, |x| x.yellow()).to_string(),
            Self::Blue => text.if_supports_color(stream, |x| x.blue()).to_string(),
            Self::Magenta => text.if_supports_color(stream, |x| x.magenta()).to_string(),
            Self::Cyan => text.if_supports_color(stream, |x| x.cyan()).to_string(),
            Self::White => text.if_supports_color(stream, |x| x.white()).to_string(),
            Self::HiBlack => text
                .if_supports_color(stream, |x| x.bright_black())
                .to_string(),
            Self::HiRed => text
                .if_supports_color(stream, |x| x.bright_red())
                .to_string(),
            Self::HiGreen => text
                .if_supports_color(stream, |x| x.bright_green())
                .to_string(),
            Self::HiYellow => text
                .if_supports_color(stream, |x| x.bright_yellow())
                .to_string(),
            Self::HiBlue => text
                .if_supports_color(stream, |x| x.bright_blue())
                .to_string(),
            Self::HiMagenta => text
                .if_supports_color(stream, |x| x.bright_magenta())
                .to_string(),
            Self::HiCyan => text
                .if_supports_color(stream, |x| x.bright_cyan())
                .to_string(),
            Self::HiWhite => text
                .if_supports_color(stream, |x| x.bright_white())
                .to_string(),
        }
    }

    /// Apply as background color
    pub(crate) fn apply_bg(&self, text: &str, stream: Stream) -> String {
        match self {
            Self::Reset => text.to_string(),
            Self::Black => text.if_supports_color(stream, |x| x.on_black()).to_string(),
            Self::Red => text.if_supports_color(stream, |x| x.on_red()).to_string(),
            Self::Green => text.if_supports_color(stream, |x| x.on_green()).to_string(),
            Self::Yellow => text
                .if_supports_color(stream, |x| x.on_yellow())
                .to_string(),
            Self::Blue => text.if_supports_color(stream, |x| x.on_blue()).to_string(),
            Self::Magenta => text
                .if_supports_color(stream, |x| x.on_magenta())
                .to_string(),
            Self::Cyan => text.if_supports_color(stream, |x| x.on_cyan()).to_string(),
            Self::White => text.if_supports_color(stream, |x| x.on_white()).to_string(),
            Self::HiBlack => text
                .if_supports_color(stream, |x| x.on_bright_black())
                .to_string(),
            Self::HiRed => text
                .if_supports_color(stream, |x| x.on_bright_red())
                .to_string(),
            Self::HiGreen => text
                .if_supports_color(stream, |x| x.on_bright_green())
                .to_string(),
            Self::HiYellow => text
                .if_supports_color(stream, |x| x.on_bright_yellow())
                .to_string(),
            Self::HiBlue => text
                .if_supports_color(stream, |x| x.on_bright_blue())
                .to_string(),
            Self::HiMagenta => text
                .if_supports_color(stream, |x| x.on_bright_magenta())
                .to_string(),
            Self::HiCyan => text
                .if_supports_color(stream, |x| x.on_bright_cyan())
                .to_string(),
            Self::HiWhite => text
                .if_supports_color(stream, |x| x.on_bright_white())
                .to_string(),
        }
    }
}

/// Foreground and background color configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    /// Foreground color
    pub fg: ColorAttribute,

    /// Background color
    pub bg: ColorAttribute,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            fg: ColorAttribute::Reset,
            bg: ColorAttribute::Reset,
        }
    }
}

impl Color {
    /// Create a color with just foreground
    pub fn fg(color: ColorAttribute) -> Self {
        Self {
            fg: color,
            bg: ColorAttribute::Reset,
        }
    }

    /// Create a color with foreground and background
    pub fn new(fg: ColorAttribute, bg: ColorAttribute) -> Self {
        Self { fg, bg }
    }

    /// Apply both foreground and background to text
    pub(crate) fn apply(&self, text: &str, stream: Stream) -> String {
        let with_fg = self.fg.apply(text, stream);
        if self.bg == ColorAttribute::Reset {
            with_fg
        } else {
            self.bg.apply_bg(&with_fg, stream)
        }
    }
}

// Convenience constructors
impl Color {
    pub fn black() -> Self {
        Self::fg(ColorAttribute::Black)
    }
    pub fn red() -> Self {
        Self::fg(ColorAttribute::Red)
    }
    pub fn green() -> Self {
        Self::fg(ColorAttribute::Green)
    }
    pub fn yellow() -> Self {
        Self::fg(ColorAttribute::Yellow)
    }
    pub fn blue() -> Self {
        Self::fg(ColorAttribute::Blue)
    }
    pub fn magenta() -> Self {
        Self::fg(ColorAttribute::Magenta)
    }
    pub fn cyan() -> Self {
        Self::fg(ColorAttribute::Cyan)
    }
    pub fn white() -> Self {
        Self::fg(ColorAttribute::White)
    }
    pub fn hi_black() -> Self {
        Self::fg(ColorAttribute::HiBlack)
    }
    pub fn hi_red() -> Self {
        Self::fg(ColorAttribute::HiRed)
    }
    pub fn hi_green() -> Self {
        Self::fg(ColorAttribute::HiGreen)
    }
    pub fn hi_yellow() -> Self {
        Self::fg(ColorAttribute::HiYellow)
    }
    pub fn hi_blue() -> Self {
        Self::fg(ColorAttribute::HiBlue)
    }
    pub fn hi_magenta() -> Self {
        Self::fg(ColorAttribute::HiMagenta)
    }
    pub fn hi_cyan() -> Self {
        Self::fg(ColorAttribute::HiCyan)
    }
    pub fn hi_white() -> Self {
        Self::fg(ColorAttribute::HiWhite)
    }
}

/// Fine-grained color configuration for all log components.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Colors {
    /// Timestamp color (default: Green to match current behavior)
    pub timestamp: Option<Color>,

    /// TRACE level color (default: HiBlue)
    pub level_trace: Option<Color>,

    /// DEBUG level color (default: Cyan)
    pub level_debug: Option<Color>,

    /// INFO level color (default: HiGreen)
    pub level_info: Option<Color>,

    /// WARN level color (default: HiYellow)
    pub level_warn: Option<Color>,

    /// ERROR level color (default: Red)
    pub level_error: Option<Color>,

    /// Message text color (default: Green)
    pub message: Option<Color>,

    /// Arrow separator "▶" (default: Cyan)
    pub arrow: Option<Color>,

    /// Caller file name (default: HiYellow)
    pub caller_file: Option<Color>,

    /// Caller line number (default: HiYellow)
    pub caller_line: Option<Color>,

    /// Target/module name (default: HiYellow)
    pub target: Option<Color>,

    /// Structured logging key (default: HiYellow)
    pub attr_key: Option<Color>,

    /// Structured logging value (default: Cyan)
    pub attr_value: Option<Color>,
}

impl Default for Colors {
    /// Returns colors matching current twyg behavior
    fn default() -> Self {
        Self {
            timestamp: Some(Color::green()),
            level_trace: Some(Color::hi_blue()),
            level_debug: Some(Color::cyan()),
            level_info: Some(Color::hi_green()),
            level_warn: Some(Color::hi_yellow()),
            level_error: Some(Color::red()),
            message: Some(Color::green()),
            arrow: Some(Color::cyan()),
            caller_file: Some(Color::hi_yellow()),
            caller_line: Some(Color::hi_yellow()),
            target: Some(Color::hi_yellow()),
            attr_key: Some(Color::hi_yellow()),
            attr_value: Some(Color::cyan()),
        }
    }
}

impl Colors {
    /// Get color for a specific log level
    pub(crate) fn level_color(&self, level: Level) -> Option<&Color> {
        match level {
            Level::Error => self.level_error.as_ref(),
            Level::Warn => self.level_warn.as_ref(),
            Level::Info => self.level_info.as_ref(),
            Level::Debug => self.level_debug.as_ref(),
            Level::Trace => self.level_trace.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_default() {
        let c = Color::default();
        assert_eq!(c.fg, ColorAttribute::Reset);
        assert_eq!(c.bg, ColorAttribute::Reset);
    }

    #[test]
    fn test_color_constructors() {
        let c = Color::red();
        assert_eq!(c.fg, ColorAttribute::Red);
        assert_eq!(c.bg, ColorAttribute::Reset);

        let c2 = Color::new(ColorAttribute::Green, ColorAttribute::Black);
        assert_eq!(c2.fg, ColorAttribute::Green);
        assert_eq!(c2.bg, ColorAttribute::Black);
    }

    #[test]
    fn test_colors_default_matches_current_behavior() {
        let colors = Colors::default();
        assert!(colors.timestamp.is_some());
        assert!(colors.level_error.is_some());
        assert!(colors.level_warn.is_some());
        assert!(colors.level_info.is_some());
        assert!(colors.level_debug.is_some());
        assert!(colors.level_trace.is_some());
        assert!(colors.message.is_some());
        assert!(colors.arrow.is_some());
        assert!(colors.caller_file.is_some());
        assert!(colors.caller_line.is_some());
        assert!(colors.target.is_some());
        assert!(colors.attr_key.is_some());
        assert!(colors.attr_value.is_some());
    }

    #[test]
    fn test_level_color_lookup() {
        let colors = Colors::default();
        assert!(colors.level_color(Level::Error).is_some());
        assert!(colors.level_color(Level::Warn).is_some());
        assert!(colors.level_color(Level::Info).is_some());
        assert!(colors.level_color(Level::Debug).is_some());
        assert!(colors.level_color(Level::Trace).is_some());
    }

    #[test]
    fn test_color_attribute_default() {
        let attr = ColorAttribute::default();
        assert_eq!(attr, ColorAttribute::Reset);
    }

    #[test]
    fn test_convenience_constructors() {
        assert_eq!(Color::black().fg, ColorAttribute::Black);
        assert_eq!(Color::red().fg, ColorAttribute::Red);
        assert_eq!(Color::green().fg, ColorAttribute::Green);
        assert_eq!(Color::yellow().fg, ColorAttribute::Yellow);
        assert_eq!(Color::blue().fg, ColorAttribute::Blue);
        assert_eq!(Color::magenta().fg, ColorAttribute::Magenta);
        assert_eq!(Color::cyan().fg, ColorAttribute::Cyan);
        assert_eq!(Color::white().fg, ColorAttribute::White);
        assert_eq!(Color::hi_black().fg, ColorAttribute::HiBlack);
        assert_eq!(Color::hi_red().fg, ColorAttribute::HiRed);
        assert_eq!(Color::hi_green().fg, ColorAttribute::HiGreen);
        assert_eq!(Color::hi_yellow().fg, ColorAttribute::HiYellow);
        assert_eq!(Color::hi_blue().fg, ColorAttribute::HiBlue);
        assert_eq!(Color::hi_magenta().fg, ColorAttribute::HiMagenta);
        assert_eq!(Color::hi_cyan().fg, ColorAttribute::HiCyan);
        assert_eq!(Color::hi_white().fg, ColorAttribute::HiWhite);
    }
}
