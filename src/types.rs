use std::{fmt::{self, Display}, ops::{Add, Div, Sub}};
#[derive(Debug, PartialEq)]
pub struct Amount {
    pub netto: Money,
    pub bruto: Money,
    pub tax: Money,
    pub percentage: u8,
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(
            f,
            "Netto: {:.2}\nBruto: {:.2}\nBtw bedrag: {:.2}\nBtw percentage: {}",
            self.netto, self.bruto, self.tax, self.percentage
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Money {
    pub cents: i64,
}

impl Money {
    pub fn new(cents: i64) -> Self {
        Money{cents}
    }
}

impl From<f64> for Money {
    fn from(fractional: f64) -> Self {
        let cents = 100.0 * fractional;
        Money {
            cents: cents.round() as i64,
        }
    }
}

impl Into<f64> for Money {
    fn into(self) -> f64 {
        let cents = self.cents as f64;
        cents / 100.0
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cents)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Money::new(self.cents + rhs.cents)
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Money::new(self.cents - rhs.cents)
    }
}

impl Div for Money {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Money::new(self.cents / rhs.cents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_from_float() {
        let float = 1.83;
        let expected = Money { cents: 183 };
        let actual = Money::from(float);

        assert_eq!(expected, actual);
    }

    #[test]
    fn money_to_float() {
        let money = Money { cents: 183 };
        let expected = 1.83;
        let actual: f64 = money.into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn money_float_trucated() {
        let float = 13.3742;
        let expected = Money { cents: 1337 };
        let actual = Money::from(float);

        assert_eq!(expected, actual);
    }

    #[test]
    fn money_float_rounded() {
        let float = 13.3750;
        let expected = Money { cents: 1338 };
        let actual = Money::from(float);

        assert_eq!(expected, actual);
    }
}
