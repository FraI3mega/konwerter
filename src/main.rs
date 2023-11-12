const CHAR_TABLE: &str = "0123456789ABCDEF";
use colored::Colorize;
use std::io;

// selection 1 - dec do alien
// selection 2 - alien to dec
// selection 3 - exit

fn main() {
    loop {
        println!(
            "{}",
            "\n1. Z dziesiętnego na obcy\n 2. Z obcego na dziesiętny\n 3. Wyjdź z programu"
                .bold()
                .cyan()
        );
        let selection = parse_to_nmb(input("\nPodaj swój wybór"));
        if selection == 1 {
            let inpt = parse_to_nmb(input("\nPodaj liczbę"));
            let base = get_base();
            println!(
                "\n{} w systemie o podstawie {} to {}",
                &inpt,
                &base,
                to_alien(inpt, base)
            )
        } else if selection == 2 {
            let inpt = input("\nPodaj liczbę");
            let base = get_base();
            println!("\nW systemie dziesiętnym to {}", to_dec(inpt, base))
        } else if selection == 3 {
            break;
        } else {
            println!("{}", "Podaj 1, 2 lub 3".bright_red())
        }
    }
}

fn input(prompt: &str) -> String {
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

fn parse_to_nmb(input: String) -> i32 {
    input.trim().parse().expect("Podaj liczbę")
}

fn get_base() -> i32 {
    let mut inpt: i32;
    loop {
        inpt = parse_to_nmb(input("\nPodaj podstawę"));
        if inpt >= 2 {
            break;
        }
        println!("{}", "Podstawa musi być większa lub równa 2".bright_red())
    }
    inpt
}

fn to_dec(input: String, base: i32) -> i32 {
    let mut result: i32 = 0;
    for character in input.chars() {
        result = result * base + CHAR_TABLE.find(character).unwrap_or_default() as i32;
    }
    result
}

fn to_alien(mut input: i32, base: i32) -> String {
    let mut output = String::new();
    if input == 0 {output += "0"}
    while input > 0 {
        output += CHAR_TABLE.chars().nth((input % base) as usize).unwrap_or_default().to_string().as_str();
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
        assert_eq!(to_alien(9999999, 16), format!("{:x}", 9999999).to_uppercase());
        assert_eq!(to_alien(1234, 10), "1234")
    }
}