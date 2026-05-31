use colored::Colorize;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;
use chrono::Local;
use serde::{Deserialize, Serialize};

// ── ES params ────────────────────────────────────────────────
#[derive(Serialize, Deserialize)]
struct LaviParams {
    responses: Vec<String>,
    weights:   Vec<f64>,
}

impl Default for LaviParams {
    fn default() -> Self {
        LaviParams {
            responses: vec![
                "Hello.".into(),
                "Hi.".into(),
                "I see.".into(),
                "Tell me more.".into(),
                "Interesting.".into(),
            ],
            weights: vec![1.0, 1.0, 1.0, 1.0, 1.0],
        }
    }
}

fn params_path() -> PathBuf { PathBuf::from("lavi_params.json") }

fn load_params() -> LaviParams {
    if let Ok(data) = std::fs::read_to_string(params_path()) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        LaviParams::default()
    }
}

fn save_params(p: &LaviParams) {
    if let Ok(json) = serde_json::to_string_pretty(p) {
        let _ = std::fs::write(params_path(), json);
    }
}

// weighted random selection
fn select_response(p: &LaviParams) -> (usize, String) {
    let total: f64 = p.weights.iter().sum();
    // simple LCG-style seed from system time
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(42) as f64;
    let r = (seed % 1_000_000.0) / 1_000_000.0 * total;
    let mut cum = 0.0;
    for (i, w) in p.weights.iter().enumerate() {
        cum += w;
        if r <= cum {
            return (i, p.responses[i].clone());
        }
    }
    let last = p.responses.len() - 1;
    (last, p.responses[last].clone())
}

// ES update: reward/penalise the chosen response based on Loa reply length
fn es_update(p: &mut LaviParams, chosen: usize, loa_reply: &str) {
    let score = loa_reply.chars().count();
    let delta = if score >= 30 { 0.1 } else { -0.05 };
    p.weights[chosen] = (p.weights[chosen] + delta).max(0.1);
    save_params(p);
}

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

fn lavi_respond(_input: &str, params: &LaviParams) -> (usize, String) {
    select_response(params)
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
    let mut params = load_params();

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

        // Lavi responds (ES weighted selection)
        let (chosen, lavi_reply) = lavi_respond(&input, &params);
        print_message("Lavi", &lavi_reply);

        // Loa responds
        let prompt = format!("Gles : {}\nLavi : {}", input, lavi_reply);
        let loa_reply = ask_loa(&prompt, turn == 0);
        print_message("Loa", &loa_reply);
        println!();

        // ES update
        es_update(&mut params, chosen, &loa_reply);

        // save to memory
        memory.conversations.push(Turn {
            timestamp: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
            gles: input.to_string(),
            lavi: lavi_reply.clone(),
        });
        save_memory(&memory);

        turn += 1;
    }
}
