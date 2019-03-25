//! Basic helper functions
use super::*;

// Search options for the transactions function
pub enum SearchBy{
    Range(u32, u32),
    Txid(&'static str),
    Address(&'static str)
}

pub fn str_to_hex(utf8: &str) -> String {
  let strs: Vec<String> = utf8.as_bytes()
                                .iter()
                                .map(|b| format!("{:02X}", b))
                                .collect();
  strs.join("")
}

// Retrieves future, blocks until Result is returned
pub fn fetch<F, R, E>(fut: F)-> Result<R, E>
    where
        F: Send + 'static + Future<Item = R, Error = E>,
        R: Send + 'static,
        E: Send + 'static,
    {
        let mut runtime = Runtime::new().expect("Unable to create a tokio runtime");
        runtime.block_on(fut)
    }

pub fn to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}