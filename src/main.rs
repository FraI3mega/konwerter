use clap::{Parser, ValueEnum};
use colored::Colorize;
use konwerter::convert::{to_alien, to_dec};
use konwerter::input::{get_base, get_number, get_number_string};
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// W jakim trybie ma działać program
    #[arg(value_enum)]
    mode: Option<Mode>,
    #[arg(value_parser = clap::value_parser!(u32).range(2..))]
    /// Podstawa systemu obcego
    base: Option<u32>,
    /// Liczby do zamiany
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
                    to_dec(val.trim().to_uppercase(), cli.base.unwrap_or_default())
                        .to_string()
                        .green()
                );
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
                        cli.base.unwrap_or_default()
                    )
                    .to_string()
                    .green(),
                    &cli.base.unwrap_or_default().to_string().blue()
                );
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
            );
        } else if selection == 2 {
            let inpt = get_number_string("\nPodaj liczbę");
            let base = get_base();
            println!("\nW systemie dziesiętnym to {}", to_dec(inpt, base));
        } else {
            if selection == 3 {
                exit(0);
            }
            eprintln!("{}", "\nPodaj 1, 2 lub 3".bright_red());
        }
    }
}
