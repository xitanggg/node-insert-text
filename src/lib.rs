#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use arboard::Clipboard;
use enigo::{
  Direction::{Click, Press, Release},
  Enigo, Key, Keyboard, Settings,
};
use std::{thread, time};

/// Insert the given text at the current cursor position.
/// 
/// The whole text string is entered at once so it is performed quickly.
/// 
/// ##### Arguments
/// * `text` - Text to be inserted
/// * `insertWithPaste` - An optional boolean that sets whether to insert text with the paste 
///                       method. Default to false. (Setting true to use the paste method is 
///                       sometimes useful when the default insert method doesn't work for certain apps)
/// * `arrowKeyToClickBeforeInsert` - An optional string that sets which arrow key to click before 
///                                   inserting text. Can be either "left" or "right". Default to None. 
#[napi]
pub fn insert_text(text: String, insert_with_paste: Option<bool>, arrow_key_to_click_before_insert: Option<String>){
  let mut enigo = Enigo::new(&Settings::default()).unwrap();

  let arrow_key_to_click_before_insert = arrow_key_to_click_before_insert.unwrap_or(String::new());
  if arrow_key_to_click_before_insert == String::from("left") {
    enigo.key(Key::LeftArrow, Click).unwrap()
  }else if arrow_key_to_click_before_insert == String::from("right"){
    enigo.key(Key::RightArrow, Click).unwrap()
  }

  let insert_with_paste = insert_with_paste.unwrap_or(false);
  if insert_with_paste{
    let mut clipboard = Clipboard::new().unwrap();

    // Save clipboard existing text
    let clipboard_existing_text = clipboard.get_text().unwrap_or(String::new());

    // Set insert text to clipboard
    clipboard.set_text(&text).unwrap();

    // Simulate Ctrl/Cmd + V keyboard input to paste text
    let control_or_command_key = if cfg!(target_os = "macos") {
      Key::Meta
    } else {
      Key::Control
    };
    enigo.key(control_or_command_key, Press).unwrap();
    enigo.key(Key::Unicode('v'), Click).unwrap();
    enigo.key(control_or_command_key, Release).unwrap();

    // Wait some time for paste to be processed
    let twenty_ms = time::Duration::from_millis(20);
    thread::sleep(twenty_ms);
    
    // Restore clipboard previous existing text to minimize side effects to users
    clipboard.set_text(&clipboard_existing_text).unwrap();
  }else{
    enigo.text(&text).unwrap();
  }
}
