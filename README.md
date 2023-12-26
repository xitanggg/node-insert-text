# Enigo Node Insert Text

This project provides a simple Node.js util that allows you to programmatically insert text in desktop.

The core of this project is powered by [enigo](https://github.com/enigo-rs/enigo), a cross platform input simulation in Rust. This project simply uses [NAPI-RS](https://github.com/napi-rs/napi-rs) to compile the `enigo.text` function as Node.js addons and exposes it to npm for easy use.

Only supports Windows and Mac.

## Usage

```typescript
import {insertText} from '@xitanggg/enigo-node-insert-text;

insertText("Hello World! here is a lot of text❤️")
```
