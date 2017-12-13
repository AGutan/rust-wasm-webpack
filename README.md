# rust-wasm-webpack 🏎️ [![Build Status](https://travis-ci.org/yamafaktory/rust-wasm-webpack.svg?branch=master)](https://travis-ci.org/yamafaktory/rust-wasm-webpack) [![npm version](https://img.shields.io/npm/v/rust-wasm-webpack.svg?style=flat)](https://www.npmjs.com/package/rust-wasm-webpack) [![Standard - JavaScript Style Guide](https://img.shields.io/badge/code%20style-standard-brightgreen.svg)](http://standardjs.com/)

A simple boilerplate to get WebAssembly (WASM) code generated by Rust and bundled by Webpack!

## Prerequisite

To get started, we need to install **rustup**, the Rust toolchain installer:

```sh
curl https://sh.rustup.rs -sSf | sh
```

Please refer to its [documentation](https://github.com/rust-lang-nursery/rustup.rs).

## Usage

### Setup

This step will update to the latest Rust nightly and add the `wasm32-unknown-unknown` toolchain.

```sh
yarn setup
```

### Build

*You can skip this step and directly launch the dev server.*

```sh
yarn build
```

### Launch the dev server

```sh
yarn start
```

Go to http://localhost:9000/ and enjoy!
