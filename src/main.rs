use clap::{clap_derive::ArgEnum, Parser};
use std::io::{BufRead, Error, Lines, StdinLock, StdoutLock, Write};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum WriteMode {
    Single,
    Multi,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum MatchMode {
    Find,
    Remove,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = Option::None)]
struct Args {
    /// The search text to match against.
    #[clap(value_parser)]
    search_text: String,

    /// Controls whether the search text will simply be matched or removed.
    #[clap(short, long, arg_enum, value_parser, default_value_t = MatchMode::Remove)]
    match_mode: MatchMode,

    /// Whether to write to a single line or multiple lines.
    #[clap(short, long, arg_enum, value_parser, default_value_t = WriteMode::Single)]
    write_mode: WriteMode,
}

fn main() {
    let args = Args::parse();
    let lines = get_lines_from_stdin();
    let matches = scan_buffer_for_matches(lines, &args.search_text, &args.match_mode).unwrap();

    write_matches_to_output(matches, &args.write_mode).unwrap();
}

fn get_lines_from_stdin() -> Lines<StdinLock<'static>> {
    std::io::stdin().lock().lines()
}

fn scan_buffer_for_matches(
    lines: Lines<StdinLock>,
    match_sequence: &str,
    match_mode: &MatchMode,
) -> Result<Vec<String>, Error> {
    let matching_lines = lines
        .into_iter()
        .filter_map(|line_result| match line_result {
            Ok(line) => {
                if let Some(matched_index) = line.find(match_sequence) {
                    return match match_mode {
                        MatchMode::Find => Option::Some(line),
                        MatchMode::Remove => Option::Some(
                            line.get(matched_index + match_sequence.len()..line.len())
                                .unwrap()
                                .to_string(),
                        ),
                    };
                }

                return Option::None;
            }
            Err(_) => Option::None,
        })
        .collect();

    Result::Ok(matching_lines)
}

fn write_matches_to_output(matches: Vec<String>, mode: &WriteMode) -> Result<(), Error> {
    let output = std::io::stdout().lock();

    match mode {
        WriteMode::Single => write_single_line(matches, output),
        WriteMode::Multi => write_multi_line(matches, output),
    }
}

fn write_single_line(matches: Vec<String>, mut output: StdoutLock) -> Result<(), Error> {
    let line = matches.into_iter().reduce(|a, b| a + " " + &b).unwrap();

    output.write(line.as_bytes()).unwrap();
    output.flush()
}

fn write_multi_line(matches: Vec<String>, mut output: StdoutLock) -> Result<(), Error> {
    matches.into_iter().for_each(|matched_line| {
        output.write((matched_line + "\n").as_bytes()).unwrap();
    });

    output.flush()
}
