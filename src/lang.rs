use std::{fmt::Display, path::PathBuf};

use termcolor::Color;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Language {
    Ada,
    C,
    CMake,
    Cobol,
    CPlusPlus,
    CSharp,
    D,
    Dart,
    Dockerfile,
    Fortran,
    FSharp,
    Gherkin,
    Go,
    Haskell,
    Java,
    JavaScript,
    Julia,
    Lua,
    Makefile,
    Nim,
    Nix,
    OCaml,
    Perl,
    Php,
    Powershell,
    Python,
    Qml,
    Ruby,
    Rust,
    Shell,
    TypeScript,
    V,
    VisualBasic,
    Zig,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Language::Cobol => "COBOL",
            Language::CPlusPlus => "C++",
            Language::CSharp => "C#",
            Language::FSharp => "F#",
            Language::Php => "PHP",
            Language::Qml => "QML",
            Language::VisualBasic => "Visual Basic",
            _ => &format!("{:?}", self),
        };
        write!(f, "{}", name)
    }
}

impl Language {
    pub fn color(&self) -> Color {
        match self {
            Language::Ada => Color::Rgb(0, 0, 255),
            Language::C => Color::Rgb(0, 89, 156),
            Language::CMake => Color::Rgb(0, 143, 186),
            Language::Cobol => Color::Rgb(222, 209, 180),
            Language::CPlusPlus => Color::Rgb(0, 89, 156),
            Language::CSharp | Language::VisualBasic => Color::Rgb(81, 43, 212),
            Language::D => Color::Rgb(152, 49, 42),
            Language::Dart => Color::Rgb(1, 117, 194),
            Language::Dockerfile => Color::Rgb(29, 99, 237),
            Language::Fortran => Color::Rgb(115, 79, 150),
            Language::FSharp => Color::Rgb(48, 185, 219),
            Language::Gherkin => Color::Rgb(0, 168, 24),
            Language::Go => Color::Rgb(1, 173, 216),
            Language::Haskell => Color::Rgb(94, 80, 134),
            Language::Java => Color::Rgb(248, 152, 29),
            Language::JavaScript => Color::Rgb(240, 219, 79),
            Language::Julia => Color::Rgb(149, 88, 178),
            Language::Lua => Color::Rgb(0, 0, 128),
            Language::Makefile => Color::Rgb(63, 63, 63),
            Language::Nim => Color::Rgb(255, 233, 83),
            Language::Nix => Color::Rgb(126, 126, 255),
            Language::OCaml => Color::Rgb(238, 106, 26),
            Language::Perl => Color::Rgb(57, 69, 126),
            Language::Php => Color::Rgb(119, 123, 179),
            Language::Powershell => Color::Rgb(83, 145, 254),
            Language::Python => Color::Rgb(255, 221, 84),
            Language::Qml => Color::Rgb(44, 222, 133),
            Language::Ruby => Color::Rgb(204, 52, 45),
            Language::Rust => Color::Rgb(255, 67, 0),
            Language::Shell => Color::Rgb(80, 80, 80),
            Language::TypeScript => Color::Rgb(49, 120, 198),
            Language::V => Color::Rgb(60, 86, 109),
            Language::Zig => Color::Rgb(247, 164, 66),
        }
    }
}

pub fn determine_language(path: PathBuf) -> Option<Language> {
    match path.file_name() {
        Some(os_str) => match os_str.as_encoded_bytes() {
            b"CMakeLists.txt" => return Some(Language::CMake),
            b"Containerfile" => return Some(Language::Dockerfile),
            b"Dockerfile" => return Some(Language::Dockerfile),
            b"Makefile" | b"makefile" => return Some(Language::Makefile),
            _ => (),
        },
        None => return None,
    }

    match path.extension() {
        None => None,
        Some(os_str) => Some(match os_str.as_encoded_bytes() {
            b"adb" => Language::Ada,
            b"ads" => Language::Ada,
            b"bash" => Language::Shell,
            b"c" => Language::C,
            b"cc" => Language::CPlusPlus,
            b"cob" => Language::Cobol,
            b"cpp" => Language::CPlusPlus,
            b"cppm" => Language::CPlusPlus,
            b"cxx" => Language::CPlusPlus,
            b"cs" => Language::CSharp,
            b"d" => Language::D,
            b"dart" => Language::Dart,
            b"f" => Language::Fortran,
            b"feature" => Language::Gherkin,
            b"for" => Language::Fortran,
            b"f90" => Language::Fortran,
            b"f95" => Language::Fortran,
            b"f03" => Language::Fortran,
            b"f15" => Language::Fortran,
            b"fish" => Language::Shell,
            b"fs" => Language::FSharp,
            b"go" => Language::Go,
            b"h" => disambiguate_header(path),
            b"hpp" => Language::CPlusPlus,
            b"hs" => Language::Haskell,
            b"ixx" => Language::CPlusPlus,
            b"java" => Language::Java,
            b"jl" => Language::Julia,
            b"js" => Language::JavaScript,
            b"lua" => Language::Lua,
            b"ml" => Language::OCaml,
            b"mpp" => Language::CPlusPlus,
            b"nim" => Language::Nim,
            b"nix" => Language::Nix,
            b"php" => Language::Php,
            b"pl" => Language::Perl,
            b"ps1" => Language::Powershell,
            b"py" => Language::Python,
            b"qml" => Language::Qml,
            b"rb" => Language::Ruby,
            b"rs" => Language::Rust,
            b"sh" => Language::Shell,
            b"ts" => Language::TypeScript,
            b"v" => Language::V,
            b"vb" => Language::VisualBasic,
            b"zig" => Language::Zig,
            b"zsh" => Language::Shell,
            _ => return None,
        }),
    }
}

fn disambiguate_header(path: PathBuf) -> Language {
    let contents = std::fs::read_to_string(path).unwrap();

    if contents.contains("<string>")
        || contents.contains("<vector>")
        || contents.contains("<iostream>")
        || contents.contains("template")
        || contents.contains("namespace")
        || contents.contains("extern \"C\"")
    {
        return Language::CPlusPlus;
    }

    Language::C
}
