# Code Musique

> [!WARNING]
> This is an old student project. This probably won't be maintained,
> and this may not properly work. Also, this was done as an opportunity to learn
> Rust, WASM, music synthesis and language parsing (all hairy topics), so this
> project is probably very naive regarding all those things.

**Code Musique** is a simple live-coding environment on the browser. At every
bar, the code section is read, compiled, and used as instructions for music
synthesis.

It uses Rust compiled to WASM for both language parsing and music synthesis.

## Install

### Install `wasm-pack` with cargo

```bash
cargo install wasm-pack
```


### Build with `wasm-pack build`

```bash
wasm-pack build
```

### Test with `npm start`

```bash
cd www/
npm start
```

### Dependencies versions

npm version: 11.6.2
wasm-pack: 0.11.1
