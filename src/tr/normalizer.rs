//! Text normalization for Turkish
//!
//! Converts numbers, currency, dates, etc. to their spoken form.

use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d+)").unwrap()
});

static CURRENCY_TRY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"₺\s*(\d+(?:[.,]\d+)?)|(\d+(?:[.,]\d+)?)\s*TL").unwrap()
});

static CURRENCY_USD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\$\s*(\d+(?:[.,]\d+)?)").unwrap()
});

static CURRENCY_EUR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"€\s*(\d+(?:[.,]\d+)?)").unwrap()
});

/// Normalize Turkish text
pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();

    // Normalize currency
    result = normalize_currency_try(&result);
    result = normalize_currency_usd(&result);
    result = normalize_currency_eur(&result);

    // Normalize plain numbers
    result = normalize_numbers(&result);

    result
}

fn normalize_currency_try(text: &str) -> String {
    CURRENCY_TRY.replace_all(text, |caps: &regex::Captures| {
        let amount_str = caps.get(1).or(caps.get(2)).map(|m| m.as_str()).unwrap_or("0");
        let amount = amount_str.replace('.', "").replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let kurus = ((num - whole as f64) * 100.0).round() as u64;

        if kurus > 0 {
            format!("{} lira {} kuruş", number_to_turkish(whole), number_to_turkish(kurus))
        } else {
            format!("{} lira", number_to_turkish(whole))
        }
    }).to_string()
}

fn normalize_currency_usd(text: &str) -> String {
    CURRENCY_USD.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let cents = ((num - whole as f64) * 100.0).round() as u64;

        if cents > 0 {
            format!("{} dolar {} sent", number_to_turkish(whole), number_to_turkish(cents))
        } else {
            format!("{} dolar", number_to_turkish(whole))
        }
    }).to_string()
}

fn normalize_currency_eur(text: &str) -> String {
    CURRENCY_EUR.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let cents = ((num - whole as f64) * 100.0).round() as u64;

        if cents > 0 {
            format!("{} avro {} sent", number_to_turkish(whole), number_to_turkish(cents))
        } else {
            format!("{} avro", number_to_turkish(whole))
        }
    }).to_string()
}

fn normalize_numbers(text: &str) -> String {
    NUMBER_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_turkish(num)
    }).to_string()
}

/// Convert a number to Turkish words
pub fn number_to_turkish(n: u64) -> String {
    match n {
        0 => "sıfır".to_string(),
        1 => "bir".to_string(),
        2 => "iki".to_string(),
        3 => "üç".to_string(),
        4 => "dört".to_string(),
        5 => "beş".to_string(),
        6 => "altı".to_string(),
        7 => "yedi".to_string(),
        8 => "sekiz".to_string(),
        9 => "dokuz".to_string(),
        10 => "on".to_string(),
        11..=19 => format!("on {}", number_to_turkish(n - 10)),
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            let tens_word = match tens {
                2 => "yirmi",
                3 => "otuz",
                4 => "kırk",
                5 => "elli",
                6 => "altmış",
                7 => "yetmiş",
                8 => "seksen",
                9 => "doksan",
                _ => "",
            };
            if ones == 0 {
                tens_word.to_string()
            } else {
                format!("{} {}", tens_word, number_to_turkish(ones))
            }
        }
        100 => "yüz".to_string(),
        101..=199 => format!("yüz {}", number_to_turkish(n - 100)),
        200..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} yüz", number_to_turkish(hundreds))
            } else {
                format!("{} yüz {}", number_to_turkish(hundreds), number_to_turkish(remainder))
            }
        }
        1000 => "bin".to_string(),
        1001..=1999 => format!("bin {}", number_to_turkish(n - 1000)),
        2000..=999999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            if remainder == 0 {
                format!("{} bin", number_to_turkish(thousands))
            } else {
                format!("{} bin {}", number_to_turkish(thousands), number_to_turkish(remainder))
            }
        }
        1000000 => "bir milyon".to_string(),
        1000001..=999999999 => {
            let millions = n / 1000000;
            let remainder = n % 1000000;
            if remainder == 0 {
                format!("{} milyon", number_to_turkish(millions))
            } else {
                format!("{} milyon {}", number_to_turkish(millions), number_to_turkish(remainder))
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
        assert_eq!(number_to_turkish(0), "sıfır");
        assert_eq!(number_to_turkish(1), "bir");
        assert_eq!(number_to_turkish(11), "on bir");
        assert_eq!(number_to_turkish(21), "yirmi bir");
        assert_eq!(number_to_turkish(100), "yüz");
        assert_eq!(number_to_turkish(1000), "bin");
    }

    #[test]
    fn test_currency_try() {
        let result = normalize("₺50");
        assert!(result.contains("elli lira"));
    }

    #[test]
    fn test_currency_tl() {
        let result = normalize("100 TL");
        assert!(result.contains("yüz lira"));
    }
}
