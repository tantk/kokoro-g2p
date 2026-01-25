//! Text normalization for Italian
//!
//! Converts numbers, currency, dates, etc. to their spoken form.

use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d+)").unwrap()
});

static CURRENCY_EUR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"€\s*(\d+(?:[.,]\d+)?)").unwrap()
});

static CURRENCY_USD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\$\s*(\d+(?:[.,]\d+)?)").unwrap()
});

static ORDINAL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d+)[°ºª]").unwrap()
});

/// Normalize Italian text
pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();

    // Normalize ordinals (1° → primo)
    result = normalize_ordinals(&result);

    // Normalize currency
    result = normalize_currency_eur(&result);
    result = normalize_currency_usd(&result);

    // Normalize plain numbers
    result = normalize_numbers(&result);

    result
}

fn normalize_ordinals(text: &str) -> String {
    ORDINAL_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_ordinal(num)
    }).to_string()
}

fn normalize_currency_eur(text: &str) -> String {
    CURRENCY_EUR.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace('.', "").replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let cents = ((num - whole as f64) * 100.0).round() as u64;

        if cents > 0 {
            format!("{} euro e {} centesimi", number_to_italian(whole), number_to_italian(cents))
        } else {
            format!("{} euro", number_to_italian(whole))
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
            format!("{} dollari e {} centesimi", number_to_italian(whole), number_to_italian(cents))
        } else {
            format!("{} dollari", number_to_italian(whole))
        }
    }).to_string()
}

fn normalize_numbers(text: &str) -> String {
    NUMBER_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_italian(num)
    }).to_string()
}

/// Convert a number to Italian words
pub fn number_to_italian(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "uno".to_string(),
        2 => "due".to_string(),
        3 => "tre".to_string(),
        4 => "quattro".to_string(),
        5 => "cinque".to_string(),
        6 => "sei".to_string(),
        7 => "sette".to_string(),
        8 => "otto".to_string(),
        9 => "nove".to_string(),
        10 => "dieci".to_string(),
        11 => "undici".to_string(),
        12 => "dodici".to_string(),
        13 => "tredici".to_string(),
        14 => "quattordici".to_string(),
        15 => "quindici".to_string(),
        16 => "sedici".to_string(),
        17 => "diciassette".to_string(),
        18 => "diciotto".to_string(),
        19 => "diciannove".to_string(),
        20 => "venti".to_string(),
        21 => "ventuno".to_string(),
        22..=27 => format!("venti{}", number_to_italian(n - 20)),
        28 => "ventotto".to_string(),
        29 => "ventinove".to_string(),
        30..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            let tens_word = match tens {
                3 => "trenta",
                4 => "quaranta",
                5 => "cinquanta",
                6 => "sessanta",
                7 => "settanta",
                8 => "ottanta",
                9 => "novanta",
                _ => "",
            };
            if ones == 0 {
                tens_word.to_string()
            } else if ones == 1 || ones == 8 {
                // Drop final vowel before uno/otto
                let base = &tens_word[..tens_word.len() - 1];
                format!("{}{}", base, number_to_italian(ones))
            } else {
                format!("{}{}", tens_word, number_to_italian(ones))
            }
        }
        100 => "cento".to_string(),
        101..=199 => format!("cento{}", number_to_italian(n - 100)),
        200..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{}cento", number_to_italian(hundreds))
            } else {
                format!("{}cento{}", number_to_italian(hundreds), number_to_italian(remainder))
            }
        }
        1000 => "mille".to_string(),
        1001..=1999 => format!("mille{}", number_to_italian(n - 1000)),
        2000..=999999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            if remainder == 0 {
                format!("{}mila", number_to_italian(thousands))
            } else {
                format!("{}mila{}", number_to_italian(thousands), number_to_italian(remainder))
            }
        }
        1000000 => "un milione".to_string(),
        1000001..=1999999 => format!("un milione {}", number_to_italian(n - 1000000)),
        2000000..=999999999 => {
            let millions = n / 1000000;
            let remainder = n % 1000000;
            if remainder == 0 {
                format!("{} milioni", number_to_italian(millions))
            } else {
                format!("{} milioni {}", number_to_italian(millions), number_to_italian(remainder))
            }
        }
        _ => n.to_string(),
    }
}

fn number_to_ordinal(n: u64) -> String {
    match n {
        1 => "primo".to_string(),
        2 => "secondo".to_string(),
        3 => "terzo".to_string(),
        4 => "quarto".to_string(),
        5 => "quinto".to_string(),
        6 => "sesto".to_string(),
        7 => "settimo".to_string(),
        8 => "ottavo".to_string(),
        9 => "nono".to_string(),
        10 => "decimo".to_string(),
        11 => "undicesimo".to_string(),
        12 => "dodicesimo".to_string(),
        _ => format!("{}esimo", number_to_italian(n)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_numbers() {
        assert_eq!(number_to_italian(0), "zero");
        assert_eq!(number_to_italian(1), "uno");
        assert_eq!(number_to_italian(21), "ventuno");
        assert_eq!(number_to_italian(28), "ventotto");
        assert_eq!(number_to_italian(100), "cento");
        assert_eq!(number_to_italian(1000), "mille");
        assert_eq!(number_to_italian(2000), "duemila");
    }

    #[test]
    fn test_currency_eur() {
        let result = normalize("€50");
        assert!(result.contains("cinquanta euro"));
    }

    #[test]
    fn test_ordinals() {
        let result = normalize("1° posto");
        assert!(result.contains("primo"));
    }
}
