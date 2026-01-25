//! Chinese text normalization
//!
//! Handles conversion of numbers, dates, currency, and other special formats
//! to spoken Chinese text.

use once_cell::sync::Lazy;
use regex::Regex;

// Regex patterns for various normalizations
static NUM_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
static DECIMAL_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)\.(\d+)").unwrap());
static DATE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日").unwrap());
static TIME_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d{1,2})[点时](\d{1,2})分?").unwrap());
static CURRENCY_SGD: Lazy<Regex> = Lazy::new(|| Regex::new(r"S\$(\d+(?:\.\d{2})?)").unwrap());
static CURRENCY_RMB: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:RMB|￥|¥)(\d+(?:\.\d{2})?)").unwrap());
static CURRENCY_USD: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$(\d+(?:\.\d{2})?)").unwrap());
static PERCENT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+(?:\.\d+)?)%").unwrap());
static PHONE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"1[3-9]\d{9}").unwrap());

/// Chinese digit characters
const DIGITS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

/// Chinese unit characters for large numbers
const UNITS: [&str; 4] = ["", "十", "百", "千"];
const BIG_UNITS: [&str; 4] = ["", "万", "亿", "兆"];

/// Normalize Chinese text
pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();

    // Apply normalizations in order (most specific first)
    result = normalize_currency_sgd(&result);
    result = normalize_currency_rmb(&result);
    result = normalize_currency_usd(&result);
    result = normalize_percent(&result);
    result = normalize_date(&result);
    result = normalize_time(&result);
    result = normalize_phone(&result);
    result = normalize_decimal(&result);
    result = normalize_integers(&result);

    result
}

/// Convert a digit to Chinese
fn digit_to_chinese(d: char) -> &'static str {
    match d {
        '0' => DIGITS[0],
        '1' => DIGITS[1],
        '2' => DIGITS[2],
        '3' => DIGITS[3],
        '4' => DIGITS[4],
        '5' => DIGITS[5],
        '6' => DIGITS[6],
        '7' => DIGITS[7],
        '8' => DIGITS[8],
        '9' => DIGITS[9],
        _ => "",
    }
}

/// Convert an integer string to Chinese
pub fn number_to_chinese(num_str: &str) -> String {
    // Handle zero
    if num_str == "0" {
        return DIGITS[0].to_string();
    }

    // Remove leading zeros
    let num_str = num_str.trim_start_matches('0');
    if num_str.is_empty() {
        return DIGITS[0].to_string();
    }

    let len = num_str.len();

    // For very large numbers or numbers with many digits, read digit by digit
    if len > 8 {
        return num_str.chars().map(digit_to_chinese).collect::<String>();
    }

    // Parse and convert
    let mut result = String::new();
    let chars: Vec<char> = num_str.chars().collect();

    // Process in groups of 4 (万 units)
    let groups: Vec<&[char]> = chars.rchunks(4).collect::<Vec<_>>();
    let num_groups = groups.len();

    for (group_idx, group) in groups.iter().rev().enumerate() {
        let group_result = convert_four_digits(group);
        if !group_result.is_empty() {
            result.push_str(&group_result);
            // Add big unit (万, 亿)
            let big_unit_idx = num_groups - 1 - group_idx;
            if big_unit_idx > 0 && big_unit_idx < BIG_UNITS.len() {
                result.push_str(BIG_UNITS[big_unit_idx]);
            }
        } else if !result.is_empty() && !result.ends_with('零') {
            // Add zero placeholder for empty groups in middle
            result.push_str(DIGITS[0]);
        }
    }

    // Clean up trailing zeros
    while result.ends_with('零') {
        result.pop();
    }

    // Handle special case: 一十 -> 十 for numbers 10-19
    // (i.e., 一十, 一十一, 一十二, etc. become 十, 十一, 十二)
    if result.starts_with("一十") && result.chars().count() <= 3 {
        result = result.replacen("一十", "十", 1);
    }

    result
}

/// Convert a 4-digit group to Chinese
fn convert_four_digits(digits: &[char]) -> String {
    let mut result = String::new();
    let len = digits.len();
    let mut need_zero = false;

    for (i, &d) in digits.iter().enumerate() {
        let pos = len - 1 - i; // Position from right (0=units, 1=tens, etc.)
        let digit_val = d.to_digit(10).unwrap_or(0) as usize;

        if digit_val == 0 {
            need_zero = true;
        } else {
            if need_zero && !result.is_empty() {
                result.push_str(DIGITS[0]);
            }
            result.push_str(DIGITS[digit_val]);
            if pos > 0 {
                result.push_str(UNITS[pos]);
            }
            need_zero = false;
        }
    }

    result
}

/// Normalize integer numbers in text
fn normalize_integers(text: &str) -> String {
    NUM_PATTERN
        .replace_all(text, |caps: &regex::Captures| number_to_chinese(&caps[0]))
        .to_string()
}

/// Normalize decimal numbers
fn normalize_decimal(text: &str) -> String {
    DECIMAL_PATTERN
        .replace_all(text, |caps: &regex::Captures| {
            let integer_part = number_to_chinese(&caps[1]);
            let decimal_part: String = caps[2].chars().map(digit_to_chinese).collect();
            format!("{}点{}", integer_part, decimal_part)
        })
        .to_string()
}

/// Normalize dates (2024年1月15日)
fn normalize_date(text: &str) -> String {
    DATE_PATTERN
        .replace_all(text, |caps: &regex::Captures| {
            let year: String = caps[1].chars().map(digit_to_chinese).collect();
            let month = number_to_chinese(&caps[2]);
            let day = number_to_chinese(&caps[3]);
            format!("{}年{}月{}日", year, month, day)
        })
        .to_string()
}

/// Normalize time (3点30分)
fn normalize_time(text: &str) -> String {
    TIME_PATTERN
        .replace_all(text, |caps: &regex::Captures| {
            let hour = number_to_chinese(&caps[1]);
            let minute = number_to_chinese(&caps[2]);
            if minute == "零" {
                format!("{}点整", hour)
            } else {
                format!("{}点{}分", hour, minute)
            }
        })
        .to_string()
}

/// Normalize Singapore Dollar (S$50 -> 新加坡元五十)
fn normalize_currency_sgd(text: &str) -> String {
    CURRENCY_SGD
        .replace_all(text, |caps: &regex::Captures| {
            let amount = &caps[1];
            if amount.contains('.') {
                let parts: Vec<&str> = amount.split('.').collect();
                let dollars = number_to_chinese(parts[0]);
                let cents = number_to_chinese(parts[1]);
                format!("新加坡元{}元{}分", dollars, cents)
            } else {
                format!("新加坡元{}", number_to_chinese(amount))
            }
        })
        .to_string()
}

/// Normalize RMB (¥100 / RMB100 -> 一百元)
fn normalize_currency_rmb(text: &str) -> String {
    CURRENCY_RMB
        .replace_all(text, |caps: &regex::Captures| {
            let amount = &caps[1];
            if amount.contains('.') {
                let parts: Vec<&str> = amount.split('.').collect();
                let yuan = number_to_chinese(parts[0]);
                let jiao_fen = parts[1];
                if jiao_fen.len() >= 2 {
                    let jiao = digit_to_chinese(jiao_fen.chars().next().unwrap());
                    let fen = digit_to_chinese(jiao_fen.chars().nth(1).unwrap());
                    if fen == "零" {
                        format!("{}元{}角", yuan, jiao)
                    } else {
                        format!("{}元{}角{}分", yuan, jiao, fen)
                    }
                } else {
                    let jiao = digit_to_chinese(jiao_fen.chars().next().unwrap());
                    format!("{}元{}角", yuan, jiao)
                }
            } else {
                format!("{}元", number_to_chinese(amount))
            }
        })
        .to_string()
}

/// Normalize USD ($100 -> 美元一百)
fn normalize_currency_usd(text: &str) -> String {
    CURRENCY_USD
        .replace_all(text, |caps: &regex::Captures| {
            let amount = &caps[1];
            if amount.contains('.') {
                let parts: Vec<&str> = amount.split('.').collect();
                let dollars = number_to_chinese(parts[0]);
                let cents = number_to_chinese(parts[1]);
                format!("美元{}元{}分", dollars, cents)
            } else {
                format!("美元{}", number_to_chinese(amount))
            }
        })
        .to_string()
}

/// Normalize percentage (50% -> 百分之五十)
fn normalize_percent(text: &str) -> String {
    PERCENT_PATTERN
        .replace_all(text, |caps: &regex::Captures| {
            let num = &caps[1];
            if num.contains('.') {
                let parts: Vec<&str> = num.split('.').collect();
                let integer = number_to_chinese(parts[0]);
                let decimal: String = parts[1].chars().map(digit_to_chinese).collect();
                format!("百分之{}点{}", integer, decimal)
            } else {
                format!("百分之{}", number_to_chinese(num))
            }
        })
        .to_string()
}

/// Normalize phone numbers (read digit by digit)
fn normalize_phone(text: &str) -> String {
    PHONE_PATTERN
        .replace_all(text, |caps: &regex::Captures| {
            caps[0].chars().map(digit_to_chinese).collect::<String>()
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_chinese() {
        assert_eq!(number_to_chinese("0"), "零");
        assert_eq!(number_to_chinese("1"), "一");
        assert_eq!(number_to_chinese("10"), "十");
        assert_eq!(number_to_chinese("11"), "十一");
        assert_eq!(number_to_chinese("100"), "一百");
        assert_eq!(number_to_chinese("101"), "一百零一");
        assert_eq!(number_to_chinese("1000"), "一千");
        assert_eq!(number_to_chinese("10000"), "一万");
        assert_eq!(number_to_chinese("12345"), "一万二千三百四十五");
    }

    #[test]
    fn test_normalize_currency_sgd() {
        let result = normalize("S$50");
        assert!(result.contains("新加坡元"));
        assert!(result.contains("五十"));
    }

    #[test]
    fn test_normalize_currency_rmb() {
        let result = normalize("¥100");
        assert!(result.contains("一百"));
        assert!(result.contains("元"));
    }

    #[test]
    fn test_normalize_percent() {
        let result = normalize("50%");
        assert!(result.contains("百分之"));
        assert!(result.contains("五十"));
    }

    #[test]
    fn test_normalize_date() {
        let result = normalize("2024年1月15日");
        assert!(result.contains("二零二四年"));
        assert!(result.contains("一月"));
        assert!(result.contains("十五日"));
    }

    #[test]
    fn test_normalize_mixed() {
        let result = normalize("我有S$100和50%的折扣");
        assert!(result.contains("新加坡元"));
        assert!(result.contains("百分之"));
    }
}
