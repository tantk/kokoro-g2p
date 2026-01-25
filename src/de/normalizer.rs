//! Text normalization for German

use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
static CURRENCY_EUR: Lazy<Regex> = Lazy::new(|| Regex::new(r"€\s*(\d+(?:[.,]\d+)?)").unwrap());

pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();
    result = normalize_currency_eur(&result);
    result = normalize_numbers(&result);
    result
}

fn normalize_currency_eur(text: &str) -> String {
    CURRENCY_EUR.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace('.', "").replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let cents = ((num - whole as f64) * 100.0).round() as u64;

        if cents > 0 {
            format!("{} Euro {} Cent", number_to_german(whole), number_to_german(cents))
        } else {
            format!("{} Euro", number_to_german(whole))
        }
    }).to_string()
}

fn normalize_numbers(text: &str) -> String {
    NUMBER_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_german(num)
    }).to_string()
}

pub fn number_to_german(n: u64) -> String {
    match n {
        0 => "null".to_string(),
        1 => "eins".to_string(),
        2 => "zwei".to_string(),
        3 => "drei".to_string(),
        4 => "vier".to_string(),
        5 => "fünf".to_string(),
        6 => "sechs".to_string(),
        7 => "sieben".to_string(),
        8 => "acht".to_string(),
        9 => "neun".to_string(),
        10 => "zehn".to_string(),
        11 => "elf".to_string(),
        12 => "zwölf".to_string(),
        13 => "dreizehn".to_string(),
        14 => "vierzehn".to_string(),
        15 => "fünfzehn".to_string(),
        16 => "sechzehn".to_string(),
        17 => "siebzehn".to_string(),
        18 => "achtzehn".to_string(),
        19 => "neunzehn".to_string(),
        20 => "zwanzig".to_string(),
        21 => "einundzwanzig".to_string(),
        22..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            let tens_word = match tens {
                2 => "zwanzig",
                3 => "dreißig",
                4 => "vierzig",
                5 => "fünfzig",
                6 => "sechzig",
                7 => "siebzig",
                8 => "achtzig",
                9 => "neunzig",
                _ => "",
            };
            if ones == 0 {
                tens_word.to_string()
            } else {
                // German: ones-und-tens (einundzwanzig = one-and-twenty)
                let ones_word = match ones {
                    1 => "ein",
                    2 => "zwei",
                    3 => "drei",
                    4 => "vier",
                    5 => "fünf",
                    6 => "sechs",
                    7 => "sieben",
                    8 => "acht",
                    9 => "neun",
                    _ => "",
                };
                format!("{}und{}", ones_word, tens_word)
            }
        }
        100 => "hundert".to_string(),
        101..=199 => format!("hundert{}", number_to_german(n - 100)),
        200..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{}hundert", number_to_german(hundreds))
            } else {
                format!("{}hundert{}", number_to_german(hundreds), number_to_german(remainder))
            }
        }
        1000 => "tausend".to_string(),
        1001..=999999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            let thousands_part = if thousands == 1 {
                "tausend".to_string()
            } else {
                format!("{}tausend", number_to_german(thousands))
            };
            if remainder == 0 {
                thousands_part
            } else {
                format!("{}{}", thousands_part, number_to_german(remainder))
            }
        }
        1000000 => "eine Million".to_string(),
        1000001..=999999999 => {
            let millions = n / 1000000;
            let remainder = n % 1000000;
            let millions_part = if millions == 1 {
                "eine Million".to_string()
            } else {
                format!("{} Millionen", number_to_german(millions))
            };
            if remainder == 0 {
                millions_part
            } else {
                format!("{} {}", millions_part, number_to_german(remainder))
            }
        }
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_numbers() {
        assert_eq!(number_to_german(0), "null");
        assert_eq!(number_to_german(1), "eins");
        assert_eq!(number_to_german(21), "einundzwanzig");
        assert_eq!(number_to_german(100), "hundert");
    }

    #[test]
    fn test_currency() {
        let result = normalize("€50");
        assert!(result.contains("fünfzig Euro"));
    }
}
