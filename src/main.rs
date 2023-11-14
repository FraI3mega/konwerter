use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::{io, process::exit};

const CHAR_TABLE: &str = "0123456789ABCDEF";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// W jakim trybie ma działać program
    #[arg(value_enum)]
    mode: Option<Mode>,
    #[arg(value_parser = clap::value_parser!(u32).range(2..))]
    /// Podstawa systemu obcego
    base: Option<u32>,
    // Liczby do zamiany
    values: Option<Vec<String>>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Konwersja z systemu obcego na dziesiętny
    Dec,
    /// Konwersja z systemu dziesiętnego na obcy
    Alien,
}

fn main() {
    //Tryb cli
    let cli = Cli::parse();

    match cli.mode {
        Some(Mode::Dec) => {
            for val in cli.values.unwrap_or_default() {
                println!(
                    "{} to w systemie dziesiętnym {}",
                    &val.to_string().magenta(),
                    to_dec(
                        val.trim().to_uppercase(),
                        cli.base.to_owned().unwrap_or_default()
                    )
                    .to_string()
                    .green()
                )
            }
            exit(0)
        }
        Some(Mode::Alien) => {
            for val in cli.values.unwrap_or_default() {
                println!(
                    "{} to {} w systemie o podstawie {}",
                    &val.to_string().magenta(),
                    to_alien(
                        val.trim().parse().unwrap_or_default(),
                        cli.base.to_owned().unwrap_or_default()
                    )
                    .to_string()
                    .green(),
                    &cli.base.unwrap_or_default().to_string().blue()
                )
            }
            exit(0)
        }
        _ => println!(
            "{}",
            "Brak argumentów. Uruchamiam tryb interaktywny."
                .bright_magenta()
                .blink()
        ),
    }

    //Tryb interaktywny
    loop {
        println!(
            "{}",
            "\n 1. Z dziesiętnego na obcy\n 2. Z obcego na dziesiętny\n 3. Wyjdź z programu"
                .bold()
                .cyan()
        );
        let selection = get_number("\nPodaj swój wybór");
        if selection == 1 {
            let inpt = get_number("\nPodaj liczbę");
            let base = get_base();
            println!(
                "\n{} w systemie o podstawie {} to {}",
                &inpt,
                &base,
                to_alien(inpt, base)
            )
        } else if selection == 2 {
            let inpt = get_number_string("\nPodaj liczbę");
            let base = get_base();
            println!("\nW systemie dziesiętnym to {}", to_dec(inpt, base))
        } else if selection == 3 {
            break;
        } else {
            println!("{}", "\nPodaj 1, 2 lub 3".bright_red())
        }
    }
}

fn get_number_string(prompt: &str) -> String {
    let mut input = String::new();
    let mut output = String::new();

    loop {
        println!("{}", prompt.blue());
        match io::stdin().read_line(&mut input) {
            Ok(_) => break,
            Err(_) => println!("{}", "Nie można odczytać wejścia".bright_red()),
        }
    }

    for character in input.trim().to_uppercase().chars() {
        if CHAR_TABLE.to_string().contains(character) {
            output += character.to_string().as_str()
        }
    }
    output
}

fn get_number(prompt: &str) -> u32 {
    let mut input = String::new();
    let mut output = String::new();

    loop {
        loop {
            println!("{}", prompt.blue());
            match io::stdin().read_line(&mut input) {
                Ok(_) => break,
                Err(_) => println!("{}", "Nie można odczytać wejścia".bright_red()),
            }
        }

        for character in input.trim().to_uppercase().chars() {
            if character.is_numeric() {
                output += character.to_string().as_str()
            }
        }

        match output.parse() {
            Ok(nmb) => return nmb,
            Err(_) => println!("{}", "Podaj liczbę".bright_red()),
        }
    }
}

fn get_base() -> u32 {
    let mut input = String::new();
    let mut output = String::new();

    loop {
        loop {
            println!("{}", "\nPodaj podstawę: ".blue());
            match io::stdin().read_line(&mut input) {
                Ok(_) => break,
                Err(_) => println!("{}", "Nie można odczytać wejścia".bright_red()),
            }
        }

        for character in input.trim().to_uppercase().chars() {
            if character.is_ascii_digit() {
                output += character.to_string().as_str()
            }
        }

        match output.parse() {
            Ok(nmb) => return nmb,
            Err(_) => println!("{}", "Błąd: Nie podano liczby".bright_red()),
        }
    }
}

fn to_dec(input: String, base: u32) -> u32 {
    let mut result: u32 = 0;
    for character in input.chars() {
        result = result * base + CHAR_TABLE.find(character).unwrap_or_default() as u32;
    }
    result
}

fn to_alien(mut input: u32, base: u32) -> String {
    let mut output = String::new();
    if input == 0 {
        return "0".to_string();
    }
    while input > 0 {
        output += CHAR_TABLE
            .chars()
            .nth((input % base) as usize)
            .unwrap_or_default()
            .to_string()
            .as_str();
        input /= base;
    }
    output.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::{to_alien, to_dec};

    #[test]
    fn edge_cases() {
        assert_eq!(to_alien(0, 16), "0");
        assert_eq!(to_alien(0, 8), "0");
        assert_eq!(to_alien(0, 2), "0");
        assert_eq!(to_dec(0.to_string(), 16), 0);
        assert_eq!(to_dec(0.to_string(), 8), 0);
        assert_eq!(to_dec(0.to_string(), 2), 0);
    }
    #[test]
    fn aln() {
        assert_eq!(to_alien(1234, 16), format!("{:x}", 1234).to_uppercase());
        assert_eq!(to_alien(5647, 16), format!("{:x}", 5647).to_uppercase());
        assert_eq!(to_alien(1, 16), format!("{:x}", 1).to_uppercase());
        assert_eq!(
            to_alien(9999999, 16),
            format!("{:x}", 9999999).to_uppercase()
        );
        assert_eq!(to_alien(1234, 10), "1234")
    }
    #[test]
    fn dec() {
        assert_eq!(to_dec("1234".to_string(), 10), 1234);
        assert_eq!(to_dec("AACB".to_string(), 16), 43723);
        assert_eq!(to_dec("".to_string(), 10), 0);
        assert_eq!(to_dec("F".to_string(), 16), 15);
        assert_eq!(to_dec("10".to_string(), 2), 2);
    }
}
