#[macro_use]
extern crate afl;
extern crate factom;

use factom::*;

async fn main() {
    let client = Factom::new();
    fuzz!(|data: &[u8]| {
      let input = str::from_utf8(data).unwrap()
      let response = debug::send_raw_message(&client, input).await.unwrap();
    });
}