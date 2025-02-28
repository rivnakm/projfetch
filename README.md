# projfetch

Fetch tool to provide code information about multiple projects within a directory

![image](https://github.com/user-attachments/assets/bde35924-1022-492f-bb58-4f6eece4e40f)

## Installation

projfetch is not currently hosted on [crates.io](https://crates.io), but you can install via `cargo` regardless.

```sh
cargo install --git https://github.com/rivnakm/projfetch
```

## Usage

Run `projfetch` in a directory to see the language distribution.

You can use the `--count/-n N` flag to only display at most the top `N` languages, or the `--all/-a` flag to show them all. The default is 10

Files/directories can also be ignored with a `.gitignore` or `.ignore` file.

## Supported filetypes

- Ada
- Astro
- C
- CMake
- CSS
- COBOL
- C++
- C#
- D
- Dart
- Dockerfile
- Fish
- Fortran
- F#
- GDScript
- GLSL
- Go
- Haskell
- HCL
- HTML
- Java
- JavaScript
- Julia
- Lua
- Makefile
- Nim
- Nix
- Nu
- OCaml
- Perl
- PHP
- Powershell
- Python
- QML
- Razor
- React
- Ruby
- Rust
- Sass
- SCSS
- Shell
- Svelte
- TypeScript
- V
- Vue
- Visual Basic
- Zig
