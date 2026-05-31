use colored::Colorize;
use std::io::{self, BufRead, Write};
use std::process::Command;

const MAX_NAME: usize = 4;

fn print_message(speaker: &str, text: &str) {
    let indent = " ".repeat(MAX_NAME + 3);
    for (i, line) in text.lines().enumerate() {
        if i == 0 {
            let prefix = match speaker {
                "Gles" => format!("{} : ", "Gles".green().bold()),
                "Lavi" => format!("{} : ", "Lavi".cyan().bold()),
                "Loa"  => format!("{}  : ", "Loa".yellow().bold()),
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
    print!("{}", "Loa  : ".yellow().bold());
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
    println!("{}", "  Type your message and press Enter.".dimmed());
    println!("{}", "  Ctrl+C to exit.".dimmed());
    println!("{}", "─".repeat(40).dimmed());
    println!();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut turn = 0u32;

    loop {
        print!("{}", "Gles : ".green().bold());
        stdout.flush().unwrap();

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let input = input.trim_end_matches(['\n', '\r']);
        if input.is_empty() {
            continue;
        }

        // Lavi responds
        let lavi_reply = lavi_respond(input);
        print_message("Lavi", &lavi_reply);

        // Loa responds (with context of both Gles and Lavi)
        let prompt = format!("Gles : {}\nLavi : {}", input, lavi_reply);
        let loa_reply = ask_loa(&prompt, turn == 0);
        print_message("Loa", &loa_reply);
        println!();

        turn += 1;
    }
}
