use std::path::Path;

use crate::lang::Language;

pub struct CodeReader {
    contents: String,
    lang: Language,
}

impl CodeReader {
    #[allow(dead_code)]
    pub fn new(contents: String, lang: Language) -> CodeReader {
        CodeReader { contents, lang }
    }

    pub fn from_path(path: &Path, lang: Language) -> std::io::Result<CodeReader> {
        let contents = std::fs::read_to_string(path)?;

        Ok(CodeReader { contents, lang })
    }

    pub fn sloc(&self) -> usize {
        let lines = self.contents.lines();

        let mut sloc = 0;

        let comment_token = comment_token(self.lang);
        let (block_comment_start, block_comment_end) = match block_comment_tokens(self.lang) {
            Some((start, end)) => (Some(start), Some(end)),
            None => (None, None),
        };

        let mut in_block_comment = false;

        for line in lines {
            // COBOL is weird and columns matter
            if matches!(self.lang, Language::Cobol) {
                // Indicator column is column 7 (1-indexed)
                const INDICATOR_COLUMN: usize = 6;

                if let Some(ch) = line.chars().nth(INDICATOR_COLUMN) {
                    if ch == '*' || ch == '/' {
                        continue;
                    }
                }
            }

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Block comments
            if let Some(ref block_comment_start) = block_comment_start {
                if !in_block_comment && line.starts_with(block_comment_start.as_str()) {
                    in_block_comment = true;
                }
            }
            if let Some(ref block_comment_end) = block_comment_end {
                if in_block_comment && line.ends_with(block_comment_end.as_str()) {
                    in_block_comment = false;
                    continue;
                }
            }
            if in_block_comment {
                continue;
            }

            if let Some(ref comment_token) = comment_token {
                if line.starts_with(comment_token) {
                    continue;
                }
            }

            sloc += 1;
        }

        sloc
    }
}

fn comment_token(lang: Language) -> Option<String> {
    Some(
        match lang {
            Language::CMake
            | Language::Dockerfile
            | Language::Fish
            | Language::Gherkin
            | Language::Hcl
            | Language::Julia
            | Language::Makefile
            | Language::Nim
            | Language::Nix
            | Language::Nu
            | Language::Perl
            | Language::Powershell
            | Language::Python
            | Language::Qml
            | Language::Ruby
            | Language::Shell => "#",
            Language::C
            | Language::CPlusPlus
            | Language::CSharp
            | Language::D
            | Language::Dart
            | Language::FSharp
            | Language::Glsl
            | Language::Go
            | Language::Java
            | Language::JavaScript
            | Language::Protobuf
            | Language::React
            | Language::Rust
            | Language::TypeScript
            | Language::V
            | Language::Zig => "//",
            Language::Ada | Language::Haskell | Language::Lua | Language::Sql => "--",
            Language::Cobol => "*>",
            Language::Fortran => "!",
            Language::OCaml => "\0", // OCaml doesn't technically have single line comments
            Language::Php => "//",   // FIXME: '#' is also valid
            Language::VisualBasic => "'",
            _ => return None,
        }
        .into(),
    )
}

fn block_comment_tokens(lang: Language) -> Option<(String, String)> {
    let tokens = match lang {
        Language::C
        | Language::CPlusPlus
        | Language::CSharp
        | Language::Css
        | Language::Dart
        | Language::Glsl
        | Language::Go
        | Language::Java
        | Language::JavaScript
        | Language::Nix
        | Language::Php
        | Language::Qml
        | Language::React
        | Language::Sql
        | Language::TypeScript
        | Language::V => ("/*", "*/"),
        Language::CMake => ("#[[", "]]"),
        Language::FSharp | Language::OCaml => ("(*", "*)"),
        Language::Julia => ("#=", "=#"),
        Language::Lua => ("--[[", "]]--"),
        Language::Nim => ("#[", "]#"),
        Language::Perl => ("=", "=cut"),
        Language::Powershell => ("<#", "#>"),
        Language::Python => ("'''", "'''"), // FIXME: '"""' is also valid
        Language::Ruby => ("=begin", "=end"),

        // FIXME: D has multiple different ways to comment out code
        // TBD if I care enough to handle all those cases
        Language::D => ("/*", "*/"),
        Language::Astro
        | Language::Html
        | Language::Razor
        | Language::Svelte
        | Language::Vue
        | Language::Xaml => ("<!--", "-->"),
        _ => return None,
    };

    Some((tokens.0.into(), tokens.1.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let contents = String::from(
            r#"
#include <stdio.h>

int main(void) {
    printf("Hello, World!\n");

    return 0;
}
"#,
        );

        let expected = 5;

        let reader = CodeReader::new(contents, Language::C);
        let actual = reader.sloc();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comment() {
        let contents = String::from(
            r#"
using System;

namespace Projfetch;

public class Program
{
    public static void Main(string[] args)
    {
        // Print output
        Console.WriteLine("Hello, World!");
    }
}
        "#,
        );

        let expected = 9;

        let reader = CodeReader::new(contents, Language::CSharp);
        let actual = reader.sloc();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_block_comment() {
        let contents = String::from(
            r#"
#include <iostream>

int main() {
    /*
     * C++ uses overloaded bit shift operators to print to output streams
     */
    std::cout << "Hello, World" << std::endl;

    return 0;
}
        "#,
        );

        let expected = 5;

        let reader = CodeReader::new(contents, Language::CPlusPlus);
        let actual = reader.sloc();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_single_line_block_comment() {
        let contents = String::from(
            r#"
(* single line block format *)
let () = Printf.printf "%s\n" "Hello, World!"
        "#,
        );

        let expected = 1;

        let reader = CodeReader::new(contents, Language::OCaml);
        let actual = reader.sloc();

        assert_eq!(actual, expected);
    }
}
