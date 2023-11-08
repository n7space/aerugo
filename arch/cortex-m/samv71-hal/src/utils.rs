//! Utility functions that provide abstraction over common HAL operations.

use paste::paste;

/// Waits until provided functor returns `true`.
///
/// # Parameters
/// * `check_function` - Functor that returns `true` when operation finishes, and `false` otherwise.
/// * `max_checks_amount` - Maximum amount of `functor` executions.
///
/// # Returns
/// `Some(u32)`, with amount of checks left before "timeout", or `None` if maximum checks amount
/// has been reached.
pub fn wait_until<F>(check_function: F, max_checks_amount: u32) -> Option<u32>
where
    F: Fn() -> bool,
{
    for wasted_cycles in 0..max_checks_amount {
        if check_function() {
            return Some(max_checks_amount - wasted_cycles);
        }
    }

    None
}

/// Macro creating a generic bounded value type, which allows for storing a value of specific type
/// in specified range.
///
/// You'd usually want to create a type alias that uses this structure.
macro_rules! generic_bounded_value {
    ($underlying_type:ty) => {
        paste! {
            #[doc = "Generic bounded value that uses `" $underlying_type "` as underlying type. "]
            #[doc = "Provided range is inclusive."]
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub struct [<Bounded$underlying_type:upper>]<const LOW: $underlying_type, const HIGH: $underlying_type>($underlying_type);

            impl<const LOW: $underlying_type, const HIGH: $underlying_type> [<Bounded$underlying_type:upper>]<{ LOW }, { HIGH }> {
                #[doc = "Lowest value that an instance of this struct can have."]
                pub const LOW: $underlying_type = LOW;
                #[doc = "Highest value that an instance of this struct can have."]
                pub const HIGH: $underlying_type = HIGH;

                #[doc = "Creates a new bounded value. Returns `None` if the value is out of range."]
                pub const fn new(value: $underlying_type) -> Option<Self> {
                    if value >= Self::LOW && value <= Self::HIGH {
                        Some(Self(value))
                    } else {
                        None
                    }
                }

                #[doc = "Creates a new bounded value. If `value` is out of range, it will be saturated "]
                #[doc = "to `LOW` or `HIGH`, depending whether it's too small or large."]
                pub const fn new_saturated(value: $underlying_type) -> Self {
                    Self(match value {
                        value if value < Self::LOW => Self::LOW,
                        value if value > Self::HIGH => Self::HIGH,
                        value => value
                    })
                }

                #[doc = "Returns the stored value. A `const` alternative to `Deref` trait."]
                pub const fn get(&self) -> $underlying_type {
                    self.0
                }
            }

            impl<const LOW: $underlying_type, const HIGH: $underlying_type> core::ops::Deref for [<Bounded$underlying_type:upper>]<{ LOW }, { HIGH }> {
                type Target = $underlying_type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    };
}

generic_bounded_value!(u8);
generic_bounded_value!(u16);
generic_bounded_value!(u32);
generic_bounded_value!(u64);
generic_bounded_value!(usize);

generic_bounded_value!(i8);
generic_bounded_value!(i16);
generic_bounded_value!(i32);
generic_bounded_value!(i64);
generic_bounded_value!(isize);
