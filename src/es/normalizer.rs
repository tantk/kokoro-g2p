//! Text normalization for Spanish
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
    Regex::new(r"(\d+)[ºª]").unwrap()
});

/// Normalize Spanish text
pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();

    // Normalize ordinals first (1º → primero)
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
        let amount = caps[1].replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let cents = ((num - whole as f64) * 100.0).round() as u64;

        if cents > 0 {
            format!("{} euros con {} céntimos", number_to_spanish(whole), number_to_spanish(cents))
        } else {
            format!("{} euros", number_to_spanish(whole))
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
            format!("{} dólares con {} centavos", number_to_spanish(whole), number_to_spanish(cents))
        } else {
            format!("{} dólares", number_to_spanish(whole))
        }
    }).to_string()
}

fn normalize_numbers(text: &str) -> String {
    NUMBER_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_spanish(num)
    }).to_string()
}

/// Convert a number to Spanish words
pub fn number_to_spanish(n: u64) -> String {
    match n {
        0 => "cero".to_string(),
        1 => "uno".to_string(),
        2 => "dos".to_string(),
        3 => "tres".to_string(),
        4 => "cuatro".to_string(),
        5 => "cinco".to_string(),
        6 => "seis".to_string(),
        7 => "siete".to_string(),
        8 => "ocho".to_string(),
        9 => "nueve".to_string(),
        10 => "diez".to_string(),
        11 => "once".to_string(),
        12 => "doce".to_string(),
        13 => "trece".to_string(),
        14 => "catorce".to_string(),
        15 => "quince".to_string(),
        16 => "dieciséis".to_string(),
        17 => "diecisiete".to_string(),
        18 => "dieciocho".to_string(),
        19 => "diecinueve".to_string(),
        20 => "veinte".to_string(),
        21 => "veintiuno".to_string(),
        22 => "veintidós".to_string(),
        23 => "veintitrés".to_string(),
        24 => "veinticuatro".to_string(),
        25 => "veinticinco".to_string(),
        26 => "veintiséis".to_string(),
        27 => "veintisiete".to_string(),
        28 => "veintiocho".to_string(),
        29 => "veintinueve".to_string(),
        30..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            let tens_word = match tens {
                3 => "treinta",
                4 => "cuarenta",
                5 => "cincuenta",
                6 => "sesenta",
                7 => "setenta",
                8 => "ochenta",
                9 => "noventa",
                _ => "",
            };
            if ones == 0 {
                tens_word.to_string()
            } else {
                format!("{} y {}", tens_word, number_to_spanish(ones))
            }
        }
        100 => "cien".to_string(),
        101..=199 => format!("ciento {}", number_to_spanish(n - 100)),
        200..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            let hundreds_word = match hundreds {
                2 => "doscientos",
                3 => "trescientos",
                4 => "cuatrocientos",
                5 => "quinientos",
                6 => "seiscientos",
                7 => "setecientos",
                8 => "ochocientos",
                9 => "novecientos",
                _ => "",
            };
            if remainder == 0 {
                hundreds_word.to_string()
            } else {
                format!("{} {}", hundreds_word, number_to_spanish(remainder))
            }
        }
        1000 => "mil".to_string(),
        1001..=999999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            let thousands_part = if thousands == 1 {
                "mil".to_string()
            } else {
                format!("{} mil", number_to_spanish(thousands))
            };
            if remainder == 0 {
                thousands_part
            } else {
                format!("{} {}", thousands_part, number_to_spanish(remainder))
            }
        }
        1000000 => "un millón".to_string(),
        1000001..=999999999 => {
            let millions = n / 1000000;
            let remainder = n % 1000000;
            let millions_part = if millions == 1 {
                "un millón".to_string()
            } else {
                format!("{} millones", number_to_spanish(millions))
            };
            if remainder == 0 {
                millions_part
            } else {
                format!("{} {}", millions_part, number_to_spanish(remainder))
            }
        }
        _ => n.to_string(), // Fallback for very large numbers
    }
}

fn number_to_ordinal(n: u64) -> String {
    match n {
        1 => "primero".to_string(),
        2 => "segundo".to_string(),
        3 => "tercero".to_string(),
        4 => "cuarto".to_string(),
        5 => "quinto".to_string(),
        6 => "sexto".to_string(),
        7 => "séptimo".to_string(),
        8 => "octavo".to_string(),
        9 => "noveno".to_string(),
        10 => "décimo".to_string(),
        _ => format!("{}º", n), // Keep as-is for higher ordinals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_numbers() {
        assert_eq!(number_to_spanish(0), "cero");
        assert_eq!(number_to_spanish(1), "uno");
        assert_eq!(number_to_spanish(15), "quince");
        assert_eq!(number_to_spanish(21), "veintiuno");
        assert_eq!(number_to_spanish(100), "cien");
        assert_eq!(number_to_spanish(101), "ciento uno");
    }

    #[test]
    fn test_currency() {
        let result = normalize("€50");
        assert!(result.contains("cincuenta euros"));
    }

    #[test]
    fn test_ordinals() {
        let result = normalize("1º lugar");
        assert!(result.contains("primero"));
    }
}
