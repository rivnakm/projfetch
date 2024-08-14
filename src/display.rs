use std::path::Path;

use crossterm::{cursor::MoveToColumn, ExecutableCommand};
use human_repr::HumanCount;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::lang::Language;

pub fn print_results(results: Vec<(Language, usize)>, pwd: &Path) {
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
    let bar_col_width = window_size.columns - lines_col_width - lang_col_width - 2; // -2 for
                                                                                    // padding
    let bar_col_start = lang_col_width + 1; // +1 for padding
    let lines_col_start = window_size.columns - lines_col_width + 1; // +1 for padding

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
