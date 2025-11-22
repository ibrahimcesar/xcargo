//! User-facing output formatting and helpful messages
//!
//! This module provides utilities for displaying information, tips, hints,
//! and progress to users in a delightful and informative way.

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fmt;
use std::time::{Duration, Instant};

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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
        println!("{self}");
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
    use super::{colors, Message};

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
        println!(
            "\n{}{}{}{}",
            colors::BOLD,
            colors::CYAN,
            title,
            colors::RESET
        );
        println!("{}", "─".repeat(title.len()));
    }
}

/// Progress bar utilities for build operations
pub mod progress {
    use super::{colors, Duration, Instant, MultiProgress, ProgressBar, ProgressStyle};

    /// A timed build progress tracker
    pub struct BuildProgress {
        bar: ProgressBar,
        start_time: Instant,
        target: String,
    }

    impl BuildProgress {
        /// Create a new build progress spinner
        #[must_use]
        pub fn new(target: &str, operation: &str) -> Self {
            let bar = ProgressBar::new_spinner();
            bar.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template(&format!(
                        "{{spinner:.cyan}} {operation} {{msg:.bold}} [{{elapsed_precise}}]"
                    ))
                    .unwrap(),
            );
            bar.set_message(target.to_string());
            bar.enable_steady_tick(Duration::from_millis(80));

            Self {
                bar,
                start_time: Instant::now(),
                target: target.to_string(),
            }
        }

        /// Create a build progress for compiling
        #[must_use]
        pub fn compiling(target: &str) -> Self {
            Self::new(target, "Compiling")
        }

        /// Create a build progress for checking
        #[must_use]
        pub fn checking(target: &str) -> Self {
            Self::new(target, "Checking")
        }

        /// Create a build progress for testing
        #[must_use]
        pub fn testing(target: &str) -> Self {
            Self::new(target, "Testing")
        }

        /// Update the message
        pub fn set_message(&self, msg: &str) {
            self.bar.set_message(msg.to_string());
        }

        /// Mark as finished with success
        pub fn finish_success(&self) {
            let elapsed = self.start_time.elapsed();
            self.bar.finish_with_message(format!(
                "{}{}{} {} {}({}){}",
                colors::GREEN,
                "✓",
                colors::RESET,
                self.target,
                colors::DIM,
                format_duration(elapsed),
                colors::RESET
            ));
        }

        /// Mark as finished with error
        pub fn finish_error(&self, error: &str) {
            let elapsed = self.start_time.elapsed();
            self.bar.finish_with_message(format!(
                "{}{}{} {} - {} {}({}){}",
                colors::RED,
                "✗",
                colors::RESET,
                self.target,
                error,
                colors::DIM,
                format_duration(elapsed),
                colors::RESET
            ));
        }

        /// Get elapsed duration
        #[must_use]
        pub fn elapsed(&self) -> Duration {
            self.start_time.elapsed()
        }
    }

    /// Multi-target progress tracker for parallel builds
    pub struct MultiTargetProgress {
        multi: MultiProgress,
        start_time: Instant,
    }

    impl MultiTargetProgress {
        /// Create a new multi-target progress tracker
        #[must_use]
        pub fn new() -> Self {
            Self {
                multi: MultiProgress::new(),
                start_time: Instant::now(),
            }
        }

        /// Add a target progress bar
        #[must_use]
        pub fn add_target(&self, target: &str, operation: &str) -> ProgressBar {
            let bar = self.multi.add(ProgressBar::new_spinner());
            bar.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template(&format!(
                        "  {{spinner:.cyan}} {operation} {{msg:.bold}} [{{elapsed_precise}}]"
                    ))
                    .unwrap(),
            );
            bar.set_message(target.to_string());
            bar.enable_steady_tick(Duration::from_millis(80));
            bar
        }

        /// Get total elapsed time
        #[must_use]
        pub fn elapsed(&self) -> Duration {
            self.start_time.elapsed()
        }

        /// Print summary
        pub fn finish_summary(&self, successes: usize, failures: usize) {
            let elapsed = self.elapsed();
            println!();
            if failures == 0 {
                println!(
                    "{}{}✓{} All {} targets completed in {}",
                    colors::BOLD,
                    colors::GREEN,
                    colors::RESET,
                    successes,
                    format_duration(elapsed)
                );
            } else {
                println!(
                    "{}{}⚠{} {} succeeded, {} failed in {}",
                    colors::BOLD,
                    colors::YELLOW,
                    colors::RESET,
                    successes,
                    failures,
                    format_duration(elapsed)
                );
            }
        }
    }

    impl Default for MultiTargetProgress {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Format a duration in a human-readable way
    #[must_use]
    pub fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs();
        let millis = duration.subsec_millis();

        if secs >= 60 {
            let mins = secs / 60;
            let secs = secs % 60;
            format!("{mins}m {secs:02}s")
        } else if secs > 0 {
            format!("{}.{:02}s", secs, millis / 10)
        } else {
            format!("{millis}ms")
        }
    }

    /// Simple timer for tracking operation duration
    pub struct Timer {
        start: Instant,
        label: String,
    }

    impl Timer {
        /// Start a new timer
        #[must_use]
        pub fn start(label: &str) -> Self {
            Self {
                start: Instant::now(),
                label: label.to_string(),
            }
        }

        /// Get elapsed duration
        #[must_use]
        pub fn elapsed(&self) -> Duration {
            self.start.elapsed()
        }

        /// Print elapsed time
        pub fn print_elapsed(&self) {
            println!(
                "{}{}⏱{} {} completed in {}{}{}",
                colors::BOLD,
                colors::CYAN,
                colors::RESET,
                self.label,
                colors::DIM,
                format_duration(self.elapsed()),
                colors::RESET
            );
        }
    }
}

/// Common tips for xcargo users
pub mod tips {
    /// Tip about installing targets
    pub const INSTALL_TARGET: &str = "Use 'xcargo target add <triple>' to install a new target";

    /// Tip about checking installed targets
    pub const LIST_TARGETS: &str = "Use 'xcargo target list' to see all available targets";

    /// Tip about configuration
    pub const CONFIG_FILE: &str = "Create an xcargo.toml file to customize build behavior";

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
        let output = format!("{msg}");
        assert!(output.contains("Testing message"));
    }
}
