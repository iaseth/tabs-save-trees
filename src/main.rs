use std::{fs, io::{self, BufRead, Write}, path::Path};
use clap::{Arg, Command};
use walkdir::WalkDir;

fn main() {
	let matches = Command::new("tab_saver")
		.arg(Arg::new("paths")
			.required(true)
			.num_args(1..)
			.help("Files or directories to process"))
		.arg(Arg::new("--save")
			.long("save")
			.help("Modify files to use tabs instead of spaces"))
		.get_matches();

	let paths: Vec<&str> = matches.get_many::<String>("paths")
		.unwrap()
		.map(|s| s.as_str())
		.collect();
	let save = matches.contains_id("--save");

	for path in paths {
		process_path(path, save);
	}
}

fn process_path(path: &str, save: bool) {
	let metadata = fs::metadata(path).unwrap();
	if metadata.is_dir() {
		for entry in WalkDir::new(path) {
			let entry = entry.unwrap();
			if entry.path().is_file() {
				process_file(entry.path(), save);
			}
		}
	} else {
		process_file(Path::new(path), save);
	}
}

fn process_file(path: &Path, save: bool) {
	if let Ok(file) = fs::File::open(path) {
		let reader = io::BufReader::new(file);
		let mut lines = Vec::new();
		let mut total_saved = 0;
		let mut all_tabs = true;

		for line in reader.lines() {
			let line = line.unwrap();
			let trimmed_line = line.trim_end();
			let leading_spaces = count_leading_spaces(trimmed_line);
			let converted_line = convert_spaces_to_tabs(trimmed_line, leading_spaces);

			if leading_spaces > 0 {
				total_saved += leading_spaces / 4;
			}

			if leading_spaces > 0 && trimmed_line.starts_with(' ') {
				all_tabs = false;
			}

			lines.push(converted_line);
		}

		if all_tabs {
			println!("{} already uses only tabs.", path.display());
		} else {
			println!("{}: {} bytes can be saved", path.display(), total_saved);
		}

		if save && total_saved > 0 {
			let mut file = fs::File::create(path).unwrap();
			for line in &lines {
				writeln!(file, "{}", line).unwrap();
			}
		}
	}
}

fn count_leading_spaces(line: &str) -> usize {
	line.chars().take_while(|c| *c == ' ').count()
}

fn convert_spaces_to_tabs(line: &str, spaces: usize) -> String {
	let tab_count = spaces / 4;
	let remainder_spaces = spaces % 4;
	format!("{}{}", "\t".repeat(tab_count), " ".repeat(remainder_spaces)) + &line[spaces..]
}
