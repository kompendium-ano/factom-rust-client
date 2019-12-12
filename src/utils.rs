//! General purpose helper functions

/// Converts Factoshis to Factoids
pub fn factoshis_to_fct(factoshis: usize) -> f64 {
  factoshis as f64 / 100_000_000f64
} 

/// Converts Factoids to Factoshis
pub fn fct_to_factoshis(factoids: f64) -> f64 {
  factoids * 100_000_000f64
}
