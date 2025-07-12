//! Provides features related to general-purpose content formatting.

use std::any;
use std::fmt::Display;

use anyhow::{Result, anyhow};
use num_traits::{PrimInt, Unsigned};

/// Gets the length in digits of a number.
///
/// # Parameters
/// - `number`: the number to be checked.
///
/// # Returns
/// The length or an error.
///
/// # Errors
/// It returns a displayable error if the required divisions by 10 causes an overflow in the number
/// type.
pub(crate) fn number_length<T>(mut number: T) -> Result<usize>
where
    T: PrimInt + Unsigned + Display,
{
    let mut length: usize = if number == T::zero() { 1 } else { 0 };
    loop {
        if number == T::zero() {
            break;
        }
        length += 1;
        number = number
            / T::from(10).ok_or_else(|| {
                anyhow!(
                    r#"number "10" used overflows expected type "{}"."#,
                    any::type_name::<T>()
                )
            })?;
    }
    Ok(length)
}
