use crate::types::{Amount, Money};

/// calc from bruto
pub fn calc_bruto(value: Money, percentage: u8) -> Amount {
    let p =percentage as i64; // u8 to prevent negatives
    let tax = Money::new(value.cents / (100 + p) * p);
    Amount {
        tax,
        bruto: value,
        netto: value - tax,
        percentage,
    }
}

/// calc from netto
pub fn calc_netto(value: Money, percentage: u8) -> Amount {
    let p = percentage as i64; // u8 to prevent negatives
    let tax = Money::new(value.cents * p / 100);
    Amount {
        tax,
        bruto: value + tax,
        netto: value,
        percentage,
    }
}

#[cfg(test)]
mod tests {
    use crate::calc::*;

    /// assert that the formatting output is equal between `expected` and `actual`
    macro_rules! assert_feq {
        ($expected:expr, $actual:expr) => {
            assert_eq!(format!("{}", $expected), format!("{}", $actual));
        };
    }

    #[test]
    fn bruto_hoog_correct() {
        let bruto = 121.0.into();
        let expected = Amount {
            tax: 21.0.into(),
            bruto,
            netto: 100.0.into(),
            percentage: 21,
        };

        let actual = calc_bruto(bruto, 21);

        assert_eq!(expected, actual);
    }

    #[test]
    fn bruto_laag_correct() {
        let bruto = 109.0.into();
        let expected = Amount {
            tax: 9.0.into(),
            bruto,
            netto: 100.0.into(),
            percentage: 9,
        };

        let actual = calc_bruto(bruto, 9);

        assert_eq!(expected, actual);
    }

    #[test]
    fn bruto_netto_consistent() {
        let bruto = 1.50.into();
        let netto = 1.24.into();

        let actual_bruto = calc_bruto(bruto, 21);
        let actual_netto = calc_netto(netto, 21);

        assert_feq!(actual_bruto, actual_netto);
    }

    #[test]
    fn bruto_rounding_correct() {
        let bruto = 34.89.into();

        let expected = Amount {
            netto: 28.83.into(),
            bruto,
            tax: 6.06.into(),
            percentage: 21,
        };

        let actual = calc_bruto(bruto, 21);

        assert_feq!(expected, actual);
    }

    #[test]
    fn netto_rounding_correct() {
        let netto = 28.83.into();

        let expected = Amount {
            netto,
            bruto: 34.89.into(),
            tax: 6.06.into(),
            percentage: 21,
        };

        let actual = calc_netto(netto, 21);

        assert_feq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn bruto_hoog_negative() {
        let bruto = Money::from(-121.0);

        let actual = calc_bruto(bruto, 21);
        dbg!(actual);
        todo!();
    }
}
