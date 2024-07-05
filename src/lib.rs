#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use arboard::{ Clipboard, ImageData };
use enigo::{ Direction::{ Click, Press, Release }, Enigo, Key, Keyboard, Settings };
use std::{ thread, time::Duration };
#[cfg(target_os = "macos")]
use core_graphics::{
    event::{ CGEvent, CGEventTapLocation, KeyCode, CGEventFlags },
    event_source::{ CGEventSource, CGEventSourceStateID },
};

#[cfg(target_os = "macos")]
static DEFAULT_MAC_CG_EVENT_WAIT_TIME_MS: u64 = 20;

static DEFAULT_PASTE_WAIT_TIME_MS: u32 = 30;

/// Insert the given text at the current cursor position.
///
/// The whole text string is entered at once so it is performed quickly.
///
/// ##### Arguments
/// * `text` - Text to be inserted
/// * `insertWithPaste` - An optional boolean that sets whether to insert text with the paste method.
///                       Default to false. (Setting true to use the paste method is useful to bypass
///                       some limitations in the default insert method. For example, the default
///                       insert method may not work for some apps, and in Mac, it doesn't work
///                       when certain key, such as Cmd, is pressed during insert.)
/// * `arrowKeyToClickBeforeInsert` - An optional string that sets which arrow key to click before
///                                   inserting text. Can be either "left" or "right". Default to None.
/// * `pasteWaitTimeMs` - An optional number that sets how long to wait after performing the paste
///                       operation before restoring the previous clipboard state. Default to 30ms.
///                       `pasteWaitTimeMs` is only used when using the paste method, i.e. when
///                       `insertWithPaste` is set to true. (Beware of setting this value too low,
///                       as it may end up pasting the previous clipboard text/image)
#[napi]
pub fn insert_text(
    text: String,
    insert_with_paste: Option<bool>,
    arrow_key_to_click_before_insert: Option<String>,
    paste_wait_time_ms: Option<u32>
) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let arrow_key = arrow_key_to_click_before_insert.unwrap_or(String::new());
    if arrow_key == "left" || arrow_key == "right" {
        _click_arrow_key(&mut enigo, arrow_key);
    }

    let insert_with_paste = insert_with_paste.unwrap_or(false);
    if !insert_with_paste {
        // Insert text using default method
        // Note: This may not work for some apps, and in Mac, it doesn't work when
        // certain key, such as Cmd, is pressed during insert (https://github.com/enigo-rs/enigo/issues/297)
        enigo.text(&text).unwrap();
    } else {
        // Insert text using paste method in 5 steps process:
        let mut clipboard = Clipboard::new().unwrap();

        // 1. Save clipboard existing text or image
        let clipboard_text = clipboard.get_text().unwrap_or(String::new());
        let clipboard_image = clipboard
            .get_image()
            .unwrap_or(ImageData { width: 0, height: 0, bytes: [].as_ref().into() });

        // 2. Clear clipboard
        clipboard.clear().unwrap();

        // 3. Set text to be inserted to clipboard
        clipboard.set_text(&text).unwrap();

        // 4. Simulate Ctrl+ V (Cmd + V in Mac) keyboard input to paste text from clipboard
        _paste(&mut enigo);

        // 5. Restore clipboard previous text or image to minimize side effects to users
        let should_restore_clipboard_text = !clipboard_text.is_empty();
        let should_restore_clipboard_image = clipboard_image.width > 0;
        if should_restore_clipboard_text || should_restore_clipboard_image {
            // Wait for paste to be processed before restoring clipboard state
            // If restoring too soon, it would end up pasting the previous clipboard text/image
            thread::sleep(
                Duration::from_millis(
                    u64::from(paste_wait_time_ms.unwrap_or(DEFAULT_PASTE_WAIT_TIME_MS))
                )
            );

            if should_restore_clipboard_text {
                clipboard.set_text(&clipboard_text).unwrap();
            } else if should_restore_clipboard_image {
                clipboard.set_image(clipboard_image).unwrap();
            }
        }
    }
}

/// Simulate Ctrl+ V (Cmd + V in Mac) keyboard input to perform paste
///
/// ##### Arguments
/// * `arrowKeyToClickBeforePaste` - An optional string that sets which arrow key to click before
///                                  pasting. Can be either "left" or "right". Default to None.
#[napi]
pub fn paste(arrow_key_to_click_before_paste: Option<String>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let arrow_key = arrow_key_to_click_before_paste.unwrap_or(String::new());
    if arrow_key == "left" || arrow_key == "right" {
        _click_arrow_key(&mut enigo, arrow_key);
    }

    _paste(&mut enigo);
}

/// Simulate arrow key click (left or right) - Mac
#[cfg(target_os = "macos")]
fn _click_arrow_key(enigo: &mut Enigo, arrow_key: String) {
    let arrow_key_code = if arrow_key == "left" {
        KeyCode::LEFT_ARROW
    } else {
        KeyCode::RIGHT_ARROW
    };

    let event_source_state_id = CGEventSourceStateID::CombinedSessionState;
    let event_source = CGEventSource::new(event_source_state_id).unwrap();
    let event_tap_location = CGEventTapLocation::HID;

    let press_arrow_key_event = CGEvent::new_keyboard_event(
        event_source.clone(),
        arrow_key_code,
        true
    ).unwrap();
    press_arrow_key_event.post(event_tap_location);

    let release_arrow_key_event = CGEvent::new_keyboard_event(
        event_source.clone(),
        arrow_key_code,
        false
    ).unwrap();
    release_arrow_key_event.post(event_tap_location);
    thread::sleep(Duration::from_millis(DEFAULT_MAC_CG_EVENT_WAIT_TIME_MS));
}

/// Simulate arrow key click (left or right) - Windows
#[cfg(not(target_os = "macos"))]
fn _click_arrow_key(enigo: &mut Enigo, arrow_key: String) {
    let key = if arrow_key == "left" { Key::LeftArrow } else { Key::RightArrow };
    enigo.key(key, Click).unwrap();
}

// Define CG key code for "v" key
// Reference: https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.13.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h#L206
#[cfg(target_os = "macos")]
static V_KEY_CODE: u16 = 0x09;

/// Simulate Ctrl+ V (Cmd + V in Mac) keyboard input to perform paste - Mac
///
/// Windows calls into Enigo to simulate keyboard input. But for Mac, it calls into
/// Mac's Core Graphics CGEvent library directly to work around 2 issues with Enigo's current
/// implementation, which causes additional delay (https://github.com/enigo-rs/enigo/issues/105)
/// and subjects to mouse movement/keyboard interruption (https://github.com/enigo-rs/enigo/issues/201).
/// Calling into CGEvent and setting event flag solves both issues.
#[cfg(target_os = "macos")]
fn _paste(enigo: &mut Enigo) {
    // Implementation reference: https://stackoverflow.com/questions/2008126/cgeventpost-possible-bug-when-simulating-keyboard-events

    // Event source state id reference: https://developer.apple.com/documentation/coregraphics/cgeventsourcestateid
    let event_source_state_id = CGEventSourceStateID::CombinedSessionState;
    let event_source = CGEventSource::new(event_source_state_id).unwrap();
    // Event tap location reference: https://developer.apple.com/documentation/coregraphics/cgeventtaplocation
    let event_tap_location = CGEventTapLocation::HID;

    let press_cmd_v_event = CGEvent::new_keyboard_event(
        event_source.clone(),
        V_KEY_CODE,
        true
    ).unwrap();
    press_cmd_v_event.set_flags(CGEventFlags::CGEventFlagCommand); // Set flags to Cmd
    press_cmd_v_event.post(event_tap_location);

    let release_v_event = CGEvent::new_keyboard_event(
        event_source.clone(),
        V_KEY_CODE,
        false
    ).unwrap();
    release_v_event.set_flags(CGEventFlags::CGEventFlagNull); // Reset flags to null
    release_v_event.post(event_tap_location);

    // Release Cmd Key for completeness. May or may not be necessary
    // given Apple's documentation is not clear on this.
    let release_cmd_event = CGEvent::new_keyboard_event(
        event_source.clone(),
        KeyCode::COMMAND,
        false
    ).unwrap();
    release_cmd_event.post(event_tap_location);
}

/// Simulate Ctrl+ V (Cmd + V in Mac) keyboard input to perform paste - Windows
///
/// Windows calls into Enigo to simulate keyboard input
#[cfg(not(target_os = "macos"))]
fn _paste(enigo: &mut Enigo) {
    enigo.key(Key::Control, Press).unwrap();
    enigo.key(Key::Unicode('v'), Click).unwrap();
    enigo.key(Key::Control, Release).unwrap();
}
