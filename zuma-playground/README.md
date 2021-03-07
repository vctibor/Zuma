# ZUMA playground

This is source for [ZUMA playground](https://zuma-playground.netlify.app/). It is Rust / WASM application developed using [seed](https://seed-rs.org/) library.

## Install dependencies

    cargo install cargo-make

## Development

In one terminal:

    cargo make watch

In another:

    cargo make serve

## Deploy

    cargo make verify
    cargo make build_release

    rm -rf dist
    mkdir dist
    cp index.html dist
    cp -r pkg dist
    cp -r resources dist
