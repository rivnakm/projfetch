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

            if line.starts_with(&comment_token) {
                continue;
            }

            sloc += 1;
        }

        sloc
    }
}

fn comment_token(lang: Language) -> String {
    match lang {
        Language::CMake
        | Language::Dockerfile
        | Language::Julia
        | Language::Makefile
        | Language::Nim
        | Language::Nix
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
        | Language::Go
        | Language::Java
        | Language::JavaScript
        | Language::Rust
        | Language::TypeScript
        | Language::V
        | Language::Zig => "//",
        Language::Ada | Language::Haskell | Language::Lua => "--",
        Language::Cobol => "*>",
        Language::Fortran => "!",
        Language::OCaml => "\0", // OCaml doesn't technically have single line comments
        Language::Php => "//",   // FIXME: '#' is also valid
        Language::VisualBasic => "'",
    }
    .into()
}

fn block_comment_tokens(lang: Language) -> Option<(String, String)> {
    match lang {
        Language::C
        | Language::CPlusPlus
        | Language::CSharp
        | Language::Dart
        | Language::Go
        | Language::Java
        | Language::JavaScript
        | Language::Nix
        | Language::Php
        | Language::Qml
        | Language::TypeScript
        | Language::V => Some(("/*", "*/")),
        Language::CMake => Some(("#[[", "]]")),
        Language::FSharp | Language::OCaml => Some(("(*", "*)")),
        Language::Julia => Some(("#=", "=#")),
        Language::Lua => Some(("--[[", "]]--")),
        Language::Nim => Some(("#[", "]#")),
        Language::Perl => Some(("=", "=cut")),
        Language::Powershell => Some(("<#", "#>")),
        Language::Python => Some(("'''", "'''")), // FIXME: '"""' is also valid
        Language::Ruby => Some(("=begin", "=end")),

        // FIXME: D has multiple different ways to comment out code
        // TBD if I care enough to handle all those cases
        Language::D => Some(("/*", "*/")),
        _ => None,
    }
    .map(|b| (b.0.into(), b.1.into()))
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
