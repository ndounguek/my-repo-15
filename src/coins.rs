//! Coin arithmetic for the change machine.

/// Whole quarters that fit in `cents`.
///
/// ```
/// use dw15::coins::count_quarters;
///
/// assert_eq!(count_quarters(87), 3);
/// ```
pub fn count_quarters(cents: u32) -> u32 {
    cents / 25
}

/// Whole dimes that fit in what is left after taking out the quarters.
///
/// ```
/// use dw15::coins::count_dimes;
///
/// assert_eq!(count_dimes(87), 1);
/// ```
pub fn count_dimes(cents: u32) -> u32 {
    (cents % 25) / 10
}

/// Cents left after taking out the quarters and the dimes.
///
/// ```
/// use dw15::coins::leftover_cents;
///
/// assert_eq!(leftover_cents(87), 2);
/// ```
pub fn leftover_cents(cents: u32) -> u32 {
    (cents % 25) % 10
}
