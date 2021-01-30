# memo_manager

Simple Memo Management Tool for a Survey of Academic Papers

## Usage

This program uses a JSON file for the management of memos.
If the program cannot find the file, it automatically creates a new JSON
file as a given name.
`memo_string`, shown below, is String, for example `"Well written. Great."`.

```console
memo_manager <json_path> <pdf_path> <memo_string>
```

## Build

You could build this project with cargo.
The binary will be placed in the `memo_manager/target`.

```console
git clone https://github.com/AkiraHasegawa1997/memo_manager
cd memo_manager
cargo build --release
# cp ./target/release/memo_manager ~/wherever-you-want-to-place
```
