/// calc from bruto
pub fn calc_bruto(value: f64, percentage: u8) -> f64 {
    let p = f64::from(percentage); // u8 to prevent negatives
    value / (100.0 + p) * p
}

pub fn calc_netto(value: f64, percentage: u8) -> f64 {
    let p = f64::from(percentage); // u8 to prevent negatives
    dbg!(p);
    value * p / 100.0
}
