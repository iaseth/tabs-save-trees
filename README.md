# Tabs Save Trees (tst)

[![GitHub](https://img.shields.io/badge/GitHub-iaseth/tabs--save--trees-blue?style=flat-square)](https://github.com/iaseth/tabs-save-trees)

`tst` (Tabs Save Trees) is a lightweight command-line tool that analyzes text files to determine how much space can be saved by converting leading spaces to tabs. It also removes trailing spaces and provides an option to modify files in-place.

## Features
- Analyzes text files to estimate space savings from converting leading spaces to tabs.
- Reports whether a file already uses tabs consistently.
- Removes trailing spaces from each line.
- Supports directories: recursively processes all text files.
- `--save` flag modifies files to replace spaces with tabs where applicable.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/) installed, then clone and build the project:

```sh
git clone https://github.com/iaseth/tabs-save-trees.git
cd tabs-save-trees
cargo build --release
```

The compiled executable will be located at `target/release/tst`.

## Usage

### Analyze files/directories
```sh
tst file1.txt dir1 file2.txt
```
This will check the given files and directories for leading spaces and report potential space savings.

### Modify files to replace spaces with tabs
```sh
tst file1.txt dir1 --save
```
This will replace leading spaces with tabs (assuming a 4-space tab width) and remove trailing spaces from each line.

### Example Output
```
file1.txt: 120 bytes can be saved
file2.txt already uses only tabs.
dir1/file3.txt: 300 bytes can be saved
```

## License
MIT License. See [LICENSE](https://github.com/iaseth/tabs-save-trees/blob/main/LICENSE) for details.

