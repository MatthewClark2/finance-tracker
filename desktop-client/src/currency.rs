use std::fmt::Display;
use std::ops::{Add, Sub};

/// US Currency supporting values ranging from -92,233,720,368,547,758.08 to 92,233,720,368,547,758.07.
pub struct USD {
    total_cents: i64,
    dollars: i64,
    cents: u8,
}

fn sign(num: i64) -> i64 {
    if num < 0 {
        -1
    } else {
        1
    }
}

impl USD {
    /// Unchecked function for creating new dollar amounts.
    /// This function will panic for total currency amounts outside of the range
    /// -2^63 / 100 and 2^63 / 100.
    pub fn new(dollars: i64, cents: usize) -> Self {
        let (carry, remaining_cents) = (cents / 100, cents % 100);
        let carry: i64 = carry.try_into().unwrap();
        let sign = sign(dollars);

        let carried_dollars = sign * carry;

        let signed_cents: i64 = remaining_cents.try_into().unwrap();
        let signed_cents = signed_cents * sign;

        let dollars = dollars + carried_dollars;

        let total_cents = dollars * 100;
        let total_cents = total_cents + signed_cents;

        Self::from_cents(total_cents)
    }

    fn from_cents(total_cents: i64) -> Self {
        let cents = (total_cents % 100).abs();
        // We know the value is less than 100, so it's safe to just unwrap.
        let display_cents: u8 = cents.try_into().unwrap();
        Self {
            total_cents,
            cents: display_cents,
            dollars: total_cents / 100,
        }
    }

    fn _add(&self, other: &Self) -> Self {
        Self::from_cents(self.total_cents + other.total_cents)
    }

    fn _sub(&self, other: &Self) -> Self {
        let inversion = Self::from_cents(other.total_cents * -1);
        self.add(&inversion)
    }
}

impl Display for USD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.dollars < 0 { "-" } else { "" };
        write!(f, "{}${}.{:02}", sign, self.dollars.abs(), self.cents)
    }
}

impl Add<&USD> for &USD {
    type Output = USD;

    fn add(self, other: &USD) -> USD {
        self._add(other)
    }
}

impl Add<USD> for &USD {
    type Output = USD;

    fn add(self, other: USD) -> USD {
        self._add(&other)
    }
}

impl Add<&USD> for USD {
    type Output = USD;

    fn add(self, other: &USD) -> USD {
        self._add(other)
    }
}

impl Add<USD> for USD {
    type Output = USD;

    fn add(self, other: USD) -> USD {
        self._add(&other)
    }
}

impl Sub<&USD> for &USD {
    type Output = USD;

    fn sub(self, other: &USD) -> USD {
        self._sub(other)
    }
}

impl Sub<USD> for &USD {
    type Output = USD;

    fn sub(self, other: USD) -> USD {
        self._sub(&other)
    }
}

impl Sub<&USD> for USD {
    type Output = USD;

    fn sub(self, other: &USD) -> USD {
        self._sub(other)
    }
}

impl Sub<USD> for USD {
    type Output = USD;

    fn sub(self, other: USD) -> USD {
        self._sub(&other)
    }
}

#[cfg(test)]
mod usd_tests {
    use super::*;

    #[test]
    fn new_should_set_correct_fields() {
        let c = USD::new(22, 75);
        assert_eq!(22, c.dollars);
        assert_eq!(75, c.cents);
    }

    #[test]
    fn can_create_negative_currency() {
        let c = USD::new(-8, 96);
        assert_eq!(15 - 22 - 1, c.dollars);
        assert_eq!(96, c.cents);
    }

    #[test]
    fn cents_over_100_should_roll_over_for_positive_inputs() {
        let c = USD::new(1, 1015);
        assert_eq!(c.dollars, 11);
        assert_eq!(c.cents, 15);
    }

    #[test]
    fn cents_over_100_should_roll_over_for_negative_inputs() {
        let c = USD::new(-1, 115);
        assert_eq!(c.dollars, -2);
        assert_eq!(c.cents, 15);
    }

    #[test]
    fn adding_two_values_that_carry_over_should_increase_dollar_amount() {
        let c1 = USD::new(500, 32);
        let c2 = USD::new(31, 99);
        let c3 = c1.add(&c2);
        assert_eq!(532, c3.dollars);
        assert_eq!(31, c3.cents);
    }

    #[test]
    fn add_positive_and_negative_currency_with_carry() {
        let c1 = USD::new(15, 95);
        let c2 = USD::new(-22, 99);
        let c3 = c1.add(&c2);
        assert_eq!(-7, c3.dollars);
        assert_eq!(4, c3.cents);
    }

    #[test]
    fn can_create_0_value_currency() {
        let c = USD::new(0, 0);
        assert_eq!(0, c.dollars);
        assert_eq!(0, c.cents);
    }

    #[test]
    fn can_add_negative_currency_amounts() {
        let c1 = USD::new(-1, 50);
        let c2 = USD::new(-1, 50);
        let c3 = c1.add(&c2);
        assert_eq!(-3, c3.dollars);
        assert_eq!(0, c3.cents);
    }

    #[test]
    fn add_is_commutative() {
        let c1 = USD::new(1, 50);
        let c2 = USD::new(2, 10);
        let left_sum = &c1 + &c2;
        let right_sum = &c2 + &c1;
        assert_eq!(left_sum.dollars, right_sum.dollars);
        assert_eq!(left_sum.cents, right_sum.cents);
        assert_eq!(3, left_sum.dollars);
        assert_eq!(60, left_sum.cents);
    }

    #[test]
    fn add_0_returns_same_value() {
        let c1 = USD::new(-1, 50);
        let c2 = USD::new(0, 0);
        let c3 = Add::add(&c1, &c2);
        assert_eq!(c1.dollars, c3.dollars);
        assert_eq!(c1.cents, c3.cents);
    }

    #[test]
    fn subtract_positive_from_0() {
        let c1 = USD::new(0, 0);
        let c2 = USD::new(15, 31);
        let c3 = c1.sub(&c2);
        assert_eq!(-15, c3.dollars);
        assert_eq!(31, c3.cents);
    }

    #[test]
    fn subtract_negative_from_0() {
        let c1 = USD::new(0, 0);
        let c2 = USD::new(-15, 31);
        let c3 = c1.sub(&c2);
        assert_eq!(15, c3.dollars);
        assert_eq!(31, c3.cents);
    }

    #[test]
    fn subtract_0() {
        let c1 = USD::new(-1, 50);
        let c2 = USD::new(0, 0);
        let c3 = &c1 + &c2;
        assert_eq!(c1.dollars, c3.dollars);
        assert_eq!(c1.cents, c3.cents);
    }

    #[test]
    fn subtract_with_carry() {
        let c1 = USD::new(15, 29);
        let c2 = USD::new(14, 31);
        let c3 = c1.sub(&c2);
        assert_eq!(0, c3.dollars);
        assert_eq!(98, c3.cents);
    }

    #[test]
    fn subtract_negative_with_carry() {
        let c1 = USD::new(9, 83);
        let c2 = USD::new(-5, 17);
        let c3 = c1.sub(&c2);
        assert_eq!(15, c3.dollars);
        assert_eq!(0, c3.cents);
    }

    #[test]
    fn does_not_panic_for_u64_div_100() {
        USD::new(i64::MAX / 100, (i64::MAX % 100).try_into().unwrap());
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn should_panic_above_upper_limit() {
        let cents: usize = (i64::MAX % 100).abs().try_into().unwrap();
        USD::new(i64::MAX / 100, cents + 1);
    }

    #[test]
    fn does_not_panic_for_negative_i64_div_100() {
        USD::new(i64::MIN / 100, (i64::MIN % 100).abs().try_into().unwrap());
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn should_panic_at_lower_limit() {
        let cents: usize = (i64::MIN % 100).abs().try_into().unwrap();
        USD::new(i64::MIN / 100, cents + 1);
    }
}

#[cfg(test)]
mod usd_display_tests {
    use super::*;

    #[test]
    fn should_print_0_value() {
        let c = USD::new(0, 0);
        assert_eq!("$0.00", c.to_string());
    }

    #[test]
    fn should_print_positive_with_single_cents() {
        let c = USD::new(3_705, 7);
        assert_eq!("$3705.07", c.to_string());
    }

    #[test]
    fn should_print_negative_with_single_cents() {
        let c = USD::new(-10_513_012, 3);
        assert_eq!("-$10513012.03", c.to_string());
    }

    #[test]
    fn should_print_positive_with_many_cents() {
        let c = USD::new(51, 82);
        assert_eq!("$51.82", c.to_string());
    }

    #[test]
    fn should_print_negative_with_many_cents() {
        let c = USD::new(-300, 16);
        assert_eq!("-$300.16", c.to_string());
    }
}
