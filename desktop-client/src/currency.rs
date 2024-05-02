use rug::Integer;
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct USD {
    total_cents: Integer,
}

impl USD {
    pub fn new(dollars: i64, cents: usize) -> Self {
        let carry = cents / 100;
        let remaining_cents: i64 = (cents % 100).try_into().unwrap();

        let sign: i64 = if dollars < 0 { -1 } else { 1 };

        let carry: i64 = carry.try_into().unwrap();
        let carry = carry * sign;

        let dollars = Integer::from(dollars);
        let dollars = dollars + carry;
        let cents = sign * remaining_cents;

        Self::from(dollars * 100 + cents)
    }

    pub fn dollars(&self) -> Integer {
        self.total_cents.clone() / 100
    }

    pub fn cents(&self) -> u32 {
        let euclid_remainder = self.total_cents.mod_u(100);
        if self.total_cents < 0 && euclid_remainder != 0 {
            100 - euclid_remainder
        } else {
            euclid_remainder
        }
    }

    fn _add(&self, other: &Self) -> Self {
        let result = &self.total_cents + &other.total_cents;
        Self::from(Integer::from(result))
    }

    fn _sub(&self, other: &Self) -> Self {
        let result = &self.total_cents - &other.total_cents;
        Self::from(Integer::from(result))
    }
}

impl From<Integer> for USD {
    fn from(total_cents: Integer) -> Self {
        Self { total_cents }
    }
}

impl Display for USD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.dollars() < 0 { "-" } else { "" };
        write!(f, "{}${}.{:02}", sign, self.dollars().abs(), self.cents())
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
mod usd_creation_tests {
    use super::*;

    #[test]
    fn new_should_set_correct_fields() {
        let c = USD::new(22, 75);
        assert_eq!(22, c.dollars());
        assert_eq!(75, c.cents());
    }

    #[test]
    fn can_create_negative_currency() {
        let c = USD::new(-8, 96);
        assert_eq!(15 - 22 - 1, c.dollars());
        assert_eq!(96, c.cents());
    }

    #[test]
    fn cents_over_100_should_roll_over_for_positive_inputs() {
        let c = USD::new(1, 1015);
        assert_eq!(c.dollars(), 11);
        assert_eq!(c.cents(), 15);
    }

    #[test]
    fn cents_over_100_should_roll_over_for_negative_inputs() {
        let c = USD::new(-1, 115);
        assert_eq!(c.dollars(), -2);
        assert_eq!(c.cents(), 15);
    }

    #[test]
    fn adding_two_values_that_carry_over_should_increase_dollar_amount() {
        let c1 = USD::new(500, 32);
        let c2 = USD::new(31, 99);
        let c3 = c1.add(&c2);
        assert_eq!(532, c3.dollars());
        assert_eq!(31, c3.cents());
    }

    #[test]
    fn add_positive_and_negative_currency_with_carry() {
        let c1 = USD::new(15, 95);
        let c2 = USD::new(-22, 99);
        let c3 = c1.add(&c2);
        assert_eq!(-7, c3.dollars());
        assert_eq!(4, c3.cents());
    }

    #[test]
    fn can_create_0_value_currency() {
        let c = USD::new(0, 0);
        assert_eq!(0, c.dollars());
        assert_eq!(0, c.cents());
    }

    #[test]
    fn does_not_panic_for_huge_positive_values() {
        USD::new(i64::MAX, 275);
    }

    #[test]
    fn does_not_panic_for_huge_negative_values() {
        USD::new(i64::MIN, 399);
    }
}

#[cfg(test)]
mod usd_ops_tests {
    use super::*;

    #[test]
    fn can_add_negative_currency_amounts() {
        let c1 = USD::new(-1, 50);
        let c2 = USD::new(-1, 50);
        let c3 = c1.add(&c2);
        assert_eq!(-3, c3.dollars());
        assert_eq!(0, c3.cents());
    }

    #[test]
    fn add_is_commutative() {
        let c1 = USD::new(1, 50);
        let c2 = USD::new(2, 10);
        let left_sum = &c1 + &c2;
        let right_sum = &c2 + &c1;
        assert_eq!(left_sum.dollars(), right_sum.dollars());
        assert_eq!(left_sum.cents(), right_sum.cents());
        assert_eq!(3, left_sum.dollars());
        assert_eq!(60, left_sum.cents());
    }

    #[test]
    fn add_0_returns_same_value() {
        let c1 = USD::new(-1, 50);
        let c2 = USD::new(0, 0);
        let c3 = Add::add(&c1, &c2);
        assert_eq!(c1.dollars(), c3.dollars());
        assert_eq!(c1.cents(), c3.cents());
    }

    #[test]
    fn subtract_positive_from_0() {
        let c1 = USD::new(0, 0);
        let c2 = USD::new(15, 31);
        let c3 = c1.sub(&c2);
        assert_eq!(-15, c3.dollars());
        assert_eq!(31, c3.cents());
    }

    #[test]
    fn subtract_negative_from_0() {
        let c1 = USD::new(0, 0);
        let c2 = USD::new(-15, 31);
        let c3 = c1.sub(&c2);
        assert_eq!(15, c3.dollars());
        assert_eq!(31, c3.cents());
    }

    #[test]
    fn subtract_0() {
        let c1 = USD::new(-1, 50);
        let c2 = USD::new(0, 0);
        let c3 = &c1 + &c2;
        assert_eq!(c1.dollars(), c3.dollars());
        assert_eq!(c1.cents(), c3.cents());
    }

    #[test]
    fn subtract_with_carry() {
        let c1 = USD::new(15, 29);
        let c2 = USD::new(14, 31);
        let c3 = c1.sub(&c2);
        assert_eq!(0, c3.dollars());
        assert_eq!(98, c3.cents());
    }

    #[test]
    fn subtract_negative_with_carry() {
        let c1 = USD::new(9, 83);
        let c2 = USD::new(-5, 17);
        let c3 = c1.sub(&c2);
        assert_eq!(15, c3.dollars());
        assert_eq!(0, c3.cents());
    }
}

#[cfg(test)]
mod usd_conversion_tests {
    use super::*;

    #[test]
    fn should_convert_from_0i() {
        let c: USD = USD::from(Integer::new());
        assert_eq!(c.dollars(), 0);
        assert_eq!(c.cents(), 0);
    }

    #[test]
    fn should_convert_from_trivial_positive_integer() {
        let c: USD = USD::from(Integer::from(255_73));
        assert_eq!(c.dollars(), 255);
        assert_eq!(c.cents(), 73);
    }

    #[test]
    fn should_convert_from_trivial_negative_integer() {
        let c: USD = USD::from(Integer::from(-255_73));
        assert_eq!(c.dollars(), -255);
        assert_eq!(c.cents(), 73);
    }

    #[test]
    fn should_convert_from_massive_integer() {
        let value = u128::MAX;
        let cents: u32 = (value % 100).try_into().unwrap();
        let c: USD = USD::from(Integer::from(u128::MAX));
        assert_eq!(c.dollars(), u128::MAX / 100);
        assert_eq!(c.cents(), cents);
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
    fn should_print_multiple_of_10_cents() {
        let c = USD::new(15, 30);
        assert_eq!("$15.30", c.to_string());
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
