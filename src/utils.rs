use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Snippet {
    name: String,
    value: String,
}

fn data_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("could not resolve data directory");
    path.push("snip");
    path.push("snippets.json");
    path
}

fn load_snippets() -> Vec<Snippet> {
    let path = data_path();
    if !path.exists() {
        return Vec::new();
    }
    let text = fs::read_to_string(&path).expect("failed to read snippets file");
    serde_json::from_str(&text).unwrap_or_default()
}

fn write_snippets(snippets: &[Snippet]) {
    let path = data_path();
    fs::create_dir_all(path.parent().unwrap()).expect("failed to create data directory");
    let text = serde_json::to_string_pretty(snippets).unwrap();
    fs::write(&path, text).expect("failed to write snippets file");
}

pub fn help() {
    println!("snip - a local snippet store");
    println!();
    println!("USAGE:");
    println!("  snip <command> [args]");
    println!();
    println!("COMMANDS:");
    println!("  save <name> <value>   save a snippet");
    println!("  get <name>            print a snippet");
    println!("  rm <name>             delete a snippet");
    println!("  ls                    list all snippets");
    println!("  find <query>          fuzzy search by name");
    println!("  help                  show this message");
}

pub fn save(args: Vec<String>) {
    if args.len() == 4 {
        let mut snippets = load_snippets();

        let name = args[2].clone();

        if let Some(existing) = snippets.iter_mut().find(|s| s.name == name) {
            existing.value = args[3].clone();
        } else {
            snippets.push(Snippet {
                name,
                value: args[3].clone(),
            });
        }

        write_snippets(&snippets);

        println!("Saved '{}'.", args[2]);
    } else {
        println!("usage:    save <name> <value>   save a snippet");
    }
}

pub fn rm(args: Vec<String>) {
    if args.len() == 3 {
        let mut snippets = load_snippets();
        let name = &args[2];

        if let Some(i) = snippets.iter().position(|s| s.name == *name) {
            snippets.remove(i);
            write_snippets(&snippets);
            println!("Removed '{}'.", name);
        } else {
            println!("No snippet named '{}'.", name);
        }
    } else {
        println!("usage:    snip rm <name>");
    }
}

pub fn find(args: Vec<String>) {
    if args.len() == 3 {
        let query = args[2].to_lowercase();
        let snippets = load_snippets();
        let matches: Vec<&Snippet> = snippets
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&query))
            .collect();
        if matches.is_empty() {
            println!("No snippets matching '{}'.", args[2]);
        } else {
            for s in matches {
                println!("{} = {}", s.name, s.value);
            }
        }
    } else {
        println!("usage:    snip find <query>");
    }
}

pub fn ls() {
    let snippets = load_snippets();
    if snippets.is_empty() {
        println!("No snippets saved.");
        return;
    }
    for s in &snippets {
        println!("{} = {}", s.name, s.value);
    }
}
