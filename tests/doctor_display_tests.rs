// Tests for src/doctor/report.rs display formatting
// Focuses on uncovered display branches and formatting logic

use xcargo::doctor::{CheckResult, CheckStatus, DoctorReport};

// ============================================================================
// Display Check Formatting - All Status Types
// ============================================================================

#[test]
fn test_display_pass_check() {
    let mut report = DoctorReport::new();
    report.add_check(CheckResult::pass("Pass Check", "Everything is OK"));

    // Should not panic
    report.display();
}

#[test]
fn test_display_warning_check() {
    let mut report = DoctorReport::new();
    report.add_check(CheckResult::warning(
        "Warning Check",
        "Minor issue detected",
        "Consider fixing this",
    ));

    // Should display warning icon and yellow formatting
    report.display();
}

#[test]
fn test_display_fail_check() {
    let mut report = DoctorReport::new();
    report.add_check(CheckResult::fail(
        "Fail Check",
        "Something failed",
        "Fix this issue",
    ));

    // Should display fail icon and red formatting
    report.display();
}

#[test]
fn test_display_critical_check() {
    let mut report = DoctorReport::new();
    report.add_check(CheckResult::critical(
        "Critical Check",
        "Critical failure",
        "Urgent fix required",
    ));

    // Should display critical icon and bright red formatting
    report.display();
}

// ============================================================================
// Summary Display with Different Status Combinations
// ============================================================================

#[test]
fn test_summary_display_all_passed() {
    let mut report = DoctorReport::new();

    for i in 0..5 {
        report.add_check(CheckResult::pass(&format!("Check {}", i), "OK"));
    }

    // Should display success message
    report.display();

    let summary = report.summary();
    assert_eq!(summary.passed, 5);
    assert_eq!(summary.warnings, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.critical, 0);
}

#[test]
fn test_summary_display_with_warnings_only() {
    let mut report = DoctorReport::new();

    for i in 0..3 {
        report.add_check(CheckResult::warning(
            &format!("Warning {}", i),
            "Minor issue",
            "Fix this",
        ));
    }

    // Should display "optional features unavailable" message
    report.display();

    let summary = report.summary();
    assert_eq!(summary.warnings, 3);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.critical, 0);
}

#[test]
fn test_summary_display_with_failures_only() {
    let mut report = DoctorReport::new();

    for i in 0..2 {
        report.add_check(CheckResult::fail(
            &format!("Fail {}", i),
            "Failed",
            "Fix",
        ));
    }

    // Should display "some features may not work" message
    report.display();

    let summary = report.summary();
    assert_eq!(summary.failed, 2);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.warnings, 0);
    assert_eq!(summary.critical, 0);
}

#[test]
fn test_summary_display_with_critical_failures() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::critical(
        "Critical Issue",
        "System broken",
        "Reinstall",
    ));

    // Should display "xcargo may not function correctly" message
    report.display();

    let summary = report.summary();
    assert_eq!(summary.critical, 1);
}

#[test]
fn test_summary_display_mixed_with_passed_and_warnings() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::pass("Check 1", "OK"));
    report.add_check(CheckResult::pass("Check 2", "OK"));
    report.add_check(CheckResult::warning("Check 3", "Warning", "Fix"));

    // Should show "optional features unavailable" (warnings take precedence)
    report.display();

    let summary = report.summary();
    assert_eq!(summary.passed, 2);
    assert_eq!(summary.warnings, 1);
}

#[test]
fn test_summary_display_mixed_with_passed_and_failed() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::pass("Check 1", "OK"));
    report.add_check(CheckResult::fail("Check 2", "Failed", "Fix"));

    // Should show "some features may not work" (failures take precedence)
    report.display();

    let summary = report.summary();
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 1);
}

#[test]
fn test_summary_display_mixed_with_critical() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::pass("Check 1", "OK"));
    report.add_check(CheckResult::warning("Check 2", "Warn", "Fix"));
    report.add_check(CheckResult::fail("Check 3", "Fail", "Fix"));
    report.add_check(CheckResult::critical("Check 4", "Critical", "Fix"));

    // Should show critical message (highest severity)
    report.display();

    assert!(report.has_critical_failures());
}

// ============================================================================
// Check Result Helper Methods
// ============================================================================

#[test]
fn test_check_result_pass_helper() {
    let check = CheckResult::pass("Test", "Message");

    assert_eq!(check.name, "Test");
    assert_eq!(check.status, CheckStatus::Pass);
    assert_eq!(check.message, "Message");
    assert!(check.suggestion.is_none());
}

#[test]
fn test_check_result_warning_helper() {
    let check = CheckResult::warning("Test", "Message", "Suggestion");

    assert_eq!(check.name, "Test");
    assert_eq!(check.status, CheckStatus::Warning);
    assert_eq!(check.message, "Message");
    assert_eq!(check.suggestion, Some("Suggestion".to_string()));
}

#[test]
fn test_check_result_fail_helper() {
    let check = CheckResult::fail("Test", "Message", "Suggestion");

    assert_eq!(check.name, "Test");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_eq!(check.message, "Message");
    assert_eq!(check.suggestion, Some("Suggestion".to_string()));
}

#[test]
fn test_check_result_critical_helper() {
    let check = CheckResult::critical("Test", "Message", "Suggestion");

    assert_eq!(check.name, "Test");
    assert_eq!(check.status, CheckStatus::Critical);
    assert_eq!(check.message, "Message");
    assert_eq!(check.suggestion, Some("Suggestion".to_string()));
}

// ============================================================================
// Multiple Checks of Each Type for Display Coverage
// ============================================================================

#[test]
fn test_display_multiple_warnings() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::warning("Warn 1", "Issue 1", "Fix 1"));
    report.add_check(CheckResult::warning("Warn 2", "Issue 2", "Fix 2"));
    report.add_check(CheckResult::warning("Warn 3", "Issue 3", "Fix 3"));

    report.display();

    let summary = report.summary();
    assert_eq!(summary.warnings, 3);
}

#[test]
fn test_display_multiple_failures() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::fail("Fail 1", "Error 1", "Fix 1"));
    report.add_check(CheckResult::fail("Fail 2", "Error 2", "Fix 2"));

    report.display();

    let summary = report.summary();
    assert_eq!(summary.failed, 2);
}

#[test]
fn test_display_multiple_critical() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::critical("Crit 1", "Error 1", "Fix 1"));
    report.add_check(CheckResult::critical("Crit 2", "Error 2", "Fix 2"));
    report.add_check(CheckResult::critical("Crit 3", "Error 3", "Fix 3"));

    report.display();

    let summary = report.summary();
    assert_eq!(summary.critical, 3);
}

// ============================================================================
// Complex Report Scenarios
// ============================================================================

#[test]
fn test_comprehensive_report_with_all_statuses() {
    let mut report = DoctorReport::new();

    // Add multiple checks of each type
    report.add_check(CheckResult::pass("Rust toolchain", "✓ rustc 1.70.0 found"));
    report.add_check(CheckResult::pass("Cargo", "✓ cargo 1.70.0 found"));
    report.add_check(CheckResult::pass("Rustup", "✓ rustup 1.26.0 found"));

    report.add_check(CheckResult::warning(
        "Zig compiler",
        "Zig not found",
        "Install zig for better musl support",
    ));
    report.add_check(CheckResult::warning(
        "Docker",
        "Docker not found",
        "Install docker for container builds",
    ));

    report.add_check(CheckResult::fail(
        "MinGW",
        "MinGW toolchain not found",
        "Install mingw-w64 for Windows cross-compilation",
    ));

    report.add_check(CheckResult::critical(
        "Git",
        "Git not found",
        "Install git to work with repositories",
    ));

    report.display();

    let summary = report.summary();
    assert_eq!(summary.total, 7);
    assert_eq!(summary.passed, 3);
    assert_eq!(summary.warnings, 2);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.critical, 1);
    assert!(report.has_critical_failures());
}

#[test]
fn test_report_with_checks_without_suggestions() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Check 1".to_string(),
        status: CheckStatus::Pass,
        message: "OK".to_string(),
        suggestion: None,
    });

    report.add_check(CheckResult {
        name: "Check 2".to_string(),
        status: CheckStatus::Warning,
        message: "Warning without suggestion".to_string(),
        suggestion: None,
    });

    report.display();
}

#[test]
fn test_report_with_long_names_and_messages() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::pass(
        "Very long check name that might wrap on narrow terminals",
        "This is a very detailed message explaining what was checked and why it passed successfully",
    ));

    report.add_check(CheckResult::warning(
        "Another long check name",
        "Long warning message",
        "This is a very long suggestion that provides detailed instructions on how to fix the issue",
    ));

    report.display();
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_empty_report_display() {
    let report = DoctorReport::new();

    // Should display "all checks passed" even with no checks
    report.display();

    let summary = report.summary();
    assert_eq!(summary.total, 0);
}

#[test]
fn test_report_with_unicode_in_messages() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::pass(
        "Unicode test",
        "✓ ✗ ⚠ → 🎯 Cross-compilation test",
    ));

    report.display();
}

#[test]
fn test_report_summary_statistics() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult::pass("Pass 1", "OK"));
    report.add_check(CheckResult::pass("Pass 2", "OK"));
    report.add_check(CheckResult::pass("Pass 3", "OK"));
    report.add_check(CheckResult::warning("Warn 1", "Warn", "Fix"));
    report.add_check(CheckResult::warning("Warn 2", "Warn", "Fix"));
    report.add_check(CheckResult::fail("Fail 1", "Fail", "Fix"));
    report.add_check(CheckResult::critical("Crit 1", "Crit", "Fix"));

    let summary = report.summary();

    assert_eq!(summary.total, 7);
    assert_eq!(summary.passed, 3);
    assert_eq!(summary.warnings, 2);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.critical, 1);
}
