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
    Ruby,
    Rust,
    Shell,
    TypeScript,
    V,
    VisualBasic,
    Vue,
    Xaml,
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
            Language::VisualBasic => "Visual Basic",
            Language::Xaml => "XAML",
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
            Language::CSharp | Language::VisualBasic | Language::Xaml => Color::Rgb(81, 43, 212),
            Language::D => Color::Rgb(152, 49, 42),
            Language::Dart => Color::Rgb(1, 117, 194),
            Language::Dockerfile => Color::Rgb(29, 99, 237),
            Language::Fortran => Color::Rgb(115, 79, 150),
            Language::FSharp => Color::Rgb(48, 185, 219),
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
            Language::Python => Color::Rgb(54, 112, 160),
            Language::Ruby => Color::Rgb(204, 52, 45),
            Language::Rust => Color::Rgb(255, 67, 0),
            Language::Shell => Color::Rgb(80, 80, 80),
            Language::TypeScript => Color::Rgb(49, 120, 198),
            Language::V => Color::Rgb(60, 86, 109),
            Language::Vue => Color::Rgb(65, 184, 131),
            Language::Zig => Color::Rgb(247, 164, 66),
        }
    }
}

pub fn determine_language(path: PathBuf) -> Option<Language> {
    match path.file_name() {
        Some(os_str) => match os_str.to_string_lossy().to_string().as_str() {
            "CMakeLists.txt" => return Some(Language::CMake),
            "Containerfile" => return Some(Language::Dockerfile),
            "Dockerfile" => return Some(Language::Dockerfile),
            "Makefile" | "makefile" => return Some(Language::Makefile),
            _ => (),
        },
        None => return None,
    }

    match path.extension() {
        None => None,
        Some(os_str) => match os_str.to_string_lossy().to_string().as_str() {
            "adb" => Some(Language::Ada),
            "ads" => Some(Language::Ada),
            "axaml" => Some(Language::Xaml),
            "bash" => Some(Language::Shell),
            "c" => Some(Language::C),
            "cc" => Some(Language::CPlusPlus),
            "cob" => Some(Language::Cobol),
            "cpp" => Some(Language::CPlusPlus),
            "cppm" => Some(Language::CPlusPlus),
            "cxx" => Some(Language::CPlusPlus),
            "cs" => Some(Language::CSharp),
            "d" => Some(Language::D),
            "dart" => Some(Language::Dart),
            "f" => Some(Language::Fortran),
            "for" => Some(Language::Fortran),
            "f90" => Some(Language::Fortran),
            "f95" => Some(Language::Fortran),
            "f03" => Some(Language::Fortran),
            "f15" => Some(Language::Fortran),
            "fish" => Some(Language::Shell),
            "fs" => Some(Language::FSharp),
            "go" => Some(Language::Go),
            "h" => Some(disambiguate_header(path)),
            "hpp" => Some(Language::CPlusPlus),
            "hs" => Some(Language::Haskell),
            "ixx" => Some(Language::CPlusPlus),
            "java" => Some(Language::Java),
            "jl" => Some(Language::Julia),
            "js" => Some(Language::JavaScript),
            "lua" => Some(Language::Lua),
            "ml" => Some(Language::OCaml),
            "mpp" => Some(Language::CPlusPlus),
            "nim" => Some(Language::Nim),
            "nix" => Some(Language::Nix),
            "php" => Some(Language::Php),
            "pl" => Some(Language::Perl),
            "ps1" => Some(Language::Powershell),
            "py" => Some(Language::Python),
            "rb" => Some(Language::Ruby),
            "rs" => Some(Language::Rust),
            "sh" => Some(Language::Shell),
            "ts" => Some(Language::TypeScript),
            "v" => Some(Language::V),
            "vb" => Some(Language::VisualBasic),
            "vue" => Some(Language::Vue),
            "xaml" => Some(Language::Xaml),
            "zig" => Some(Language::Zig),
            "zsh" => Some(Language::Shell),
            _ => None,
        },
    }
}

fn disambiguate_header(path: PathBuf) -> Language {
    let contents = std::fs::read_to_string(path).unwrap();

    if contents.contains("class")
        || contents.contains("template")
        || contents.contains("extern \"C\"")
    {
        return Language::CPlusPlus;
    }

    Language::C
}
