# Enigo Node Insert Text

This package provides a simple Node.js util that allows you to programmatically insert text on desktop.

- Support Windows and Mac
  - Mac: Need to grant accessibility permission to the calling app (`Settings` -> `Privacy & Security` -> `Accessibility`)
- Require Node.js >= 10

## 📦Installation

```bash
npm i @xitanggg/enigo-node-insert-text
```

## ℹ️Usage

```typescript
import { insertText } from '@xitanggg/enigo-node-insert-text;

insertText("👋Hello World! This line is inserted programmatically🤖");
```

## 💡Implementation

This project is simply a wrapper of the `text` function of [enigo](https://github.com/enigo-rs/enigo), which is a cross platform input simulation library in Rust.

It uses [NAPI-RS](https://github.com/napi-rs/napi-rs) to compile the `enigo.text` function into binaries via GitHub actions, package it as a Node-API native addon, and then publish it to npm for distribution and easy use.

One very nice thing about the NAPI-RS tooling is that the binary has been built, so this package just works after installation, i.e. no need to build it yourself. Also, the binary is selectively installed, meaning installation only installs the binary that your system needs, e.g. windows or Mac, to keep the size small instead of including all binaries at once.
