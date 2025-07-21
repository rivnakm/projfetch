use std::{fmt::Display, path::PathBuf};

use termcolor::Color;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Language {
    Ada,
    Astro,
    C,
    CMake,
    Cobol,
    CPlusPlus,
    CSharp,
    Css,
    D,
    Dart,
    Dockerfile,
    Fish,
    Fortran,
    FSharp,
    GDScript,
    Gherkin,
    Glsl,
    Go,
    Haskell,
    Hcl,
    Html,
    Java,
    JavaScript,
    Julia,
    Lua,
    Makefile,
    Nim,
    Nix,
    Nu,
    OCaml,
    Perl,
    Php,
    Powershell,
    Python,
    Qml,
    Razor,
    React,
    Ruby,
    Rust,
    Sass,
    Scss,
    Shell,
    Sql,
    Svelte,
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
            Language::Css => "CSS",
            Language::FSharp => "F#",
            Language::Glsl => "GLSL",
            Language::Hcl => "HCL",
            Language::Html => "HTML",
            Language::Php => "PHP",
            Language::Qml => "QML",
            Language::Scss => "SCSS",
            Language::Sql => "SQL",
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
            // The color comments are just for nvim-highlight-colors
            Language::Ada => Color::Rgb(0, 0, 255), // rgb(0, 0, 255)
            Language::Astro => Color::Rgb(226, 57, 137), // rgb(226, 57, 137)
            Language::C => Color::Rgb(0, 89, 156),  // rgb(0, 89, 156)
            Language::CMake => Color::Rgb(0, 143, 186), // rgb(0, 143, 186)
            Language::Cobol => Color::Rgb(222, 209, 180), // rgb(222, 209, 180)
            Language::CPlusPlus => Color::Rgb(0, 89, 156), // rgb(0, 89, 156)
            Language::CSharp | Language::Razor | Language::VisualBasic | Language::Xaml => {
                Color::Rgb(81, 43, 212)
            } // rgb(81, 43, 212)
            Language::Css => Color::Rgb(102, 51, 153), // rgb(102, 51, 153)
            Language::D => Color::Rgb(152, 49, 42), // rgb(152, 49, 42)
            Language::Dart => Color::Rgb(1, 117, 194), // rgb(1, 117, 194)
            Language::Dockerfile => Color::Rgb(29, 99, 237), // rgb(29, 99, 237)
            Language::Fish => Color::Rgb(180, 180, 180), // rgb(180, 180, 180)
            Language::Fortran => Color::Rgb(115, 79, 150), // rgb(115, 79, 150)
            Language::FSharp => Color::Rgb(48, 185, 219), // rgb(48, 185, 219)
            Language::GDScript => Color::Rgb(53, 85, 112), // rgb(53, 85, 112)
            Language::Gherkin => Color::Rgb(0, 168, 24), // rgb(0, 168, 24)
            Language::Glsl => Color::Rgb(85, 134, 164), // rgb(85, 134, 164)
            Language::Go => Color::Rgb(1, 173, 216), // rgb(1, 173, 216)
            Language::Haskell => Color::Rgb(94, 80, 134), // rgb(94, 80, 134)
            Language::Hcl => Color::Rgb(123, 66, 188), // rgb(123, 66, 188)
            Language::Html => Color::Rgb(228, 77, 38), // rgb(228, 77, 38)
            Language::Java => Color::Rgb(248, 152, 29), // rgb(248, 152, 29)
            Language::JavaScript => Color::Rgb(240, 219, 79), // rgb(240, 219, 79)
            Language::Julia => Color::Rgb(149, 88, 178), // rgb(149, 88, 178)
            Language::Lua => Color::Rgb(0, 0, 128), // rgb(0, 0, 128)
            Language::Makefile => Color::Rgb(63, 63, 63), // rgb(63, 63, 63)
            Language::Nim => Color::Rgb(255, 233, 83), // rgb(255, 233, 83)
            Language::Nix => Color::Rgb(126, 126, 255), // rgb(126, 126, 255)
            Language::Nu => Color::Rgb(78, 154, 6), // rgb(78, 154, 6)
            Language::OCaml => Color::Rgb(238, 106, 26), // rgb(238, 106, 26)
            Language::Perl => Color::Rgb(57, 69, 126), // rgb(57, 69, 126)
            Language::Php => Color::Rgb(119, 123, 179), // rgb(119, 123, 179)
            Language::Powershell => Color::Rgb(83, 145, 254), // rgb(83, 145, 254)
            Language::Python => Color::Rgb(255, 221, 84), // rgb(255, 221, 84)
            Language::Qml => Color::Rgb(44, 222, 133), // rgb(44, 222, 133)
            Language::React => Color::Rgb(97, 219, 251), // rgb(97, 219, 251)
            Language::Ruby => Color::Rgb(204, 52, 45), // rgb(204, 52, 45)
            Language::Rust => Color::Rgb(255, 67, 0), // rgb(255, 67, 0)
            Language::Sass => Color::Rgb(165, 59, 112), // rgb(165, 59, 112)
            Language::Scss => Color::Rgb(198, 83, 140), // rgb(198, 83, 140)
            Language::Shell => Color::Rgb(80, 80, 80), // rgb(80, 80, 80)
            Language::Sql => Color::Rgb(160, 160, 160), // rgb(160, 160, 160)
            Language::Svelte => Color::Rgb(255, 62, 0), // rgb(255, 62, 0)
            Language::TypeScript => Color::Rgb(49, 120, 198), // rgb(49, 120, 198)
            Language::V => Color::Rgb(60, 86, 109), // rgb(60, 86, 109)
            Language::Vue => Color::Rgb(65, 184, 131), // rgb(65, 184, 131)
            Language::Zig => Color::Rgb(247, 164, 66), // rgb(247, 164, 66)
        }
    }
}

pub fn determine_language(path: PathBuf) -> Option<Language> {
    match path.file_name() {
        Some(os_str) => match os_str.as_encoded_bytes() {
            b"CMakeLists.txt" => return Some(Language::CMake),
            b"Makefile" | b"makefile" => return Some(Language::Makefile),
            _ => {
                let filename_string = os_str.to_string_lossy();
                if filename_string.starts_with("Containerfile")
                    || filename_string.starts_with("Dockerfile")
                {
                    return Some(Language::Dockerfile);
                }
            }
        },
        None => return None,
    }

    match path.extension() {
        None => None,
        Some(os_str) => Some(match os_str.as_encoded_bytes() {
            b"adb" => Language::Ada,
            b"ads" => Language::Ada,
            b"astro" => Language::Astro,
            b"axaml" => Language::Xaml,
            b"bash" => Language::Shell,
            b"c" => Language::C,
            b"cc" => Language::CPlusPlus,
            b"cob" => Language::Cobol,
            b"comp" => Language::Glsl,
            b"cpp" => Language::CPlusPlus,
            b"cppm" => Language::CPlusPlus,
            b"cxx" => Language::CPlusPlus,
            b"cs" => Language::CSharp,
            b"cshtml" => Language::Razor,
            b"css" => Language::Css,
            b"d" => Language::D,
            b"dart" => Language::Dart,
            b"f" => Language::Fortran,
            b"for" => Language::Fortran,
            b"f90" => Language::Fortran,
            b"f95" => Language::Fortran,
            b"f03" => Language::Fortran,
            b"f15" => Language::Fortran,
            b"feature" => Language::Gherkin,
            b"fish" => Language::Fish,
            b"frag" => Language::Glsl,
            b"fs" => Language::FSharp,
            b"gd" => Language::GDScript,
            b"geom" => Language::Glsl,
            b"glsl" => Language::Glsl,
            b"go" => Language::Go,
            b"h" => disambiguate_header(path),
            b"hpp" => Language::CPlusPlus,
            b"hs" => Language::Haskell,
            b"html" => Language::Html,
            b"ixx" => Language::CPlusPlus,
            b"java" => Language::Java,
            b"jl" => Language::Julia,
            b"js" => Language::JavaScript,
            b"jsx" => Language::React,
            b"lua" => Language::Lua,
            b"ml" => Language::OCaml,
            b"mpp" => Language::CPlusPlus,
            b"nim" => Language::Nim,
            b"nix" => Language::Nix,
            b"nu" => Language::Nu,
            b"php" => Language::Php,
            b"pl" => Language::Perl,
            b"ps1" => Language::Powershell,
            b"py" => Language::Python,
            b"qml" => Language::Qml,
            b"razor" => Language::Razor,
            b"rb" => Language::Ruby,
            b"rs" => Language::Rust,
            b"sass" => Language::Sass,
            b"scss" => Language::Scss,
            b"sh" => Language::Shell,
            b"sql" => Language::Sql,
            b"svelte" => Language::Svelte,
            b"tesc" => Language::Glsl,
            b"tese" => Language::Glsl,
            b"tf" => Language::Hcl,
            b"ts" => Language::TypeScript,
            b"tsx" => Language::React,
            b"v" => Language::V,
            b"vb" => Language::VisualBasic,
            b"vert" => Language::Glsl,
            b"vue" => Language::Vue,
            b"xaml" => Language::Xaml,
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
