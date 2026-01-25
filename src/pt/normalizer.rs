//! Text normalization for Portuguese

use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
static CURRENCY_BRL: Lazy<Regex> = Lazy::new(|| Regex::new(r"R\$\s*(\d+(?:[.,]\d+)?)").unwrap());
static CURRENCY_EUR: Lazy<Regex> = Lazy::new(|| Regex::new(r"€\s*(\d+(?:[.,]\d+)?)").unwrap());

pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();
    result = normalize_currency_brl(&result);
    result = normalize_currency_eur(&result);
    result = normalize_numbers(&result);
    result
}

fn normalize_currency_brl(text: &str) -> String {
    CURRENCY_BRL.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace('.', "").replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let cents = ((num - whole as f64) * 100.0).round() as u64;

        if cents > 0 {
            format!("{} reais e {} centavos", number_to_portuguese(whole), number_to_portuguese(cents))
        } else if whole == 1 {
            "um real".to_string()
        } else {
            format!("{} reais", number_to_portuguese(whole))
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
            format!("{} euros e {} cêntimos", number_to_portuguese(whole), number_to_portuguese(cents))
        } else {
            format!("{} euros", number_to_portuguese(whole))
        }
    }).to_string()
}

fn normalize_numbers(text: &str) -> String {
    NUMBER_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_portuguese(num)
    }).to_string()
}

pub fn number_to_portuguese(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "um".to_string(),
        2 => "dois".to_string(),
        3 => "três".to_string(),
        4 => "quatro".to_string(),
        5 => "cinco".to_string(),
        6 => "seis".to_string(),
        7 => "sete".to_string(),
        8 => "oito".to_string(),
        9 => "nove".to_string(),
        10 => "dez".to_string(),
        11 => "onze".to_string(),
        12 => "doze".to_string(),
        13 => "treze".to_string(),
        14 => "catorze".to_string(),
        15 => "quinze".to_string(),
        16 => "dezesseis".to_string(),
        17 => "dezessete".to_string(),
        18 => "dezoito".to_string(),
        19 => "dezenove".to_string(),
        20 => "vinte".to_string(),
        21..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            let tens_word = match tens {
                2 => "vinte",
                3 => "trinta",
                4 => "quarenta",
                5 => "cinquenta",
                6 => "sessenta",
                7 => "setenta",
                8 => "oitenta",
                9 => "noventa",
                _ => "",
            };
            if ones == 0 {
                tens_word.to_string()
            } else {
                format!("{} e {}", tens_word, number_to_portuguese(ones))
            }
        }
        100 => "cem".to_string(),
        101..=199 => format!("cento e {}", number_to_portuguese(n - 100)),
        200..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            let hundreds_word = match hundreds {
                2 => "duzentos",
                3 => "trezentos",
                4 => "quatrocentos",
                5 => "quinhentos",
                6 => "seiscentos",
                7 => "setecentos",
                8 => "oitocentos",
                9 => "novecentos",
                _ => "",
            };
            if remainder == 0 {
                hundreds_word.to_string()
            } else {
                format!("{} e {}", hundreds_word, number_to_portuguese(remainder))
            }
        }
        1000 => "mil".to_string(),
        1001..=999999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            let thousands_part = if thousands == 1 {
                "mil".to_string()
            } else {
                format!("{} mil", number_to_portuguese(thousands))
            };
            if remainder == 0 {
                thousands_part
            } else {
                format!("{} e {}", thousands_part, number_to_portuguese(remainder))
            }
        }
        1000000 => "um milhão".to_string(),
        1000001..=999999999 => {
            let millions = n / 1000000;
            let remainder = n % 1000000;
            let millions_part = if millions == 1 {
                "um milhão".to_string()
            } else {
                format!("{} milhões", number_to_portuguese(millions))
            };
            if remainder == 0 {
                millions_part
            } else {
                format!("{} e {}", millions_part, number_to_portuguese(remainder))
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
        assert_eq!(number_to_portuguese(0), "zero");
        assert_eq!(number_to_portuguese(1), "um");
        assert_eq!(number_to_portuguese(21), "vinte e um");
        assert_eq!(number_to_portuguese(100), "cem");
        assert_eq!(number_to_portuguese(101), "cento e um");
    }

    #[test]
    fn test_currency_brl() {
        let result = normalize("R$ 50");
        assert!(result.contains("cinquenta reais"));
    }
}
