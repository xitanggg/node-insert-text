#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use enigo::{
  Enigo, Keyboard, Settings,
};

/// Insert the given text at the current cursor position.
/// 
/// The whole text string is entered at once so it is performed quickly.
/// 
/// ##### Arguments
/// * `text` - Text to be inserted
#[napi]
pub fn insert_text(text: String){
  let mut enigo = Enigo::new(&Settings::default()).unwrap();
  enigo.text(&text).unwrap();
}
