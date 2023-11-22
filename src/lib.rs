pub mod input {
    use colored::Colorize;
    use std::io;
    const CHAR_TABLE: &str = "0123456789ABCDEF";

    pub fn get_number_string(prompt: &str) -> String {
        let mut input = String::new();
        let mut output = String::new();

        loop {
            println!("{}", prompt.blue());
            match io::stdin().read_line(&mut input) {
                Ok(_) => break,
                Err(_) => eprintln!("{}", "Nie można odczytać wejścia".bright_red()),
            }
        }

        for character in input.trim().to_uppercase().chars() {
            if CHAR_TABLE.to_string().contains(character) {
                output += character.to_string().as_str();
            }
        }
        output
    }

    pub fn get_number(prompt: &str) -> u32 {
        let mut input = String::new();
        let mut output = String::new();

        loop {
            loop {
                println!("{}", prompt.blue());
                match io::stdin().read_line(&mut input) {
                    Ok(_) => break,
                    Err(_) => eprintln!("{}", "Nie można odczytać wejścia".bright_red()),
                }
            }

            for character in input.trim().to_uppercase().chars() {
                if character.is_numeric() {
                    output += character.to_string().as_str();
                }
            }

            match output.parse() {
                Ok(nmb) => return nmb,
                Err(_) => eprintln!("{}", "Podaj liczbę".bright_red()),
            }
        }
    }

    pub fn get_base() -> u32 {
        let mut input = String::new();
        let mut output = String::new();

        loop {
            loop {
                println!("{}", "\nPodaj podstawę: ".blue());
                match io::stdin().read_line(&mut input) {
                    Ok(_) => break,
                    Err(_) => eprintln!("{}", "Nie można odczytać wejścia".bright_red()),
                }
            }

            for character in input.trim().to_uppercase().chars() {
                if character.is_ascii_digit() {
                    output += character.to_string().as_str();
                }
            }

            match output.parse() {
                Ok(nmb) => return nmb,
                Err(_) => eprintln!("{}", "Błąd: Nie podano liczby".bright_red()),
            }
        }
    }
}

pub mod convert {
    const CHAR_TABLE: &str = "0123456789ABCDEF";

    pub fn to_dec(input: String, base: u32) -> u32 {
        let mut result: u32 = 0;
        for character in input.chars() {
            result = result * base + CHAR_TABLE.find(character).unwrap_or_default() as u32;
        }
        result
    }

    pub fn to_alien(mut input: u32, base: u32) -> String {
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
}

#[cfg(test)]
mod tests {
    use crate::convert::{to_alien, to_dec};

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
