# snip

A local CLI snippet store written in Rust. Save, retrieve, list, search, and delete named text snippets on your machine.

## Install

```
cargo install --path .
```

## Usage

```
snip save NAME VALUE
snip ls
snip find QUERY
snip rm NAME
snip help
```

## Commands

```
save NAME VALUE    Save a snippet. If NAME already exists, its value is overwritten.
ls                 List all saved snippets.
find QUERY         Search snippets by name. Matches any snippet whose name contains QUERY.
rm NAME            Delete a snippet by name.
help               Show usage information.
h                  Alias for help.
```

## Examples

```
snip save greet "hello world"
snip ls
snip find gr
snip rm greet
```

## Storage

Snippets are stored as JSON at the platform data directory.

```
Linux    $HOME/.local/share/snip/snippets.json
macOS    $HOME/Library/Application Support/snip/snippets.json
Windows  %APPDATA%\snip\snippets.json
```
