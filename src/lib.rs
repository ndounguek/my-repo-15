//! # DW15 — Change Machine
//!
//! ```text
//! quarters=3 dimes=1 cents=2
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

pub mod coins;

/// Splits an amount of change into quarters, dimes and leftover cents.
///
/// ```
/// assert_eq!(dw15::change_summary(87), "quarters=3 dimes=1 cents=2");
/// ```
///
/// ```
/// assert_eq!(dw15::change_summary(24), "quarters=0 dimes=2 cents=4");
/// ```
pub fn change_summary(cents: u32) -> String {
    let quarters = coins::count_quaters(cents);
    let dimes = coins::count_dimes(cents);
    let leftover = coins::leftover_cents(cents);
    format!("quarters={} dimes={} cents={}", quarters, dimes, leftover)
}
