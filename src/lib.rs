#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use arboard::Clipboard;
use enigo::{
  Direction::{Click, Press, Release},
  Enigo, Key, Keyboard, Settings,
};
use std::{thread, time};

static DEFAULT_COPY_WAIT_TIME_MS: u32 = 5;
static DEFAULT_PASTE_WAIT_TIME_MS: u32 = 20;

/// Insert the given text at the current cursor position.
/// 
/// The whole text string is entered at once so it is performed quickly.
/// 
/// ##### Arguments
/// * `text` - Text to be inserted
/// * `insertWithPaste` - An optional boolean that sets whether to insert text with the paste 
///                       method. Default to false. (Setting true to use the paste method is 
///                       sometimes useful when the default insert method doesn't work for certain 
///                       apps. Note this feature is experimental)
/// * `arrowKeyToClickBeforeInsert` - An optional string that sets which arrow key to click before 
///                                   inserting text. Can be either "left" or "right". Default to None. 
/// * `copyWaitTimeMs`  - An optional number that sets how long to wait after performing the copy
///                       operation before pasting the clipboard text. It defaults to 5ms, which 
///                       works for most use cases with short insert text. However, a larger value
///                       would be needed to support use case for long insert text that takes 
///                       longer to copy to the clipboard. `copyWaitTimeMs` is only used when 
///                       using the paste method, i.e. when `insertWithPaste` is set to true.
/// * `pasteWaitTimeMs` - An optional number that sets how long to wait after performing the paste
///                       operation before restoring the previous clipboard text. It defaults to 20ms.
///                       `pasteWaitTimeMs` is only used when using the paste method, i.e. when
///                       `insertWithPaste` is set to true. 
#[napi]
pub fn insert_text(text: String, insert_with_paste: Option<bool>, arrow_key_to_click_before_insert: Option<String>, copy_wait_time_ms: Option<u32>, paste_wait_time_ms: Option<u32>){
  let mut enigo = Enigo::new(&Settings::default()).unwrap();

  let arrow_key_to_click_before_insert = arrow_key_to_click_before_insert.unwrap_or(String::new());
  if arrow_key_to_click_before_insert == String::from("left") {
    enigo.key(Key::LeftArrow, Click).unwrap()
  }else if arrow_key_to_click_before_insert == String::from("right"){
    enigo.key(Key::RightArrow, Click).unwrap()
  }

  let insert_with_paste = insert_with_paste.unwrap_or(false);
  if insert_with_paste{
    // If text is empty, we simply perform paste, with the assumption that user would handle
    // setting the insert text to clipboard and restoring the clipboard state themselves
    if text.is_empty(){
      paste(&mut enigo);
    }else{
      let mut clipboard = Clipboard::new().unwrap();
  
      // Save clipboard existing text
      let clipboard_existing_text = clipboard.get_text().unwrap_or(String::new());
  
      // Set insert text to clipboard
      clipboard.set_text(&text).unwrap();
  
      // Wait for clipboard to be updated with copied insert text
      thread::sleep(time::Duration::from_millis(u64::from(copy_wait_time_ms.unwrap_or(DEFAULT_PASTE_WAIT_TIME_MS))));
      
      paste(&mut enigo);
      
      // Wait for paste to be processed
      thread::sleep(time::Duration::from_millis(u64::from(paste_wait_time_ms.unwrap_or(DEFAULT_COPY_WAIT_TIME_MS))));
      
      // Restore clipboard previous existing text to minimize side effects to users
      clipboard.set_text(&clipboard_existing_text).unwrap();
    }
  }else{
    enigo.text(&text).unwrap();
  }
}

// Simulate Ctrl/Cmd + V keyboard input to paste text from clipboard
fn paste(enigo: &mut Enigo){
  let control_or_command_key = if cfg!(target_os = "macos") {
    Key::Meta
  } else {
    Key::Control
  };
  enigo.key(control_or_command_key, Press).unwrap();
  enigo.key(Key::Unicode('v'), Click).unwrap();
  enigo.key(control_or_command_key, Release).unwrap();
}