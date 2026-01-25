//! Text normalization for Korean

use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
static CURRENCY_KRW: Lazy<Regex> = Lazy::new(|| Regex::new(r"₩\s*(\d+(?:,\d+)*)").unwrap());

pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();
    result = normalize_currency_krw(&result);
    result = normalize_numbers(&result);
    result
}

fn normalize_currency_krw(text: &str) -> String {
    CURRENCY_KRW.replace_all(text, |caps: &regex::Captures| {
        let amount = caps[1].replace(',', "");
        let num: u64 = amount.parse().unwrap_or(0);
        format!("{} 원", number_to_korean_sino(num))
    }).to_string()
}

fn normalize_numbers(text: &str) -> String {
    NUMBER_PATTERN.replace_all(text, |caps: &regex::Captures| {
        let num: u64 = caps[1].parse().unwrap_or(0);
        number_to_korean_sino(num)
    }).to_string()
}

/// Convert number to Sino-Korean (used for counting, prices, etc.)
pub fn number_to_korean_sino(n: u64) -> String {
    match n {
        0 => "영".to_string(),
        1 => "일".to_string(),
        2 => "이".to_string(),
        3 => "삼".to_string(),
        4 => "사".to_string(),
        5 => "오".to_string(),
        6 => "육".to_string(),
        7 => "칠".to_string(),
        8 => "팔".to_string(),
        9 => "구".to_string(),
        10 => "십".to_string(),
        11..=19 => format!("십{}", number_to_korean_sino(n - 10)),
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            if ones == 0 {
                format!("{}십", number_to_korean_sino(tens))
            } else {
                format!("{}십{}", number_to_korean_sino(tens), number_to_korean_sino(ones))
            }
        }
        100 => "백".to_string(),
        101..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            let h = if hundreds == 1 {
                "백".to_string()
            } else {
                format!("{}백", number_to_korean_sino(hundreds))
            };
            if remainder == 0 {
                h
            } else {
                format!("{}{}", h, number_to_korean_sino(remainder))
            }
        }
        1000 => "천".to_string(),
        1001..=9999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            let t = if thousands == 1 {
                "천".to_string()
            } else {
                format!("{}천", number_to_korean_sino(thousands))
            };
            if remainder == 0 {
                t
            } else {
                format!("{}{}", t, number_to_korean_sino(remainder))
            }
        }
        10000 => "만".to_string(),
        10001..=99999999 => {
            let man = n / 10000;
            let remainder = n % 10000;
            let m = if man == 1 {
                "만".to_string()
            } else {
                format!("{}만", number_to_korean_sino(man))
            };
            if remainder == 0 {
                m
            } else {
                format!("{}{}", m, number_to_korean_sino(remainder))
            }
        }
        100000000 => "억".to_string(),
        100000001..=999999999999 => {
            let eok = n / 100000000;
            let remainder = n % 100000000;
            let e = if eok == 1 {
                "억".to_string()
            } else {
                format!("{}억", number_to_korean_sino(eok))
            };
            if remainder == 0 {
                e
            } else {
                format!("{}{}", e, number_to_korean_sino(remainder))
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
        assert_eq!(number_to_korean_sino(0), "영");
        assert_eq!(number_to_korean_sino(1), "일");
        assert_eq!(number_to_korean_sino(10), "십");
        assert_eq!(number_to_korean_sino(11), "십일");
        assert_eq!(number_to_korean_sino(21), "이십일");
        assert_eq!(number_to_korean_sino(100), "백");
        assert_eq!(number_to_korean_sino(1000), "천");
        assert_eq!(number_to_korean_sino(10000), "만");
    }

    #[test]
    fn test_currency() {
        let result = normalize("₩50,000");
        assert!(result.contains("오만 원"));
    }
}
