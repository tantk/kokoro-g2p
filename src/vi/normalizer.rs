//! Text normalization for Vietnamese

use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
static CURRENCY_VND: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+(?:[.,]\d+)?)\s*(?:VND|đ|₫)").unwrap());
static CURRENCY_USD: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\s*(\d+(?:[.,]\d+)?)").unwrap());

pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();
    result = normalize_currency_vnd(&result);
    result = normalize_currency_usd(&result);
    result = normalize_numbers(&result);
    result
}

fn normalize_currency_vnd(text: &str) -> String {
    CURRENCY_VND.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace('.', "").replace(',', "");
        let num: u64 = amount.parse().unwrap_or(0);
        format!("{} đồng", number_to_vietnamese(num))
    }).to_string()
}

fn normalize_currency_usd(text: &str) -> String {
    CURRENCY_USD.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace(',', ".");
        let num: f64 = amount.parse().unwrap_or(0.0);
        let whole = num as u64;
        let cents = ((num - whole as f64) * 100.0).round() as u64;

        if cents > 0 {
            format!("{} đô la {} xu", number_to_vietnamese(whole), number_to_vietnamese(cents))
        } else {
            format!("{} đô la", number_to_vietnamese(whole))
        }
    }).to_string()
}

fn normalize_numbers(text: &str) -> String {
    NUMBER_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_vietnamese(num)
    }).to_string()
}

pub fn number_to_vietnamese(n: u64) -> String {
    match n {
        0 => "không".to_string(),
        1 => "một".to_string(),
        2 => "hai".to_string(),
        3 => "ba".to_string(),
        4 => "bốn".to_string(),
        5 => "năm".to_string(),
        6 => "sáu".to_string(),
        7 => "bảy".to_string(),
        8 => "tám".to_string(),
        9 => "chín".to_string(),
        10 => "mười".to_string(),
        11..=19 => {
            let ones = n - 10;
            if ones == 5 {
                "mười lăm".to_string()
            } else {
                format!("mười {}", number_to_vietnamese(ones))
            }
        }
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            if ones == 0 {
                format!("{} mươi", number_to_vietnamese(tens))
            } else if ones == 1 {
                format!("{} mươi mốt", number_to_vietnamese(tens))
            } else if ones == 5 {
                format!("{} mươi lăm", number_to_vietnamese(tens))
            } else {
                format!("{} mươi {}", number_to_vietnamese(tens), number_to_vietnamese(ones))
            }
        }
        100 => "một trăm".to_string(),
        101..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} trăm", number_to_vietnamese(hundreds))
            } else if remainder < 10 {
                format!("{} trăm lẻ {}", number_to_vietnamese(hundreds), number_to_vietnamese(remainder))
            } else {
                format!("{} trăm {}", number_to_vietnamese(hundreds), number_to_vietnamese(remainder))
            }
        }
        1000 => "một nghìn".to_string(),
        1001..=999999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            if remainder == 0 {
                format!("{} nghìn", number_to_vietnamese(thousands))
            } else if remainder < 100 {
                format!("{} nghìn lẻ {}", number_to_vietnamese(thousands), number_to_vietnamese(remainder))
            } else {
                format!("{} nghìn {}", number_to_vietnamese(thousands), number_to_vietnamese(remainder))
            }
        }
        1000000 => "một triệu".to_string(),
        1000001..=999999999 => {
            let millions = n / 1000000;
            let remainder = n % 1000000;
            if remainder == 0 {
                format!("{} triệu", number_to_vietnamese(millions))
            } else {
                format!("{} triệu {}", number_to_vietnamese(millions), number_to_vietnamese(remainder))
            }
        }
        1000000000 => "một tỷ".to_string(),
        1000000001..=999999999999 => {
            let billions = n / 1000000000;
            let remainder = n % 1000000000;
            if remainder == 0 {
                format!("{} tỷ", number_to_vietnamese(billions))
            } else {
                format!("{} tỷ {}", number_to_vietnamese(billions), number_to_vietnamese(remainder))
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
        assert_eq!(number_to_vietnamese(0), "không");
        assert_eq!(number_to_vietnamese(1), "một");
        assert_eq!(number_to_vietnamese(10), "mười");
        assert_eq!(number_to_vietnamese(15), "mười lăm");
        assert_eq!(number_to_vietnamese(21), "hai mươi mốt");
        assert_eq!(number_to_vietnamese(100), "một trăm");
        assert_eq!(number_to_vietnamese(105), "một trăm lẻ năm");
    }

    #[test]
    fn test_currency_vnd() {
        let result = normalize("50000 đ");
        assert!(result.contains("nghìn đồng"));
    }
}
