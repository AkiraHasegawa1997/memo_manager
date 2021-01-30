# memo_manager

Simple Memo Management Tool for a Survey of Academic Papers

## Usage

This program uses a JSON file for the management of memos.
If the program cannot find the file, it automatically creates a new JSON
file as a given name.

### Add memo

- `pdf_path` is an academic paper for which you want to memo.
- `memo_string` should be text, for example `"Well written. Great."`.

```console
memo_manager <json_path> <pdf_path> <memo_string>
```

### Read memo

- `pdf_path` is an academic paper for which you want to memo.

```console
memo_manager <json_path> <pdf_path>
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
