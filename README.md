# Node Insert Text

This package provides a simple Node.js util that allows you to programmatically insert text on desktop.

- Support Windows and Mac
  - Mac: Need to grant accessibility permission to the calling app (`Settings` -> `Privacy & Security` -> `Accessibility`) to call Apple's [Core Graphics framework](https://developer.apple.com/documentation/coregraphics) to post [CGEvent](https://developer.apple.com/documentation/coregraphics/cgevent) to insert text
- Require Node.js >= 10

## 📦Installation

```bash
npm i @xitanggg/node-insert-text
```

## ℹ️Usage

```typescript
import { insertText } from '@xitanggg/node-insert-text';

insertText('👋Hello World! This line is inserted programmatically🤖');
```

`insertText` accepts 3 arguments

1. `text` - Text to be inserted
2. `insertWithPaste` - An optional boolean that sets whether to insert text with the paste method. Default to false. (Setting true to use the paste method is useful to bypass some limitations in the default insert method. For example, the default insert method may not work for some apps, and in Mac, it doesn't work when certain key, such as `Cmd`, is pressed during insert.)
3. `arrowKeyToClickBeforeInsert` - An optional string that sets which arrow key to click before inserting text. Can be either "left" or "right". Default to None.

## 💡Implementation

**Core Logic**

The implementation is written in Rust and is ~200 lines of code in a single file `/src/lib.rs`.

The happy path is a wrapper of the `text` function of [enigo](https://github.com/enigo-rs/enigo), which is a cross platform input simulation library in Rust. The default behavior simply calls `enigo.text` to perform text insertion, which works for most use cases.

However, `enigo.text` has its limitations: 1. it may not work for some apps, and 2. In Mac, it doesn't work when certain key, such as `Cmd`, is pressed during insert [(issue link)](https://github.com/enigo-rs/enigo/issues/297). Therefore, an alternative option `insertWithPaste` is provided to insert text via clipboard paste in a 5 steps process:

1. Save clipboard existing text or image
2. Clear clipboard
3. Set text to be inserted to clipboard
4. Simulate `Ctrl + V` (`Cmd + V` in Mac) keyboard input to paste text from clipboard
5. Restore clipboard previous text or image to minimize side effects to users

**Dependencies**

It uses [Arboard (Arthur's Clipboard)](https://github.com/1Password/arboard) to perform clipboard operation and [enigo](https://github.com/enigo-rs/enigo) to perform keyboard input simulation.

Arboard supports getting and setting clipboard text and image, which should satisfy most use cases. But it is worth noting that it doesn't support other clipboard contents, e.g. html, rtf, file. A `paste` method is provided for those who would like to implement custom logics to save and restore clipboard state or just want to call `Ctrl + V` (`Cmd + V` in Mac) to paste.

```typescript
import { paste } from '@xitanggg/node-insert-text';

// Skip custom logics to save clipboard state and write text to clipboard
paste();
// Skip custom logics to restore clipboard state
```

**Build & Distribution**

It uses [NAPI-RS](https://github.com/napi-rs/napi-rs) to compile the library into binaries via GitHub actions, package it as a Node-API native addon, and then publish it to npm for distribution and easy use.

One very nice thing about the NAPI-RS tooling is that the binary has been built, so this package just works after installation, i.e. no need to build it yourself. Also, the binary is selectively installed, meaning installation only installs the binary that your system needs, e.g. windows or Mac, to keep the size small instead of including all binaries at once.
