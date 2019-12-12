
pub fn factoshis_to_fct(factoshis: usize) -> f64 {
  factoshis as f64 / 100_000_000f64
} 

pub fn fct_to_factoshis(factoids: f64) -> f64 {
  factoids * 100_000_000f64
}
