//! Color configuration for log formatting.
//!
//! This module provides fine-grained control over colors for all log components,
//! matching the zylog Go library's color system.

use log::Level;
use owo_colors::{OwoColorize, Stream};
use serde::{Deserialize, Serialize};

/// Color attribute for terminal output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorAttribute {
    /// No color (transparent/reset)
    #[default]
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

    #[test]
    fn test_color_attribute_apply_all_variants() {
        // Test all ColorAttribute variants
        let text = "test";

        // Reset should return plain text
        assert_eq!(ColorAttribute::Reset.apply(text, Stream::Stdout), "test");

        // Test all standard colors (just verify they return something)
        assert!(!ColorAttribute::Black.apply(text, Stream::Stdout).is_empty());
        assert!(!ColorAttribute::Red.apply(text, Stream::Stdout).is_empty());
        assert!(!ColorAttribute::Green.apply(text, Stream::Stdout).is_empty());
        assert!(!ColorAttribute::Yellow
            .apply(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::Blue.apply(text, Stream::Stdout).is_empty());
        assert!(!ColorAttribute::Magenta
            .apply(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::Cyan.apply(text, Stream::Stdout).is_empty());
        assert!(!ColorAttribute::White.apply(text, Stream::Stdout).is_empty());

        // Test all bright colors
        assert!(!ColorAttribute::HiBlack
            .apply(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiRed.apply(text, Stream::Stdout).is_empty());
        assert!(!ColorAttribute::HiGreen
            .apply(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiYellow
            .apply(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiBlue
            .apply(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiMagenta
            .apply(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiCyan
            .apply(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiWhite
            .apply(text, Stream::Stdout)
            .is_empty());
    }

    #[test]
    fn test_color_attribute_apply_bg_all_variants() {
        // Test all ColorAttribute variants for background
        let text = "test";

        // Reset should return plain text
        assert_eq!(ColorAttribute::Reset.apply_bg(text, Stream::Stdout), "test");

        // Test all standard background colors
        assert!(!ColorAttribute::Black
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::Red
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::Green
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::Yellow
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::Blue
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::Magenta
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::Cyan
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::White
            .apply_bg(text, Stream::Stdout)
            .is_empty());

        // Test all bright background colors
        assert!(!ColorAttribute::HiBlack
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiRed
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiGreen
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiYellow
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiBlue
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiMagenta
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiCyan
            .apply_bg(text, Stream::Stdout)
            .is_empty());
        assert!(!ColorAttribute::HiWhite
            .apply_bg(text, Stream::Stdout)
            .is_empty());
    }

    #[test]
    fn test_color_attribute_with_stderr() {
        // Test that colors work with Stderr stream
        let text = "test";

        assert!(!ColorAttribute::Red.apply(text, Stream::Stderr).is_empty());
        assert!(!ColorAttribute::Green
            .apply_bg(text, Stream::Stderr)
            .is_empty());
        assert!(!ColorAttribute::HiYellow
            .apply(text, Stream::Stderr)
            .is_empty());
    }

    #[test]
    fn test_color_apply_with_background() {
        let color = Color::new(ColorAttribute::White, ColorAttribute::Red);
        let result = color.apply("test", Stream::Stdout);

        // Should return text (with or without colors depending on terminal support)
        assert!(!result.is_empty());
        assert!(result.contains("test"));
    }

    #[test]
    fn test_color_apply_fg_only() {
        let color = Color::fg(ColorAttribute::Green);
        let result = color.apply("test", Stream::Stdout);

        // Should have applied foreground only
        assert!(!result.is_empty());
    }

    #[test]
    fn test_color_apply_reset_bg() {
        // Color with Reset background should only apply foreground
        let color = Color {
            fg: ColorAttribute::Red,
            bg: ColorAttribute::Reset,
        };
        let result = color.apply("test", Stream::Stdout);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_colors_level_color_returns_correct_option() {
        let colors = Colors::default();

        // All default colors should be Some
        assert!(colors.level_color(Level::Error).is_some());
        assert!(colors.level_color(Level::Warn).is_some());
        assert!(colors.level_color(Level::Info).is_some());
        assert!(colors.level_color(Level::Debug).is_some());
        assert!(colors.level_color(Level::Trace).is_some());

        // Test with None colors
        let empty_colors = Colors {
            timestamp: None,
            level_trace: None,
            level_debug: None,
            level_info: None,
            level_warn: None,
            level_error: None,
            message: None,
            arrow: None,
            caller_file: None,
            caller_line: None,
            target: None,
            attr_key: None,
            attr_value: None,
        };

        assert!(empty_colors.level_color(Level::Error).is_none());
        assert!(empty_colors.level_color(Level::Warn).is_none());
        assert!(empty_colors.level_color(Level::Info).is_none());
        assert!(empty_colors.level_color(Level::Debug).is_none());
        assert!(empty_colors.level_color(Level::Trace).is_none());
    }

    #[test]
    fn test_color_eq() {
        let c1 = Color::red();
        let c2 = Color::red();
        let c3 = Color::blue();

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_color_attribute_eq() {
        assert_eq!(ColorAttribute::Red, ColorAttribute::Red);
        assert_ne!(ColorAttribute::Red, ColorAttribute::Blue);
        assert_ne!(ColorAttribute::Red, ColorAttribute::HiRed);
    }

    #[test]
    fn test_color_clone() {
        let c1 = Color::new(ColorAttribute::Red, ColorAttribute::Blue);
        let c2 = c1.clone();
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_color_attribute_clone() {
        let attr = ColorAttribute::HiGreen;
        let cloned = attr.clone();
        assert_eq!(attr, cloned);
    }

    #[test]
    fn test_colors_clone() {
        let colors1 = Colors::default();
        let colors2 = colors1.clone();
        assert_eq!(colors1.timestamp, colors2.timestamp);
        assert_eq!(colors1.level_error, colors2.level_error);
    }

    #[test]
    fn test_color_debug() {
        let color = Color::red();
        let debug_str = format!("{:?}", color);
        assert!(debug_str.contains("Color"));
    }

    #[test]
    fn test_color_attribute_debug() {
        let attr = ColorAttribute::HiYellow;
        let debug_str = format!("{:?}", attr);
        assert!(debug_str.contains("HiYellow"));
    }

    #[test]
    fn test_colors_debug() {
        let colors = Colors::default();
        let debug_str = format!("{:?}", colors);
        assert!(debug_str.contains("Colors"));
    }

    #[test]
    fn test_colors_partial_eq() {
        let c1 = Colors::default();
        let c2 = Colors::default();
        assert_eq!(c1, c2);

        let mut c3 = Colors::default();
        c3.message = None;
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_all_color_convenience_constructors_have_reset_bg() {
        // All convenience constructors should have Reset background
        assert_eq!(Color::black().bg, ColorAttribute::Reset);
        assert_eq!(Color::red().bg, ColorAttribute::Reset);
        assert_eq!(Color::green().bg, ColorAttribute::Reset);
        assert_eq!(Color::yellow().bg, ColorAttribute::Reset);
        assert_eq!(Color::blue().bg, ColorAttribute::Reset);
        assert_eq!(Color::magenta().bg, ColorAttribute::Reset);
        assert_eq!(Color::cyan().bg, ColorAttribute::Reset);
        assert_eq!(Color::white().bg, ColorAttribute::Reset);
        assert_eq!(Color::hi_black().bg, ColorAttribute::Reset);
        assert_eq!(Color::hi_red().bg, ColorAttribute::Reset);
        assert_eq!(Color::hi_green().bg, ColorAttribute::Reset);
        assert_eq!(Color::hi_yellow().bg, ColorAttribute::Reset);
        assert_eq!(Color::hi_blue().bg, ColorAttribute::Reset);
        assert_eq!(Color::hi_magenta().bg, ColorAttribute::Reset);
        assert_eq!(Color::hi_cyan().bg, ColorAttribute::Reset);
        assert_eq!(Color::hi_white().bg, ColorAttribute::Reset);
    }

    #[test]
    fn test_color_new_sets_both_fg_and_bg() {
        let color = Color::new(ColorAttribute::Cyan, ColorAttribute::Magenta);
        assert_eq!(color.fg, ColorAttribute::Cyan);
        assert_eq!(color.bg, ColorAttribute::Magenta);
    }

    #[test]
    fn test_colors_default_has_all_fields_set() {
        let colors = Colors::default();

        // Verify all fields are Some (not None)
        assert!(colors.timestamp.is_some());
        assert!(colors.level_trace.is_some());
        assert!(colors.level_debug.is_some());
        assert!(colors.level_info.is_some());
        assert!(colors.level_warn.is_some());
        assert!(colors.level_error.is_some());
        assert!(colors.message.is_some());
        assert!(colors.arrow.is_some());
        assert!(colors.caller_file.is_some());
        assert!(colors.caller_line.is_some());
        assert!(colors.target.is_some());
        assert!(colors.attr_key.is_some());
        assert!(colors.attr_value.is_some());
    }

    #[test]
    fn test_color_attribute_serialize_deserialize() {
        let attr = ColorAttribute::HiCyan;
        let serialized = serde_json::to_string(&attr).unwrap();
        let deserialized: ColorAttribute = serde_json::from_str(&serialized).unwrap();
        assert_eq!(attr, deserialized);

        // Test Reset
        let reset = ColorAttribute::Reset;
        let serialized = serde_json::to_string(&reset).unwrap();
        let deserialized: ColorAttribute = serde_json::from_str(&serialized).unwrap();
        assert_eq!(reset, deserialized);
    }

    #[test]
    fn test_color_serialize_deserialize() {
        let color = Color::new(ColorAttribute::Red, ColorAttribute::Yellow);
        let serialized = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&serialized).unwrap();
        assert_eq!(color, deserialized);

        // Test with default
        let default_color = Color::default();
        let serialized = serde_json::to_string(&default_color).unwrap();
        let deserialized: Color = serde_json::from_str(&serialized).unwrap();
        assert_eq!(default_color, deserialized);
    }

    #[test]
    fn test_colors_serialize_deserialize() {
        let colors = Colors::default();
        let serialized = serde_json::to_string(&colors).unwrap();
        let deserialized: Colors = serde_json::from_str(&serialized).unwrap();
        assert_eq!(colors, deserialized);

        // Test with some None values
        let partial_colors = Colors {
            timestamp: Some(Color::green()),
            level_trace: None,
            level_debug: Some(Color::cyan()),
            level_info: None,
            level_warn: Some(Color::yellow()),
            level_error: Some(Color::red()),
            message: None,
            arrow: Some(Color::cyan()),
            caller_file: None,
            caller_line: None,
            target: Some(Color::hi_yellow()),
            attr_key: None,
            attr_value: Some(Color::cyan()),
        };
        let serialized = serde_json::to_string(&partial_colors).unwrap();
        let deserialized: Colors = serde_json::from_str(&serialized).unwrap();
        assert_eq!(partial_colors, deserialized);
    }

    #[test]
    fn test_color_apply_multiple_streams() {
        let color = Color::fg(ColorAttribute::Red);

        // Test with both Stdout and Stderr
        let stdout_result = color.apply("test", Stream::Stdout);
        let stderr_result = color.apply("test", Stream::Stderr);

        assert!(!stdout_result.is_empty());
        assert!(!stderr_result.is_empty());
    }

    #[test]
    fn test_all_convenience_constructors_work() {
        // Ensure all convenience constructors create valid Color instances
        let constructors = vec![
            Color::black(),
            Color::red(),
            Color::green(),
            Color::yellow(),
            Color::blue(),
            Color::magenta(),
            Color::cyan(),
            Color::white(),
            Color::hi_black(),
            Color::hi_red(),
            Color::hi_green(),
            Color::hi_yellow(),
            Color::hi_blue(),
            Color::hi_magenta(),
            Color::hi_cyan(),
            Color::hi_white(),
        ];

        for color in constructors {
            // Each should have a non-Reset fg and Reset bg
            assert_ne!(color.fg, ColorAttribute::Reset);
            assert_eq!(color.bg, ColorAttribute::Reset);

            // Each should be able to apply to text
            let result = color.apply("test", Stream::Stdout);
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_color_attribute_all_variants_count() {
        // Ensure we're testing all 17 variants (Reset + 16 colors)
        let all_variants = vec![
            ColorAttribute::Reset,
            ColorAttribute::Black,
            ColorAttribute::Red,
            ColorAttribute::Green,
            ColorAttribute::Yellow,
            ColorAttribute::Blue,
            ColorAttribute::Magenta,
            ColorAttribute::Cyan,
            ColorAttribute::White,
            ColorAttribute::HiBlack,
            ColorAttribute::HiRed,
            ColorAttribute::HiGreen,
            ColorAttribute::HiYellow,
            ColorAttribute::HiBlue,
            ColorAttribute::HiMagenta,
            ColorAttribute::HiCyan,
            ColorAttribute::HiWhite,
        ];

        assert_eq!(all_variants.len(), 17);

        // Test each can be serialized
        for variant in all_variants {
            let serialized = serde_json::to_string(&variant).unwrap();
            assert!(!serialized.is_empty());
        }
    }

    #[test]
    fn test_color_default_apply() {
        // Test applying default color (both Reset)
        let default_color = Color::default();
        let result = default_color.apply("test", Stream::Stdout);
        // Should return plain text since both fg and bg are Reset
        assert_eq!(result, "test");
    }

    #[test]
    fn test_color_apply_with_non_reset_bg() {
        // Test that non-Reset background gets applied
        let color = Color::new(ColorAttribute::White, ColorAttribute::Blue);
        let result = color.apply("test", Stream::Stdout);
        assert!(!result.is_empty());
        assert!(result.contains("test"));
    }

    #[test]
    fn test_colors_all_none_values() {
        // Test Colors struct with all None values
        let empty_colors = Colors {
            timestamp: None,
            level_trace: None,
            level_debug: None,
            level_info: None,
            level_warn: None,
            level_error: None,
            message: None,
            arrow: None,
            caller_file: None,
            caller_line: None,
            target: None,
            attr_key: None,
            attr_value: None,
        };

        // Verify all level_color calls return None
        assert!(empty_colors.level_color(Level::Error).is_none());
        assert!(empty_colors.level_color(Level::Warn).is_none());
        assert!(empty_colors.level_color(Level::Info).is_none());
        assert!(empty_colors.level_color(Level::Debug).is_none());
        assert!(empty_colors.level_color(Level::Trace).is_none());

        // Verify it can be serialized/deserialized
        let serialized = serde_json::to_string(&empty_colors).unwrap();
        let deserialized: Colors = serde_json::from_str(&serialized).unwrap();
        assert_eq!(empty_colors, deserialized);
    }

    #[test]
    fn test_color_with_reset_fg_non_reset_bg() {
        // Edge case: Reset foreground with colored background
        let color = Color::new(ColorAttribute::Reset, ColorAttribute::Red);
        let result = color.apply("test", Stream::Stdout);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_all_color_constructors_return_unique_colors() {
        // Verify each constructor creates a distinct color
        let colors = vec![
            Color::black(),
            Color::red(),
            Color::green(),
            Color::yellow(),
            Color::blue(),
            Color::magenta(),
            Color::cyan(),
            Color::white(),
            Color::hi_black(),
            Color::hi_red(),
            Color::hi_green(),
            Color::hi_yellow(),
            Color::hi_blue(),
            Color::hi_magenta(),
            Color::hi_cyan(),
            Color::hi_white(),
        ];

        // Check that not all colors are equal
        for i in 0..colors.len() {
            for j in (i + 1)..colors.len() {
                assert_ne!(colors[i], colors[j]);
            }
        }
    }

    #[test]
    fn test_comprehensive_color_attribute_coverage() {
        // Explicitly test every ColorAttribute variant with both streams
        let test_text = "coverage";
        let all_variants = [
            ColorAttribute::Reset,
            ColorAttribute::Black,
            ColorAttribute::Red,
            ColorAttribute::Green,
            ColorAttribute::Yellow,
            ColorAttribute::Blue,
            ColorAttribute::Magenta,
            ColorAttribute::Cyan,
            ColorAttribute::White,
            ColorAttribute::HiBlack,
            ColorAttribute::HiRed,
            ColorAttribute::HiGreen,
            ColorAttribute::HiYellow,
            ColorAttribute::HiBlue,
            ColorAttribute::HiMagenta,
            ColorAttribute::HiCyan,
            ColorAttribute::HiWhite,
        ];

        for variant in all_variants.iter() {
            // Test foreground with both streams
            let fg_stdout = variant.apply(test_text, Stream::Stdout);
            let fg_stderr = variant.apply(test_text, Stream::Stderr);
            assert!(fg_stdout.contains(test_text));
            assert!(fg_stderr.contains(test_text));

            // Test background with both streams
            let bg_stdout = variant.apply_bg(test_text, Stream::Stdout);
            let bg_stderr = variant.apply_bg(test_text, Stream::Stderr);
            assert!(bg_stdout.contains(test_text));
            assert!(bg_stderr.contains(test_text));

            // Test combined fg+bg through Color struct
            let color = Color::new(*variant, *variant);
            let combined = color.apply(test_text, Stream::Stdout);
            assert!(combined.contains(test_text));
        }
    }

    #[test]
    fn test_color_apply_branch_coverage() {
        // Explicitly test both branches of the if statement in Color::apply

        // Branch 1: bg == ColorAttribute::Reset (only apply fg)
        let color_reset_bg = Color {
            fg: ColorAttribute::Red,
            bg: ColorAttribute::Reset,
        };
        let result1 = color_reset_bg.apply("test", Stream::Stdout);
        assert!(result1.contains("test"));

        // Branch 2: bg != ColorAttribute::Reset (apply both fg and bg)
        let color_with_bg = Color {
            fg: ColorAttribute::Red,
            bg: ColorAttribute::Yellow,
        };
        let result2 = color_with_bg.apply("test", Stream::Stdout);
        assert!(result2.contains("test"));

        // Additional edge cases
        let color_both_reset = Color {
            fg: ColorAttribute::Reset,
            bg: ColorAttribute::Reset,
        };
        assert_eq!(color_both_reset.apply("test", Stream::Stdout), "test");

        let color_reset_fg = Color {
            fg: ColorAttribute::Reset,
            bg: ColorAttribute::Blue,
        };
        let result3 = color_reset_fg.apply("test", Stream::Stdout);
        assert!(result3.contains("test"));
    }
}
