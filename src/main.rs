use colored::Colorize;
use std::io::{self, BufRead, Write};

const MAX_NAME: usize = 4; // "Gles" / "Lavi" = 4 chars, "Loa" padded to 4

fn print_message(speaker: &str, text: &str) {
    let indent = " ".repeat(MAX_NAME + 3); // "Gles : " = 7 chars
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

        let lavi_reply = lavi_respond(input);
        print_message("Lavi", &lavi_reply);
        println!();
    }
}
