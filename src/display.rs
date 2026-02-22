use std::path::Path;

use crossterm::{ExecutableCommand, cursor::MoveToColumn};
use human_repr::HumanCount;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::lang::Language;

pub fn print_results_summary(results: Vec<(Language, usize)>, length: usize) {
    let total_lines = results.iter().map(|r| r.1).sum::<usize>();
    let mut stdstream_stdout = StandardStream::stdout(ColorChoice::Always);

    let length = length.min(results.len());
    let truncated = length < results.len();
    for (i, (lang, lines)) in results.into_iter().take(length).enumerate() {
        let percent = lines as f32 / total_lines as f32;

        print!("{} (", lang);

        stdstream_stdout
            .set_color(ColorSpec::new().set_fg(Some(lang.color())))
            .unwrap();
        print!("{:.0}%", percent * 100.0);

        stdstream_stdout.reset().unwrap();

        if i < length - 1 {
            print!("), ");
        } else {
            print!(")");
        }
    }

    if truncated {
        println!(", ...");
    } else {
        println!();
    }
}

pub fn print_results_compact(results: Vec<(Language, usize)>, max_width: Option<u16>) {
    if results.is_empty() {
        return;
    }

    let total_lines = results.iter().map(|r| r.1).sum::<usize>();
    let window_size = crossterm::terminal::window_size().expect("Couldn't get terminal size");
    let columns = match max_width {
        Some(max_width) => max_width.min(window_size.columns),
        None => window_size.columns,
    };

    let mut current_width = 0;

    let mut stdstream_stdout = StandardStream::stdout(ColorChoice::Always);
    for (lang, lines) in results {
        let result_width = ((lines as f32 / total_lines as f32) * columns as f32).round() as u16;
        if result_width <= 1 {
            // Blocks are too small now, just print "Other"
            let other_bg = Color::Rgb(128, 128, 128);
            let fg = foreground_color(other_bg);

            let name = "Other";
            let block = (0..(columns - current_width) as usize)
                .map(|i| name.chars().nth(i).unwrap_or(' '))
                .collect::<String>();

            stdstream_stdout
                .set_color(ColorSpec::new().set_bg(Some(other_bg)).set_fg(Some(fg)))
                .unwrap();
            print!("{}", block);

            break;
        }

        let fg = foreground_color(lang.color());

        let name = lang.to_string();
        let block = (0..result_width as usize)
            .map(|i| name.chars().nth(i).unwrap_or(' '))
            .collect::<String>();

        stdstream_stdout
            .set_color(
                ColorSpec::new()
                    .set_bg(Some(lang.color()))
                    .set_fg(Some(fg))
                    .set_bold(true),
            )
            .unwrap();
        print!("{}", block);

        current_width += result_width;
    }
    stdstream_stdout.reset().unwrap();
    println!();
}

pub fn print_results(results: Vec<(Language, usize)>, pwd: &Path, max_width: Option<u16>) {
    if results.is_empty() {
        return;
    }

    println!("Language distribution for {:?}:\n", pwd);

    const LANG_HEADER: &str = "Language";
    const LINES_HEADER: &str = "# Lines";

    let max_lines = results.first().unwrap().1;
    let lines_col_width = results
        .iter()
        .map(|(_, size)| size.human_count_bare().to_string().len())
        .max()
        .unwrap()
        .max(LANG_HEADER.len()) as u16;
    let lang_col_width = results
        .iter()
        .map(|(lang, _)| lang.to_string().len())
        .max()
        .unwrap()
        .max(LINES_HEADER.len()) as u16;

    let window_size = crossterm::terminal::window_size().expect("Couldn't get terminal size");
    let columns = match max_width {
        Some(max_width) => max_width.min(window_size.columns),
        None => window_size.columns,
    };

    let bar_col_width = columns - lines_col_width - lang_col_width - 2; // -2 for
    // padding
    let bar_col_start = lang_col_width + 1; // +1 for padding
    let lines_col_start = columns - lines_col_width + 1; // +1 for padding

    print!("{}", LANG_HEADER);
    std::io::stdout()
        .execute(MoveToColumn(lines_col_start))
        .unwrap();
    println!("{}", LINES_HEADER);

    let mut stdstream_stdout = StandardStream::stdout(ColorChoice::Always);
    for (lang, lines) in results {
        print!("{}", lang);
        std::io::stdout()
            .execute(MoveToColumn(bar_col_start))
            .unwrap();

        stdstream_stdout
            .set_color(ColorSpec::new().set_fg(Some(lang.color())))
            .unwrap();
        print!(
            "{}",
            vec![
                '█';
                ((lines as f32 / max_lines as f32) * bar_col_width as f32)
                    .round()
                    .max(1.) as usize
            ]
            .into_iter()
            .collect::<String>()
        );
        stdstream_stdout.reset().unwrap();
        std::io::stdout()
            .execute(MoveToColumn(lines_col_start))
            .unwrap();
        println!("{}", lines);
    }
}

fn foreground_color(background: Color) -> Color {
    let Color::Rgb(r, g, b) = background else {
        panic!("Cannot calculate forground color for non-RGB color");
    };

    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    const WHITE_LUMINANCE: f32 = 1.0;
    const BLACK_LUMINANCE: f32 = 0.0;
    let perceived_luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

    let white_foreground_contrast = contrast_ratio(WHITE_LUMINANCE, perceived_luminance);
    let black_foreground_contrast = contrast_ratio(BLACK_LUMINANCE, perceived_luminance);

    if black_foreground_contrast > white_foreground_contrast {
        Color::Rgb(0, 0, 0) // Color::Black is dependent on the terminal color scheme
    } else {
        Color::Rgb(255, 255, 255)
    }
}

fn contrast_ratio(l1: f32, l2: f32) -> f32 {
    if l1 > l2 {
        (l1 + 0.05) / (l2 + 0.05)
    } else {
        (l2 + 0.05) / (l1 + 0.05)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 255))]
    #[test_case(Color::Rgb(255, 255, 255), Color::Rgb(0, 0, 0))]
    #[test_case(Color::Rgb(255, 0, 0), Color::Rgb(0, 0, 0))]
    #[test_case(Color::Rgb(0, 255, 0), Color::Rgb(0, 0, 0))]
    #[test_case(Color::Rgb(0, 0, 255), Color::Rgb(255, 255, 255))]
    fn test_foreground_color(bg: Color, fg: Color) {
        let actual = foreground_color(bg);

        assert_eq!(actual, fg);
    }
}
