use std::fmt;

pub struct Amount {
    netto: f64,
    bruto: f64,
    tax: f64,
    percentage: u8
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "Netto: {:.2}\nBruto: {:.2}\nBtw bedrag: {:.2}\nBtw percentage: {}"
        , self.netto, self.bruto, self.tax, self.percentage)
    }
}

/// calc from bruto
pub fn calc_bruto(value: f64, percentage: u8) -> Amount {
    let p = f64::from(percentage); // u8 to prevent negatives
    let tax = value / (100.0 + p) * p;
    Amount{
        tax: tax, 
        bruto: value,
        netto: value - tax,
        percentage: percentage
    }
}

/// calc from netto
pub fn calc_netto(value: f64, percentage: u8) -> Amount {
    let p = f64::from(percentage); // u8 to prevent negatives
    let tax = value * p / 100.0;
    Amount{
        tax: tax, 
        bruto: value + tax,
        netto: value,
        percentage: percentage
    }
}
