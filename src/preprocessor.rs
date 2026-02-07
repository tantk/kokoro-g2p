//! Text preprocessing for G2P conversion
//!
//! Handles normalization of:
//! - Numbers (123 -> one hundred twenty three)
//! - Currency ($123.45 -> one hundred twenty three dollars and forty five cents)
//! - Time (2:30 PM -> two thirty PM)
//! - Ordinals (1st -> first)
//! - Abbreviations (Dr. -> Doctor)

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

/// Common abbreviations
static ABBREVIATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Titles
    m.insert("Mr.", "Mister");
    m.insert("Mrs.", "Missus");
    m.insert("Ms.", "Miss");
    m.insert("Dr.", "Doctor");
    m.insert("Prof.", "Professor");
    m.insert("Sr.", "Senior");
    m.insert("Jr.", "Junior");
    m.insert("Rev.", "Reverend");
    m.insert("St.", "Saint");
    m.insert("Gen.", "General");
    m.insert("Col.", "Colonel");
    m.insert("Lt.", "Lieutenant");
    m.insert("Sgt.", "Sergeant");
    m.insert("Capt.", "Captain");
    m.insert("Cmdr.", "Commander");
    m.insert("Adm.", "Admiral");
    m.insert("Gov.", "Governor");
    m.insert("Sen.", "Senator");
    m.insert("Rep.", "Representative");

    // Common abbreviations
    m.insert("vs.", "versus");
    m.insert("vs", "versus");
    m.insert("etc.", "etcetera");
    m.insert("i.e.", "that is");
    m.insert("e.g.", "for example");
    m.insert("a.m.", "AM");
    m.insert("p.m.", "PM");
    m.insert("A.M.", "AM");
    m.insert("P.M.", "PM");

    // Units
    m.insert("ft.", "feet");
    m.insert("in.", "inches");
    m.insert("lb.", "pounds");
    m.insert("lbs.", "pounds");
    m.insert("oz.", "ounces");
    m.insert("pt.", "pints");
    m.insert("qt.", "quarts");
    m.insert("gal.", "gallons");
    m.insert("mi.", "miles");
    m.insert("yd.", "yards");
    m.insert("sq.", "square");
    m.insert("hr.", "hour");
    m.insert("hrs.", "hours");
    m.insert("min.", "minute");
    m.insert("mins.", "minutes");
    m.insert("sec.", "second");
    m.insert("secs.", "seconds");

    m
});

/// Currency symbols and names
static CURRENCIES: Lazy<HashMap<char, (&str, &str)>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('$', ("dollar", "cent"));
    m.insert('£', ("pound", "pence"));
    m.insert('€', ("euro", "cent"));
    m.insert('¥', ("yen", "sen"));
    m
});

/// Regex patterns
static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(\$|£|€|¥)?(-?\d{1,3}(?:,\d{3})*(?:\.\d+)?|\d+(?:\.\d+)?)(st|nd|rd|th|s|'s)?").unwrap()
});

static TIME_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d{1,2}):(\d{2})(?::(\d{2}))?\s*(AM|PM|am|pm|a\.m\.|p\.m\.)?").unwrap()
});

#[allow(dead_code)] // Reserved for future year detection
static YEAR_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(1[0-9]{3}|20[0-9]{2})\b").unwrap()
});

/// Convert a number to words
pub fn number_to_words(n: i64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut num = n;
    let mut result = String::new();

    if num < 0 {
        result.push_str("minus ");
        num = -num;
    }

    let parts = [
        (1_000_000_000_000, "trillion"),
        (1_000_000_000, "billion"),
        (1_000_000, "million"),
        (1_000, "thousand"),
        (100, "hundred"),
    ];

    for (divisor, name) in parts {
        if num >= divisor {
            let count = num / divisor;
            num %= divisor;
            if !result.is_empty() && !result.ends_with(' ') {
                result.push(' ');
            }
            result.push_str(&small_number_to_words(count as i32));
            result.push(' ');
            result.push_str(name);
        }
    }

    if num > 0 {
        if !result.is_empty() && !result.ends_with(' ') {
            result.push(' ');
        }
        result.push_str(&small_number_to_words(num as i32));
    }

    result.trim().to_string()
}

/// Convert numbers 0-99 to words
fn small_number_to_words(n: i32) -> String {
    const ONES: &[&str] = &[
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen",
    ];
    const TENS: &[&str] = &[
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n < 20 {
        ONES[n as usize].to_string()
    } else {
        let tens = TENS[(n / 10) as usize];
        let ones = ONES[(n % 10) as usize];
        if ones.is_empty() {
            tens.to_string()
        } else {
            format!("{} {}", tens, ones)
        }
    }
}

/// Convert ordinal number to words
pub fn ordinal_to_words(n: i64) -> String {
    if n == 0 {
        return "zeroth".to_string();
    }

    let cardinal = number_to_words(n);

    // Handle special cases
    if cardinal.ends_with("one") {
        format!("{}first", &cardinal[..cardinal.len() - 3])
    } else if cardinal.ends_with("two") {
        format!("{}second", &cardinal[..cardinal.len() - 3])
    } else if cardinal.ends_with("three") {
        format!("{}third", &cardinal[..cardinal.len() - 5])
    } else if cardinal.ends_with("five") {
        format!("{}fifth", &cardinal[..cardinal.len() - 4])
    } else if cardinal.ends_with("eight") {
        format!("{}eighth", &cardinal[..cardinal.len() - 5])
    } else if cardinal.ends_with("nine") {
        format!("{}ninth", &cardinal[..cardinal.len() - 4])
    } else if cardinal.ends_with("twelve") {
        format!("{}twelfth", &cardinal[..cardinal.len() - 6])
    } else if cardinal.ends_with('y') {
        format!("{}ieth", &cardinal[..cardinal.len() - 1])
    } else {
        format!("{}th", cardinal)
    }
}

/// Convert year to words (e.g., 1984 -> nineteen eighty four)
pub fn year_to_words(year: i64) -> String {
    if year < 1000 || year >= 3000 {
        return number_to_words(year);
    }

    let century = year / 100;
    let decade = year % 100;

    if decade == 0 {
        format!("{} hundred", small_number_to_words(century as i32))
    } else if decade < 10 {
        format!(
            "{} oh {}",
            small_number_to_words(century as i32),
            small_number_to_words(decade as i32)
        )
    } else {
        format!(
            "{} {}",
            small_number_to_words(century as i32),
            small_number_to_words(decade as i32)
        )
    }
}

/// Convert time to words (e.g., 2:30 -> two thirty)
pub fn time_to_words(hours: i32, minutes: i32, seconds: Option<i32>, period: Option<&str>) -> String {
    let mut result = String::new();

    // Hours
    let h = if hours == 0 {
        "twelve".to_string()
    } else if hours > 12 {
        small_number_to_words(hours - 12)
    } else {
        small_number_to_words(hours)
    };
    result.push_str(&h);

    // Minutes
    if minutes == 0 {
        if let Some(s) = seconds {
            if s != 0 {
                result.push_str(" oh clock");
            }
        }
    } else if minutes < 10 {
        result.push_str(" oh ");
        result.push_str(&small_number_to_words(minutes));
    } else {
        result.push(' ');
        result.push_str(&small_number_to_words(minutes));
    }

    // Seconds
    if let Some(s) = seconds {
        if s > 0 {
            result.push_str(" and ");
            result.push_str(&small_number_to_words(s));
            result.push_str(" seconds");
        }
    }

    // Period
    if let Some(p) = period {
        result.push(' ');
        result.push_str(p.to_uppercase().trim_matches('.').trim());
    }

    result
}

/// Convert currency amount to words
pub fn currency_to_words(amount: f64, symbol: char) -> String {
    let (unit, subunit) = CURRENCIES.get(&symbol).unwrap_or(&("dollar", "cent"));

    let whole = amount.abs().floor() as i64;
    let frac = ((amount.abs() - whole as f64) * 100.0).round() as i64;

    let mut result = String::new();

    if amount < 0.0 {
        result.push_str("minus ");
    }

    if whole > 0 {
        result.push_str(&number_to_words(whole));
        result.push(' ');
        if whole == 1 {
            result.push_str(unit);
        } else {
            result.push_str(unit);
            result.push('s');
        }
    }

    if frac > 0 {
        if whole > 0 {
            result.push_str(" and ");
        }
        result.push_str(&number_to_words(frac));
        result.push(' ');
        if frac == 1 {
            result.push_str(subunit);
        } else if *subunit == "pence" {
            result.push_str(subunit);
        } else {
            result.push_str(subunit);
            result.push('s');
        }
    }

    if result.is_empty() {
        result.push_str("zero ");
        result.push_str(unit);
        result.push('s');
    }

    result
}

/// Preprocess text for G2P conversion
pub fn preprocess(text: &str) -> String {
    use unicode_normalization::UnicodeNormalization;

    // Normalize unicode
    let mut result: String = text.nfkc().collect();

    // Expand abbreviations
    for (abbr, expansion) in ABBREVIATIONS.iter() {
        result = result.replace(abbr, expansion);
    }

    // Handle time expressions
    result = TIME_PATTERN
        .replace_all(&result, |caps: &regex::Captures| {
            let hours: i32 = caps[1].parse().unwrap_or(0);
            let minutes: i32 = caps[2].parse().unwrap_or(0);
            let seconds: Option<i32> = caps.get(3).map(|m| m.as_str().parse().unwrap_or(0));
            let period = caps.get(4).map(|m| m.as_str());
            time_to_words(hours, minutes, seconds, period)
        })
        .to_string();

    // Handle currency and numbers
    result = NUMBER_PATTERN
        .replace_all(&result, |caps: &regex::Captures| {
            let currency = caps.get(1).and_then(|m| m.as_str().chars().next());
            let num_str = &caps[2];
            let suffix = caps.get(3).map(|m| m.as_str());

            // Parse the number
            let clean_num: String = num_str.chars().filter(|c| *c != ',').collect();
            let num: f64 = clean_num.parse().unwrap_or(0.0);

            // Handle currency
            if let Some(sym) = currency {
                return currency_to_words(num, sym);
            }

            // Handle ordinals
            if let Some(suf) = suffix {
                match suf.to_lowercase().as_str() {
                    "st" | "nd" | "rd" | "th" => {
                        return ordinal_to_words(num as i64);
                    }
                    "s" | "'s" => {
                        return format!("{}{}", number_to_words(num as i64), suf);
                    }
                    _ => {}
                }
            }

            // Check if it's a year
            let year = num as i64;
            if year >= 1000 && year <= 2100 && num.fract() == 0.0 {
                // Heuristic: standalone 4-digit numbers in this range are years
                return year_to_words(year);
            }

            // Regular number
            if num.fract() == 0.0 {
                number_to_words(num as i64)
            } else {
                // Decimal number
                let whole = num.floor() as i64;
                let frac_str = format!("{:.10}", num.fract());
                let frac_digits = frac_str.trim_start_matches("0.").trim_end_matches('0');
                let frac_digits = if frac_digits.is_empty() { "0" } else { frac_digits };

                let mut result = number_to_words(whole);
                result.push_str(" point");
                for c in frac_digits.chars() {
                    if let Some(digit) = c.to_digit(10) {
                        result.push(' ');
                        result.push_str(&small_number_to_words(digit as i32));
                    }
                }
                result
            }
        })
        .to_string();

    // Clean up whitespace
    let whitespace_re = Regex::new(r"\s+").unwrap();
    result = whitespace_re.replace_all(&result, " ").trim().to_string();

    result
}

/// Token representation for preprocessing
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub whitespace: String,
    pub phonemes: Option<String>,
    pub is_punct: bool,
}

impl Token {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            whitespace: String::new(),
            phonemes: None,
            is_punct: false,
        }
    }

    pub fn with_whitespace(mut self, ws: &str) -> Self {
        self.whitespace = ws.to_string();
        self
    }

    pub fn as_punct(mut self) -> Self {
        self.is_punct = true;
        self
    }
}

/// Tokenize text into words and punctuation
pub fn tokenize(text: &str) -> Vec<Token> {
    let word_re = Regex::new(r"([a-zA-Z'']+(?:[''][a-zA-Z]+)*|[0-9]+(?:[.,][0-9]+)*|[^\s\w])").unwrap();

    let mut tokens: Vec<Token> = Vec::new();
    let mut last_end = 0;

    for cap in word_re.captures_iter(text) {
        let m = cap.get(0).unwrap();

        // Check for whitespace before this token
        if m.start() > last_end {
            if let Some(last) = tokens.last_mut() {
                last.whitespace = " ".to_string();
            }
        }

        let word = m.as_str();
        let is_punct = word.len() == 1 && word.chars().next().map_or(false, |c| !c.is_alphanumeric());

        let token = if is_punct {
            Token::new(word).as_punct()
        } else {
            Token::new(word)
        };

        tokens.push(token);
        last_end = m.end();
    }

    // Handle trailing whitespace
    if last_end < text.len() {
        if let Some(last) = tokens.last_mut() {
            last.whitespace = " ".to_string();
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_words() {
        assert_eq!(number_to_words(0), "zero");
        assert_eq!(number_to_words(1), "one");
        assert_eq!(number_to_words(13), "thirteen");
        assert_eq!(number_to_words(21), "twenty one");
        assert_eq!(number_to_words(100), "one hundred");
        assert_eq!(number_to_words(123), "one hundred twenty three");
        assert_eq!(number_to_words(1000), "one thousand");
        assert_eq!(number_to_words(1234), "one thousand two hundred thirty four");
        assert_eq!(number_to_words(-5), "minus five");
    }

    #[test]
    fn test_ordinal_to_words() {
        assert_eq!(ordinal_to_words(1), "first");
        assert_eq!(ordinal_to_words(2), "second");
        assert_eq!(ordinal_to_words(3), "third");
        assert_eq!(ordinal_to_words(4), "fourth");
        assert_eq!(ordinal_to_words(5), "fifth");
        assert_eq!(ordinal_to_words(12), "twelfth");
        assert_eq!(ordinal_to_words(21), "twenty first");
    }

    #[test]
    fn test_year_to_words() {
        assert_eq!(year_to_words(1984), "nineteen eighty four");
        assert_eq!(year_to_words(2001), "twenty oh one");
        assert_eq!(year_to_words(2000), "twenty hundred");
        assert_eq!(year_to_words(2024), "twenty twenty four");
    }

    #[test]
    fn test_time_to_words() {
        assert_eq!(time_to_words(2, 30, None, Some("PM")), "two thirty PM");
        assert_eq!(time_to_words(12, 0, None, None), "twelve");
        assert_eq!(time_to_words(3, 5, None, Some("am")), "three oh five AM");
    }

    #[test]
    fn test_currency_to_words() {
        assert_eq!(currency_to_words(123.45, '$'), "one hundred twenty three dollars and forty five cents");
        assert_eq!(currency_to_words(1.00, '$'), "one dollar");
        assert_eq!(currency_to_words(0.50, '$'), "fifty cents");
    }

    #[test]
    fn test_preprocess() {
        let text = "Dr. Smith has $123.45 and arrived at 2:30 PM.";
        let result = preprocess(text);
        assert!(result.contains("Doctor"));
        assert!(result.contains("dollars"));
        assert!(result.contains("thirty"));
    }

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("Hello, world!");
        assert_eq!(tokens.len(), 4); // Hello , world !
        assert!(!tokens[0].is_punct);
        assert!(tokens[1].is_punct);
    }
}
