# Node Insert Text

This package provides a simple Node.js util that allows you to programmatically insert text on desktop.

- Support Windows and Mac
  - Mac: Need to grant accessibility permission to the calling app (`Settings` -> `Privacy & Security` -> `Accessibility`)
- Require Node.js >= 10

## 📦Installation

```bash
npm i @xitanggg/node-insert-text
```

## ℹ️Usage

```typescript
import { insertText } from '@xitanggg/node-insert-text;

insertText("👋Hello World! This line is inserted programmatically🤖");
```

`insertText` can accept 3 arguments

1. `text` - Text to be inserted
2. `insertWithPaste` - An optional boolean that sets whether to insert text with the paste method. Default to false. (Setting true to use the paste method is sometimes useful when the default insert method doesn't work for certain apps. Note this feature is experimental)
3. `arrowKeyToClickBeforeInsert` - An optional string that sets which arrow key to click before inserting text. Can be either "left" or "right". Default to None.

## 💡Implementation

**Core Logic**

The implementation is written in Rust and is ~20 lines of code (see `/src/lib.rs` for full source code)

The happy path is a wrapper of the `text` function of [enigo](https://github.com/enigo-rs/enigo), which is a cross platform input simulation library in Rust. The default behavior simply calls `enigo.text` to perform text insertion, which works for most use cases.

However, some apps might not support text insertion, so a `insertWithPaste` experimental fallback option is provided, which inserts text via clipboard paste in a 4 steps processes:

1. Save clipboard existing text
2. Set text to be inserted to clipboard
3. Simulate `Ctrl + V` (`Cmd + V` in Mac) keyboard input to paste text from clipboard
4. Restore the previous clipboard text to minimize side effects to users

**Dependencies**

It uses [Arboard (Arthur's Clipboard)](https://github.com/1Password/arboard) to perform clipboard operation and [enigo](https://github.com/enigo-rs/enigo) to perform keyboard input simulation

**Build & Distribution**

It uses [NAPI-RS](https://github.com/napi-rs/napi-rs) to compile the `enigo.text` function into binaries via GitHub actions, package it as a Node-API native addon, and then publish it to npm for distribution and easy use.

One very nice thing about the NAPI-RS tooling is that the binary has been built, so this package just works after installation, i.e. no need to build it yourself. Also, the binary is selectively installed, meaning installation only installs the binary that your system needs, e.g. windows or Mac, to keep the size small instead of including all binaries at once.
