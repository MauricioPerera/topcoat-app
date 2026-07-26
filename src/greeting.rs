//! Greeting text used by the `/greet/{name}` page.

/// Builds the greeting shown for a (possibly empty) URL name segment.
///
/// Stub: implemented under `knowledge/contracts/format-greeting.md`.
pub fn format_greeting(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "Hello, World!".to_string();
    }
    let mut chars = trimmed.chars();
    let first = chars.next().expect("trimmed is non-empty");
    let capitalized: String = first.to_uppercase().chain(chars).collect();
    format!("Hello, {capitalized}!")
}
