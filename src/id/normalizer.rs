//! Text normalization for Indonesian
//!
//! Converts numbers, currency, dates, etc. to their spoken form.

use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d+)").unwrap()
});

static CURRENCY_IDR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Rp\.?\s*(\d+(?:[.,]\d+)?)").unwrap()
});

static CURRENCY_USD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\$\s*(\d+(?:[.,]\d+)?)").unwrap()
});

/// Normalize Indonesian text
pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();

    // Normalize currency
    result = normalize_currency_idr(&result);
    result = normalize_currency_usd(&result);

    // Normalize plain numbers
    result = normalize_numbers(&result);

    result
}

fn normalize_currency_idr(text: &str) -> String {
    CURRENCY_IDR.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace('.', "").replace(',', ".");
        let num: u64 = amount.parse::<f64>().unwrap_or(0.0) as u64;
        format!("{} rupiah", number_to_indonesian(num))
    }).to_string()
}

fn normalize_currency_usd(text: &str) -> String {
    CURRENCY_USD.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let cents = ((num - whole as f64) * 100.0).round() as u64;

        if cents > 0 {
            format!("{} dolar {} sen", number_to_indonesian(whole), number_to_indonesian(cents))
        } else {
            format!("{} dolar", number_to_indonesian(whole))
        }
    }).to_string()
}

fn normalize_numbers(text: &str) -> String {
    NUMBER_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_indonesian(num)
    }).to_string()
}

/// Convert a number to Indonesian words
pub fn number_to_indonesian(n: u64) -> String {
    match n {
        0 => "nol".to_string(),
        1 => "satu".to_string(),
        2 => "dua".to_string(),
        3 => "tiga".to_string(),
        4 => "empat".to_string(),
        5 => "lima".to_string(),
        6 => "enam".to_string(),
        7 => "tujuh".to_string(),
        8 => "delapan".to_string(),
        9 => "sembilan".to_string(),
        10 => "sepuluh".to_string(),
        11 => "sebelas".to_string(),
        12..=19 => format!("{} belas", number_to_indonesian(n - 10)),
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            if ones == 0 {
                format!("{} puluh", number_to_indonesian(tens))
            } else {
                format!("{} puluh {}", number_to_indonesian(tens), number_to_indonesian(ones))
            }
        }
        100 => "seratus".to_string(),
        101..=199 => format!("seratus {}", number_to_indonesian(n - 100)),
        200..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} ratus", number_to_indonesian(hundreds))
            } else {
                format!("{} ratus {}", number_to_indonesian(hundreds), number_to_indonesian(remainder))
            }
        }
        1000 => "seribu".to_string(),
        1001..=1999 => format!("seribu {}", number_to_indonesian(n - 1000)),
        2000..=999999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            if remainder == 0 {
                format!("{} ribu", number_to_indonesian(thousands))
            } else {
                format!("{} ribu {}", number_to_indonesian(thousands), number_to_indonesian(remainder))
            }
        }
        1000000 => "satu juta".to_string(),
        1000001..=999999999 => {
            let millions = n / 1000000;
            let remainder = n % 1000000;
            if remainder == 0 {
                format!("{} juta", number_to_indonesian(millions))
            } else {
                format!("{} juta {}", number_to_indonesian(millions), number_to_indonesian(remainder))
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
        assert_eq!(number_to_indonesian(0), "nol");
        assert_eq!(number_to_indonesian(1), "satu");
        assert_eq!(number_to_indonesian(11), "sebelas");
        assert_eq!(number_to_indonesian(21), "dua puluh satu");
        assert_eq!(number_to_indonesian(100), "seratus");
        assert_eq!(number_to_indonesian(1000), "seribu");
    }

    #[test]
    fn test_currency_idr() {
        let result = normalize("Rp 50000");
        assert!(result.contains("lima puluh ribu rupiah"));
    }

    #[test]
    fn test_currency_usd() {
        let result = normalize("$100");
        assert!(result.contains("seratus dolar"));
    }
}
