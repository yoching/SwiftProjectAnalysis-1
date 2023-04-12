# Swift Project Analysis - 1

The bare minimum analysis of Swift projects.

<img src="./ProjectAnalysis.png" width=600>

## Features
- For each file, count file length, `struct`, `class`, and `enum`
- For the entire project, count `struct`, `class`, and `enum`
- And show them in a web UI✨

## How to run

```sh
$ cargo run -- -p path_to_project_root
```

## Motivation
- I was interested in objectively comparing multiple Swift projects because I've made a lot of iOS apps from scratch during my career.
- I picked up Rust here only because I wanted to learn the language.
- During this development, I realized that using [SwiftSyntax](https://github.com/apple/swift-syntax) was more suitable for in-depth analysis using Abstract Syntax Tree. Therefore, I made another project.


<!-- As an iOS app developer, I've made more than 10 apps from scratch. During development, I sometimes felt "this app is bigger and more complex than that", or vice versa. As an attempt to objectively compare iOS app projects, as a first step, I wrote these codes to count elements in Swift codebase.
Also, I picked up Rust just because I wanted to learn the language, however, I realized that using [SwiftSyntax] -->
