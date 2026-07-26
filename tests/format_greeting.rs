//! Frozen property tests for `topcoat_app::greeting::format_greeting`.
//!
//! This is the oracle for `knowledge/contracts/format-greeting.md` -- sealed by `tests_sha256`
//! in the contract's frontmatter and must not be edited by the implementer.

use topcoat_app::greeting::format_greeting;

#[test]
fn empty_name_greets_world() {
    assert_eq!(format_greeting(""), "Hello, World!");
}

#[test]
fn whitespace_only_name_greets_world() {
    assert_eq!(format_greeting("   "), "Hello, World!");
}

#[test]
fn lowercase_name_gets_capitalized() {
    assert_eq!(format_greeting("ana"), "Hello, Ana!");
}

#[test]
fn already_capitalized_name_is_unchanged() {
    assert_eq!(format_greeting("Bob"), "Hello, Bob!");
}

#[test]
fn surrounding_whitespace_is_trimmed() {
    assert_eq!(format_greeting("  bob  "), "Hello, Bob!");
}

#[test]
fn rest_of_name_after_first_char_is_left_as_is() {
    // Only the first char is uppercased; the rest is untouched (no full lowercasing).
    assert_eq!(format_greeting("mcCARTHY"), "Hello, McCARTHY!");
}

#[test]
fn multi_byte_first_char_is_handled() {
    // 'e' with an accent uppercases to 'E' with the same accent, a 2-byte UTF-8 char both ways.
    assert_eq!(format_greeting("école"), "Hello, École!");
}

#[test]
fn never_panics_on_single_char_name() {
    assert_eq!(format_greeting("a"), "Hello, A!");
}
