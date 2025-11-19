//! User-facing output formatting and helpful messages
//!
//! This module provides utilities for displaying information, tips, hints,
//! and progress to users in a delightful and informative way.

use std::fmt;

/// Color codes for terminal output
pub mod colors {
    /// Reset to default color
    pub const RESET: &str = "\x1b[0m";
    /// Bold text
    pub const BOLD: &str = "\x1b[1m";
    /// Dim text
    pub const DIM: &str = "\x1b[2m";

    /// Green (success)
    pub const GREEN: &str = "\x1b[32m";
    /// Yellow (warning)
    pub const YELLOW: &str = "\x1b[33m";
    /// Blue (info)
    pub const BLUE: &str = "\x1b[34m";
    /// Cyan (hint)
    pub const CYAN: &str = "\x1b[36m";
    /// Red (error)
    pub const RED: &str = "\x1b[31m";
    /// Magenta (special)
    pub const MAGENTA: &str = "\x1b[35m";
}

/// Message type for different kinds of output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    /// Success message (green checkmark)
    Success,
    /// Error message (red X)
    Error,
    /// Warning message (yellow exclamation)
    Warning,
    /// Info message (blue i)
    Info,
    /// Tip message (cyan lightbulb)
    Tip,
    /// Hint message (cyan arrow)
    Hint,
    /// Progress message (blue arrow)
    Progress,
}

impl MessageType {
    /// Get the icon for this message type
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Success => "✓",
            Self::Error => "✗",
            Self::Warning => "⚠",
            Self::Info => "ℹ",
            Self::Tip => "💡",
            Self::Hint => "→",
            Self::Progress => "⏵",
        }
    }

    /// Get the color for this message type
    pub fn color(&self) -> &'static str {
        match self {
            Self::Success => colors::GREEN,
            Self::Error => colors::RED,
            Self::Warning => colors::YELLOW,
            Self::Info => colors::BLUE,
            Self::Tip => colors::CYAN,
            Self::Hint => colors::CYAN,
            Self::Progress => colors::BLUE,
        }
    }

    /// Get the label for this message type
    pub fn label(&self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::Error => "Error",
            Self::Warning => "Warning",
            Self::Info => "Info",
            Self::Tip => "Tip",
            Self::Hint => "Hint",
            Self::Progress => "Progress",
        }
    }
}

/// A formatted message with type, color, and content
pub struct Message {
    /// Type of message
    pub msg_type: MessageType,
    /// Message content
    pub content: String,
}

impl Message {
    /// Create a new message
    pub fn new(msg_type: MessageType, content: impl Into<String>) -> Self {
        Self {
            msg_type,
            content: content.into(),
        }
    }

    /// Create a success message
    pub fn success(content: impl Into<String>) -> Self {
        Self::new(MessageType::Success, content)
    }

    /// Create an error message
    pub fn error(content: impl Into<String>) -> Self {
        Self::new(MessageType::Error, content)
    }

    /// Create a warning message
    pub fn warning(content: impl Into<String>) -> Self {
        Self::new(MessageType::Warning, content)
    }

    /// Create an info message
    pub fn info(content: impl Into<String>) -> Self {
        Self::new(MessageType::Info, content)
    }

    /// Create a tip message
    pub fn tip(content: impl Into<String>) -> Self {
        Self::new(MessageType::Tip, content)
    }

    /// Create a hint message
    pub fn hint(content: impl Into<String>) -> Self {
        Self::new(MessageType::Hint, content)
    }

    /// Create a progress message
    pub fn progress(content: impl Into<String>) -> Self {
        Self::new(MessageType::Progress, content)
    }

    /// Print the message to stdout
    pub fn print(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{} {}{}{}",
            colors::BOLD,
            self.msg_type.color(),
            self.msg_type.icon(),
            colors::RESET,
            self.content,
            colors::RESET
        )
    }
}

/// Helper functions for common output patterns
pub mod helpers {
    use super::*;

    /// Print a success message
    pub fn success(message: impl Into<String>) {
        Message::success(message).print();
    }

    /// Print an error message
    pub fn error(message: impl Into<String>) {
        Message::error(message).print();
    }

    /// Print a warning message
    pub fn warning(message: impl Into<String>) {
        Message::warning(message).print();
    }

    /// Print an info message
    pub fn info(message: impl Into<String>) {
        Message::info(message).print();
    }

    /// Print a tip message
    pub fn tip(message: impl Into<String>) {
        Message::tip(message).print();
    }

    /// Print a hint message
    pub fn hint(message: impl Into<String>) {
        Message::hint(message).print();
    }

    /// Print a progress message
    pub fn progress(message: impl Into<String>) {
        Message::progress(message).print();
    }

    /// Print a section header
    pub fn section(title: impl Into<String>) {
        let title = title.into();
        println!("\n{}{}{}{}", colors::BOLD, colors::CYAN, title, colors::RESET);
        println!("{}", "─".repeat(title.len()));
    }
}

/// Common tips for xcargo users
pub mod tips {
    /// Tip about installing targets
    pub const INSTALL_TARGET: &str =
        "Use 'xcargo target add <triple>' to install a new target";

    /// Tip about checking installed targets
    pub const LIST_TARGETS: &str =
        "Use 'xcargo target list' to see all available targets";

    /// Tip about configuration
    pub const CONFIG_FILE: &str =
        "Create an xcargo.toml file to customize build behavior";

    /// Tip about parallel builds
    pub const PARALLEL_BUILDS: &str =
        "Enable parallel builds in xcargo.toml with 'parallel = true' for faster builds";

    /// Tip about caching
    pub const BUILD_CACHE: &str =
        "xcargo caches builds by default. Use '--no-cache' to force a clean build";

    /// Tip about container builds
    pub const CONTAINER_BUILDS: &str =
        "xcargo uses containers only when necessary. Set 'force_container = true' to always use containers";

    /// Tip about native builds
    pub const NATIVE_BUILDS: &str =
        "Native builds are 2-3x faster than container builds when possible";

    /// Tip about profiles
    pub const BUILD_PROFILES: &str =
        "Define custom build profiles in xcargo.toml for different scenarios (CI, release, etc.)";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_types() {
        assert_eq!(MessageType::Success.icon(), "✓");
        assert_eq!(MessageType::Error.icon(), "✗");
        assert_eq!(MessageType::Warning.icon(), "⚠");
        assert_eq!(MessageType::Info.icon(), "ℹ");
        assert_eq!(MessageType::Tip.icon(), "💡");
        assert_eq!(MessageType::Hint.icon(), "→");
    }

    #[test]
    fn test_message_creation() {
        let msg = Message::success("Build completed");
        assert_eq!(msg.msg_type, MessageType::Success);
        assert_eq!(msg.content, "Build completed");

        let msg = Message::tip("Use --help for more options");
        assert_eq!(msg.msg_type, MessageType::Tip);
    }

    #[test]
    fn test_message_display() {
        let msg = Message::info("Testing message");
        let output = format!("{}", msg);
        assert!(output.contains("Testing message"));
    }
}
