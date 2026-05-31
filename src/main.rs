use colored::Colorize;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Turn {
    timestamp: String,
    gles: String,
    lavi: String,
}

#[derive(Serialize, Deserialize, Default)]
struct Memory {
    conversations: Vec<Turn>,
}

fn memory_path() -> PathBuf {
    PathBuf::from("lavi_memory.json")
}

fn load_memory() -> Memory {
    let path = memory_path();
    if let Ok(data) = std::fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Memory::default()
    }
}

fn save_memory(memory: &Memory) {
    let path = memory_path();
    if let Ok(json) = serde_json::to_string_pretty(memory) {
        let _ = std::fs::write(&path, json);
    }
}

const MAX_NAME: usize = 4;

fn print_message(speaker: &str, text: &str) {
    let indent = " ".repeat(MAX_NAME + 3);
    for (i, line) in text.lines().enumerate() {
        if i == 0 {
            let prefix = match speaker {
                "Gles" => format!("{} : ", "Gles".green().bold()),
                "Lavi" => format!("{} : ", "Lavi".yellow().bold()),
                "Loa"  => format!("{}  : ", "Loa".purple().bold()),
                _      => format!("{} : ", speaker),
            };
            println!("{}{}", prefix, line);
        } else {
            println!("{}{}", indent, line);
        }
    }
}

fn lavi_respond(_input: &str) -> String {
    "Hello.".to_string()
}

fn ask_loa(prompt: &str, is_first: bool) -> String {
    print!("{}", "Loa  : ".purple().bold());
    io::stdout().flush().unwrap();

    let system = "You are Loa, a participant in a 3-party chat with Gles (human) and Lavi (a growing AI). Respond naturally and briefly as Loa. No meta-commentary about the system.";

    let mut args = vec![];
    if !is_first {
        args.push("-c");
    }
    args.push("-p");
    args.push(prompt);
    args.push("--append-system-prompt");
    args.push(system);

    let result = Command::new("claude")
        .args(&args)
        .output();

    match result {
        Ok(output) => {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            // clear the "Loa  : " we already printed, then print_message handles it
            print!("\r{}\r", " ".repeat(60));
            io::stdout().flush().unwrap();
            text
        }
        Err(_) => "(claude not available)".to_string(),
    }
}

fn main() {
    println!("{}", "─".repeat(40).dimmed());
    println!("{}", "  Cataa  —  chat with Lavi and Loa".bold());
    println!("{}", "─".repeat(40).dimmed());
    println!("{}", "  Type your message. Blank line to send. Ctrl+C to exit.".dimmed());
    println!("{}", "─".repeat(40).dimmed());
    println!();

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut turn = 0u32;
    let mut memory = load_memory();

    loop {
        print!("{}", "Gles : ".green().bold());
        stdout.flush().unwrap();

        // collect lines until blank line
        let mut lines: Vec<String> = Vec::new();
        loop {
            let mut line = String::new();
            match stdin_lock.read_line(&mut line) {
                Ok(0) => return,
                Ok(_) => {}
                Err(_) => return,
            }
            let trimmed = line.trim_end_matches(['\n', '\r']).to_string();
            if trimmed.is_empty() {
                break; // blank line = send
            }
            if !lines.is_empty() {
                print!("{}", " ".repeat(MAX_NAME + 3));
                stdout.flush().unwrap();
            }
            lines.push(trimmed);
        }

        if lines.is_empty() {
            continue;
        }
        let input = lines.join("\n");

        // Lavi responds
        let lavi_reply = lavi_respond(&input);
        print_message("Lavi", &lavi_reply);

        // save to memory
        memory.conversations.push(Turn {
            timestamp: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
            gles: input.to_string(),
            lavi: lavi_reply.clone(),
        });
        save_memory(&memory);

        // Loa responds
        let prompt = format!("Gles : {}\nLavi : {}", input, lavi_reply);
        let loa_reply = ask_loa(&prompt, turn == 0);
        print_message("Loa", &loa_reply);
        println!();

        turn += 1;
    }
}
