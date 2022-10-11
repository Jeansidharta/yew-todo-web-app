# Yew.rs Todo list

This is a simple proof-of-concept project to show that it's possible to make web apps using the rust programming language. It's a Todo list app.

## Observations

- This is a Single-page application, rendered on the client's computer. This is not ideal. But there SSR and SSG on the way.
- The bundle size is around 120 KiB. This is good.
- This project generates CSS classes, but they aren't bundled during the build. This means these classes are probably generated client-side. Not good. I'll have to research for a way to generate static CSS.

## Pre-requisites

You must have installed:

- [Rust](https://www.rust-lang.org/tools/install)'s environment
- [Trunk](https://yew.rs/docs/getting-started/project-setup/using-trunk)

## How to run

Just clone this project and run `trunk serve`. Then just access http://localhost:8080

