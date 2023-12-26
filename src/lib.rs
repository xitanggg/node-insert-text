#![deny(clippy::all)]

use enigo::{
  Enigo, Keyboard, Settings,
};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn insert_text(text: String){
  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  enigo.text(&text).unwrap();
}
