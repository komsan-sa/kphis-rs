use base64::{Engine, engine::general_purpose::URL_SAFE};
use encoding_rs::WINDOWS_874;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use num::ToPrimitive;
use regex::Regex;
use rust_decimal::Decimal;
use std::{
    // hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
    sync::LazyLock,
    time::Duration,
};
use time::Date;
use unicode_width::UnicodeWidthStr;

use crate::datetime::{date_8601, js_now};

pub trait Concat {
    fn concat(&self, concat_with_space: bool) -> String;
}

pub fn concat_mutable_vec<T: Concat>(items: &MutableVec<Rc<T>>, concat_with_space: bool) -> String {
    let delimiter = if concat_with_space { " " } else { "|" };
    items.lock_ref().iter().map(|item| item.concat(concat_with_space)).collect::<Vec<String>>().join(delimiter)
}

// ===== ===== ===== //
// Sanitized Related //
// ===== ===== ===== //

/// `r"[^A-Za-z0-9]"` to ""
pub fn sanity_alphanumeric(input: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("[^A-Za-z0-9]").unwrap());
    RE.replace_all(input, "").trim().into()
}

/// `r"[^0-9]"` to ""
pub fn sanity_int(input: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("[^0-9]").unwrap());
    RE.replace_all(input, "").trim().into()
}

// pub fn sanity_sp(input: &str) -> String {
//     let re = Regex::new(r#"[!@#$%^&*(),.?\"{}<>]"#).unwrap();
//     re.replace_all(input, "").to_owned()
// }

// [ .]{2,} means two of [ .] will match ex: `  `, `..`, ` .`, `. `
/// `r"[ .]{2,}"` to " "
pub fn sanity_dot_space(input: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("[ .]{2,}").unwrap());
    RE.replace_all(input, " ").trim().into()
}

// [ ]{2,} means two of [ ] will match ex: `  `
/// `r"[ ]{2,}"` to " "
pub fn sanity_space(input: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("[ ]{2,}").unwrap());
    RE.replace_all(input, " ").trim().into()
}

pub fn first_char_uppercase(input: &str) -> String {
    let mut chars = input.chars();
    if let Some(first_char) = chars.next() {
        first_char.to_uppercase().collect::<String>() + chars.as_str()
    } else {
        input.to_owned()
    }
}

// /// not remove multiple whitespaces into one space, not trim<br>
// /// must use before `sanity_dot_space`
// pub fn whitespace_to_space(input: &str) -> String {
//     static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\s]").unwrap());
//     RE.replace_all(input, " ").trim().into()
// }

// https://en.wikipedia.org/wiki/Thai_Industrial_Standard_620-2533
// https://en.wikipedia.org/wiki/ISO/IEC_8859-11
/// remove non TIS-620 from string by encode with WINDOWS_874 and replace the HTML replacement `&#XXXX;` with ``
///
/// *NOTE*: WINDOWS_874 has 9 extra charactor more than TIS-620
/// - `€` : `&#8364;` or `\u{20ac}`
/// - `…` : `&#8230;` or `\u{2026}`
/// - `‘` : `&#8216;` or `\u{2018}`
/// - `’` : `&#8217;` or `\u{2019}`
/// - `“` : `&#8220;` or `\u{201c}`
/// - `”` : `&#8221;` or `\u{201d}`
/// - `•` : `&#8226;` or `\u{2022}`
/// - `–` : `&#8211;` or `\u{2013}`
/// - `—` : `&#8212;` or `\u{2014}`
pub fn sanity_tis620(input: &str) -> String {
    let input = &&input.replace('°', "ํ").replace('₁', "1").replace('₂', "2").replace('₃', "3").replace('∆', "delta-");
    static RE_874_EX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[€…‘’“”•–—]").unwrap());
    let remove_874_extra = RE_874_EX.replace_all(input, "");
    let (bytes, _, has_replacement) = WINDOWS_874.encode(&remove_874_extra);
    if has_replacement {
        let (result, _, _) = WINDOWS_874.decode(&bytes);
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("&#[0-9]+;").unwrap());
        RE.replace_all(&result, "").trim().into()
    } else {
        remove_874_extra.trim().to_owned()
    }
}

// /// is any `r"[ .]{2,}"` exists
// pub fn dot_space_exists(input: &str) -> bool {
//     DOT_SPACE_RE.is_match(input.trim())
// }

// ===== ===== ===== //
// Primitive Related //
// ===== ===== ===== //

pub fn u64_to_base64(value: u64) -> String {
    let bytes = value.to_be_bytes();
    URL_SAFE.encode(bytes)
}

pub fn add_u64_with_i64(u: u64, i: i64) -> u64 {
    if i.is_negative() {
        u.saturating_sub(i.unsigned_abs())
    } else {
        u.saturating_add(i.unsigned_abs())
    }
}

/// if < 0.5 then floor, else ceil
/// ex: fraction=3<br>
/// - 3.1234 -> 3.123<br>
/// - 3.1235 -> 3.124<br>
/// - 3.12 -> 3.120
/// - 3 -> 3
pub fn decimal_rescale(mut decimal: Decimal, fraction: u32) -> Decimal {
    if decimal.is_integer() {
        // bypass expensive rescale()
        decimal.round()
    } else {
        decimal.rescale(fraction);
        decimal
    }
}

/// if < 0.5 then floor, else ceil
/// ex: fraction=3<br>
/// - 3.1234 -> 3.123<br>
/// - 3.1235 -> 3.124<br>
/// - 3.12 -> 3.120
pub fn f64_rescale(number: f64, fraction: i32) -> f64 {
    let mul = number * 10.0f64.powi(fraction);
    let mul_fract = mul.fract();
    let mul_fract_is_pos = mul_fract.is_sign_positive();
    if (mul_fract_is_pos && mul_fract < 0.5) || (!mul_fract_is_pos && mul_fract <= (-0.5)) {
        mul.floor() / 10.0f64.powi(fraction)
    } else {
        mul.ceil() / 10.0f64.powi(fraction)
    }
}

pub fn f32_rescale(number: f32, fraction: i32) -> f32 {
    let mul = number * 10.0f32.powi(fraction);
    let mul_fract = mul.fract();
    let mul_fract_is_pos = mul_fract.is_sign_positive();
    if (mul_fract_is_pos && mul_fract < 0.5) || (!mul_fract_is_pos && mul_fract <= (-0.5)) {
        mul.floor() / 10.0f32.powi(fraction)
    } else {
        mul.ceil() / 10.0f32.powi(fraction)
    }
}

#[inline]
pub fn str_some(s: String) -> Option<String> {
    let s = s.trim();
    (!s.is_empty()).then_some(s.to_owned())
}

#[inline]
pub fn zero_str_none(s: String) -> Option<String> {
    let s = s.trim();
    (!s.is_empty() && s != "0").then_some(s.to_owned())
}

#[inline]
pub fn opt_empty_none(opt: Option<String>) -> Option<String> {
    opt.and_then(str_some)
}

#[inline]
pub fn zero_none<N: num::Zero>(n: N) -> Option<N> {
    (!n.is_zero()).then_some(n)
}

#[inline]
pub fn opt_zero_none<N: num::Zero>(opt: Option<N>) -> Option<N> {
    opt.and_then(zero_none)
}

#[inline]
pub fn is_opt_str_some(opt: Option<&String>) -> bool {
    opt.map(|s| !s.trim().is_empty()).unwrap_or_default()
}

/// "1234567.89" => "1,234,567.89"<br>
/// NOTE: without number sanitation
pub fn thousands(input: &str) -> String {
    let split = input.split('.').collect::<Vec<&str>>();
    if split.len() > 1 {
        let (left, right) = split.split_at(1);
        [thousands_inner(&left[0]), right.join(".")].join(".")
    } else {
        thousands_inner(input)
    }
}

fn thousands_inner(input: &str) -> String {
    let len = input.len();
    let is_negative = input.starts_with('-');
    if len < 4 || (len < 5 && is_negative) {
        input.to_owned()
    } else {
        // len always > 3 from here
        let start = if is_negative { 1 } else { 0 };
        let mut chars = input.chars().collect::<Vec<char>>();
        let mut i = chars.len().saturating_sub(3);

        while i > start {
            chars.insert(i, ',');
            i = i.saturating_sub(3);
        }

        chars.into_iter().collect()
    }
}

/// from raw db concat,<br>
/// concat size = 2 only, discard remainder<br>
/// ex:<br>
/// - space: `agent1 symptom1 agent2 symptom2` to `agent1=symptom1, agent2=symptom2`
/// - cap-pipe: `agent1^symptom1|agent2^symptom2` to `agent1=symptom1, agent2=symptom2`
pub fn raw_concat_to_comma_equal(concat: &str) -> String {
    explode(concat, 2).iter().map(|row| row.join("=")).collect::<Vec<String>>().join(", ")
}

/// convert concat string to Vec<Vec<String;size>>>, discard remainder
pub fn explode(text: &str, size: usize) -> Vec<Vec<&str>> {
    if size == 0 {
        return Vec::new();
    }
    if text.contains('^') { explode_cap_pipe(text, size) } else { explode_space(text, size) }
}

/// guarantee inner vec has items count = `size` argument
pub fn explode_cap_pipe(text: &str, size: usize) -> Vec<Vec<&str>> {
    explode_cap_pipe_iter(text, size).collect()
}

pub fn explode_cap_pipe_iter(text: &str, size: usize) -> impl Iterator<Item = Vec<&str>> {
    text.split('|').filter_map(move |row| {
        let c = row.trim().split('^').map(str::trim).collect::<Vec<&str>>();
        if c.len() >= size { Some(c.into_iter().take(size).collect()) } else { None }
    })
}

pub fn explode_space(text: &str, size: usize) -> Vec<Vec<&str>> {
    text.split(' ').map(str::trim).collect::<Vec<&str>>().chunks_exact(size).map(|c| c.to_vec()).collect()
}

pub fn explode_get_second_value_u32(text: &str) -> Vec<u32> {
    text.split('|')
        .flat_map(|g| g.trim().split('^').map(str::trim).take(1))
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
        .unwrap_or_default()
}

pub fn text_to_six_digits(input: &str) -> Option<[u8; 6]> {
    if input.len() == 6 {
        let chars = input.chars().collect::<Vec<char>>();
        if chars.iter().all(|c| c.is_ascii_digit()) {
            let num_1 = chars[0].to_digit(10).and_then(|n| n.to_u8()).unwrap_or_default();
            let num_2 = chars[1].to_digit(10).and_then(|n| n.to_u8()).unwrap_or_default();
            let num_3 = chars[2].to_digit(10).and_then(|n| n.to_u8()).unwrap_or_default();
            let num_4 = chars[3].to_digit(10).and_then(|n| n.to_u8()).unwrap_or_default();
            let num_5 = chars[4].to_digit(10).and_then(|n| n.to_u8()).unwrap_or_default();
            let num_6 = chars[5].to_digit(10).and_then(|n| n.to_u8()).unwrap_or_default();
            Some([num_1, num_2, num_3, num_4, num_5, num_6])
        } else {
            None
        }
    } else {
        None
    }
}

// pub fn hash_to_base64_string(bytes: &[u8]) -> String {
//     let mut hasher = DefaultHasher::new();
//     bytes.hash(&mut hasher);
//     let result = hasher.finish();
//     u64_to_base64(result)
// }

// ===== ===== ===== //
//   KPHIS Related   //
// ===== ===== ===== //

pub fn set_day_last(first_date: Option<Date>, last_date: Option<Date>, start_date_mutable: Mutable<String>, end_date_mutable: Mutable<String>, changed_mutable: Mutable<bool>, days: u64) {
    let now = js_now().date();
    let end_date = last_date.unwrap_or(now);
    end_date_mutable.set_neq(end_date.to_string());
    if days == 0 {
        start_date_mutable.set_neq(first_date.unwrap_or(now).to_string());
    } else {
        start_date_mutable.set_neq((end_date - Duration::new((days - 1) * 24 * 3600, 0)).to_string());
    }
    changed_mutable.set_neq(true);
}

pub fn set_days_next(start_date_mutable: Mutable<String>, end_date_mutable: Mutable<String>, changed_mutable: Mutable<bool>, forward: bool) {
    let start_opt = date_8601(&start_date_mutable.lock_ref());
    let end_opt = date_8601(&end_date_mutable.lock_ref());
    if let (Some(start), Some(end)) = (start_opt, end_opt) {
        let diff = end - start + Duration::new(24 * 60 * 60, 0);
        if forward {
            start_date_mutable.set_neq((start + diff).to_string());
            end_date_mutable.set_neq((end + diff).to_string());
        } else {
            start_date_mutable.set_neq((start - diff).to_string());
            end_date_mutable.set_neq((end - diff).to_string());
        }
        changed_mutable.set_neq(true);
    }
}

/// remove dots, split by ` `, get the first ICD10 with dot removed
pub fn find_first_icd10(search_text: &str) -> Option<String> {
    let no_dot = search_text.replace('.', "");
    no_dot.split(' ').find(|s| is_icd10_without_dot(s)).map(|s| s.to_string())
}

/// `A00`, `a000`, `B1234`, `b12345`
pub fn is_icd10_without_dot(text: &str) -> bool {
    if text.len() < 3 || ["B12", "B15", "b12", "b15"].contains(&text) {
        false
    } else {
        let mut chars = text.chars();
        let first_is_alphabet = chars.next().map(|c| c.is_ascii_alphabetic()).unwrap_or_default();
        let mut rest_is_number = false;
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                rest_is_number = true;
            } else {
                rest_is_number = false;
                break;
            }
        }
        first_is_alphabet && rest_is_number
    }
}

/// check only first 3 chars, support `A00`, `A09.9`, `A01-A09`, `A01.-`
/// - NOTE: MUST start with Uppercase charactor
/// - `B12` and `B15` is vitamin names, `B12` and 'B15' not a valid ICD10
///    we  
pub fn is_icd10_resemble(text: &str) -> bool {
    if text.len() < 3 || ["B12", "B15"].contains(&text) {
        false
    } else {
        let mut chars = text.chars();
        let first_is_uppercase = chars.next().map_or(false, |c| c.is_ascii_uppercase());
        let second_is_number = chars.next().map_or(false, |c| c.is_ascii_digit());
        let third_is_number = chars.next().map_or(false, |c| c.is_ascii_digit());
        first_is_uppercase && second_is_number && third_is_number
    }
}

pub fn is_icd9_without_dot(text: &str) -> bool {
    text.chars().all(|c| c.is_ascii_digit())
}

/// to ICD10 uppercase with dot
pub fn icd10_dot(icd10: &str) -> String {
    let mut code = icd10.to_owned();
    code.retain(|c| c.is_ascii());
    if code.contains('.') {
        code.to_ascii_uppercase()
    } else if code.len() > 3 {
        let mut s = code.to_ascii_uppercase();
        s.insert(3, '.');
        s
    } else {
        code.to_ascii_uppercase()
    }
}

pub fn icd9_dot(icd9: &str) -> String {
    let mut code = icd9.to_owned();
    code.retain(|c| c.is_ascii());
    if code.contains('.') {
        code.to_owned()
    } else if code.len() > 2 {
        let mut s = code.to_owned();
        s.insert(2, '.');
        s
    } else {
        code.to_owned()
    }
}

pub fn icd_dash(icd10: &str, is_valid: bool) -> String {
    if is_valid {
        icd10.to_owned()
    } else if icd10.contains('.') {
        [icd10, "-"].concat()
    } else {
        [icd10, ".-"].concat()
    }
}

/// for ICD code
pub fn next_key(key: &str) -> String {
    if key.is_empty() {
        String::from("0")
    } else {
        let (left, right) = key.split_at(key.len() - 1);
        if let Some(end) = right.chars().next().and_then(|c| (c..=char::MAX).nth(1)) {
            [left, &end.to_string()].concat()
        } else {
            [left, &char::MAX.to_string()].concat()
        }
    }
}

/// create image for css `background-image: url(xx);`, error will be `none`<br>
/// remove `<?xml version="1.0" encoding="UTF-8"?>` and convert to `url('data:image/svg+xml;utf8,<svg>..</svg>')`
pub fn svg_to_data_url(svg: &str) -> String {
    let maybe_svg = svg.trim_start_matches(r#"<?xml version="1.0" encoding="UTF-8"?>"#).trim();
    if maybe_svg.starts_with("<svg") {
        // // base64 method, create `url("data:image/svg+xml;base64,XXX")`
        // let image_base64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(maybe_svg.as_bytes());
        // ["url(\"data:image/svg+xml;base64,", &image_base64, "\")"].concat()

        // // svg url-encoding method, create `url('data:image/svg+xml;utf8,<svg>..</svg>')`
        // can use js_sys::encode_uri_component here
        ["url('data:image/svg+xml;utf8,", &urlencoding::encode(maybe_svg), "')"].concat()
    } else {
        String::from("none")
    }
}

pub fn zoom_step(base: f64, is_up: bool) -> f64 {
    let tail = base % 25.0;
    if is_up && base >= 450.0 {
        500.0
    } else if !is_up && base <= 50.0 {
        25.0
    } else {
        let step = match base {
            ..200.0 => 25.0,
            _ => 50.0,
        };
        let base25 = if tail > 12.0 { base + 25.0 - tail } else { base - tail };
        if is_up { base25 + step } else { base25 - step }
    }
}

pub fn pre_order_type_display(data: &str) -> &'static str {
    match data {
        "appointment" => "Admit ล่วงหน้า",
        "opd" => "Admit ในวัน",
        _ => "Template",
    }
}

/// (`2`,`45`) to `2'45"`, (`2`,`100`) to `3'40"`
pub fn lr_int_to_quote(minutes: &str, seconds: &str) -> String {
    let m_opt = minutes.parse::<u32>().ok();
    let s = seconds.parse::<u32>().unwrap_or_default();
    let s_overflow_m = s / 60;
    let s_remainer = s % 60;
    let (s_remainer_v, s_remainer_u) = if s_remainer > 0 { (s_remainer.to_string(), "\"") } else { (String::new(), "") };
    match m_opt {
        Some(m) => [&(m + s_overflow_m).to_string(), "'", &s_remainer_v, s_remainer_u].concat(),
        None => {
            if s_overflow_m > 0 {
                [&s_overflow_m.to_string(), "'", &s_remainer_v, s_remainer_u].concat()
            } else {
                [&s_remainer_v, s_remainer_u].concat()
            }
        }
    }
}

/// `2'45"` to (2,45), `2'100"` to (3,40)
pub fn lr_int_from_quote(quote_text: &str) -> (u32, u32) {
    let split = quote_text.split("'").map(str::trim).collect::<Vec<&str>>();
    if split.len() > 1 {
        let m = split[0].parse::<u32>().ok().unwrap_or_default();
        lr_int_from_quote_inner(m, split[1])
    } else {
        lr_int_from_quote_inner(0, split[0])
    }
}

fn lr_int_from_quote_inner(m: u32, s_str: &str) -> (u32, u32) {
    let s = s_str.trim_end_matches("\"").trim_end().parse::<u32>().unwrap_or_default();
    let s_overflow_m = s / 60;
    let s_reminder = s % 60;
    (m + s_overflow_m, s_reminder)
}

pub fn get_day_dose_from_detail(detail: &str) -> i32 {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

    let every = detail.split("ทุก").collect::<Vec<&str>>();
    let q = detail.split(" q ").collect::<Vec<&str>>();
    let v = if every.len() > 1 {
        let left = find_num_iter(every[0], &RE);
        let mut right = find_num_iter(every[1], &RE);
        left.last().unwrap_or(1) * (24 / right.next().unwrap_or(1))
    } else if q.len() > 1 {
        let left = find_num_iter(q[0], &RE);
        let mut right = find_num_iter(q[1], &RE);
        left.last().unwrap_or(1) * (24 / right.next().unwrap_or(1))
    } else {
        let mut iter = find_num_iter(detail, &RE);
        iter.next().unwrap_or(1) * iter.next().unwrap_or(1)
    };
    if v > 50 { 1 } else { v }
}

pub fn los_f32_to_u32(los_f32: f32) -> u32 {
    let floor_f32 = los_f32.floor();
    let floor_u32 = floor_f32 as u32;
    if los_f32 - floor_f32 > 0.25 { floor_u32 + 1 } else { floor_u32 }
}

fn find_num_iter(text: &str, re: &Regex) -> impl Iterator<Item = i32> {
    re.find_iter(text).map(|r| r.as_str().parse::<i32>().unwrap_or(1))
}

/// get first match Decimal in text
pub fn find_decimal_in_text(text: &str) -> Option<Decimal> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\d.]+").unwrap());
    RE.find(text).and_then(|r| Decimal::from_str_exact(r.as_str()).ok())
}

/// split long drug usage into 3 lines
pub fn split_to_three(input: &str, is_ivfluid: bool) -> (String, String, String) {
    let line1_limit = 25;
    let line2_limit = 35;

    let input = input.trim();
    let splited = input.split(" ").collect::<Vec<&str>>();
    let w = splited.iter().map(|s| s.width()).sum::<usize>();

    if is_ivfluid {
        let iv_line_2 = String::from("เริ่ม D.............................T....................");
        let iv_line_3 = String::from("หมด D.............................T....................");
        // can fill line1
        if w < line1_limit {
            (input.to_owned(), iv_line_2, iv_line_3)
        // cannot fill line 1, is solvent
        } else if input.starts_with("ผสม") {
            let key_pos2 = splited.iter().position(|s| *s == "IV");
            // has IV
            if let Some(pos2) = key_pos2 {
                let (left_mid, right) = splited.split_at(pos2);
                let key_pos1 = left_mid.iter().position(|s| s.starts_with("ขนาด"));
                // has ขนาด
                if let Some(pos1) = key_pos1 {
                    let (left, mid) = left_mid.split_at(pos1);
                    (left.join(" "), mid.join(" "), right.join(" "))
                // without ขนาด, cannot fill line1
                } else if left_mid.iter().fold(0, |acc, v| acc + v.width()) > line1_limit {
                    let (line1, line2) = split_to_2(left_mid, line1_limit, false);
                    (line1.join(" "), line2.join(" "), right.join(" "))
                // without ขนาด, can fill line1
                } else {
                    (left_mid.join(" "), right.join(" "), String::new())
                }
            // without IV
            } else {
                let (line1, line2, line3) = split_to_3(&splited, line1_limit, line2_limit, false);
                (line1.join(" "), line2.join(" "), line3.join(" "))
            }
        // cannot fill line 1, not solvent
        } else {
            let (line1, line2, line3) = split_to_3(&splited, line1_limit, line2_limit, false);
            (line1.join(" "), line2.join(" "), line3.join(" "))
        }
    // not iv fluid
    } else {
        let key_pos = splited.iter().position(|s| s.starts_with("วันละ") || s.starts_with("ทุก"));
        // has some keyword
        if let Some(pos) = key_pos {
            let (left, right) = splited.split_at(pos);
            let left_w = left.iter().fold(0, |acc, v| acc + v.width());
            let right_w = left.iter().fold(0, |acc, v| acc + v.width());
            // not fit in both 2 lines
            if left_w > line1_limit && right_w > line2_limit {
                // will split left
                if left_w > right_w {
                    let (line1, line2) = split_to_2(left, line1_limit, true);
                    (line1.join(" "), line2.join(" "), right.join(" "))
                // will split right
                } else {
                    let (line2, line3) = split_to_2(right, line2_limit, true);
                    (left.join(" "), line2.join(" "), line3.join(" "))
                }
            // only left need split
            } else if left_w > line1_limit {
                let (line1, line2) = split_to_2(left, line1_limit, true);
                (line1.join(" "), line2.join(" "), right.join(" "))
            // only right need split
            } else {
                let (line2, line3) = split_to_2(right, line2_limit, true);
                (left.join(" "), line2.join(" "), line3.join(" "))
            }
        // without keyword
        } else {
            if w > line1_limit {
                let (line1, line2, line3) = split_to_3(&splited, line1_limit, line2_limit, true);
                (line1.join(" "), line2.join(" "), line3.join(" "))
            } else {
                (input.to_owned(), String::new(), String::new())
            }
        }
    }
}

fn split_to_2<'a>(raw: &'a [&str], left_limit: usize, or_end_with_word: bool) -> (Vec<&'a str>, Vec<&'a str>) {
    let w = raw.iter().map(|s| s.width()).sum::<usize>();
    let mut count = 0;
    let mut has_qouta = true;
    let mut line1 = Vec::new();
    let mut line2 = Vec::new();
    for word in raw {
        count = count + word.width();
        if count < left_limit {
            line1.push(*word);
        } else if has_qouta {
            if or_end_with_word && (*word == "เม็ด" || (w - count) < 3) {
                line1.push(*word);
            } else {
                line2.push(*word);
            }
            has_qouta = false;
        } else {
            line2.push(*word);
        }
    }
    (line1, line2)
}

fn split_to_3<'a>(raw: &'a [&str], line1_limit: usize, line2_limit: usize, or_end_with_word: bool) -> (Vec<&'a str>, Vec<&'a str>, Vec<&'a str>) {
    let w = raw.iter().map(|s| s.width()).sum::<usize>();
    let mut count = 0;
    let mut has_1st_qouta = true;
    let mut has_2nd_qouta = true;
    let mut line1 = Vec::new();
    let mut line2 = Vec::new();
    let mut line3 = Vec::new();
    for word in raw {
        count = count + word.width();
        if count == word.width() {
            line1.push(*word);
        } else if count < line1_limit {
            line1.push(*word);
        } else if has_1st_qouta {
            if or_end_with_word && (*word == "เม็ด" || (w - count) < 3) {
                line1.push(*word);
            } else {
                line2.push(*word);
            }
            has_1st_qouta = false;
        } else if count < (line1_limit + line2_limit) {
            line2.push(*word);
        } else if has_2nd_qouta {
            if or_end_with_word && (*word == "เม็ด" || (w - count) < 3) {
                line2.push(*word);
            } else {
                line3.push(*word);
            }
            has_2nd_qouta = false;
        } else {
            line3.push(*word);
        }
    }
    (line1, line2, line3)
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use std::u64;

    use super::*;
    use rust_decimal::Decimal;
    use time::{Date, Month};

    #[test]
    fn test_add_u64_with_i64() {
        assert_eq!(add_u64_with_i64(100, 10), 110);
        assert_eq!(add_u64_with_i64(100, -10), 90);
        assert_eq!(add_u64_with_i64(u64::MIN, -10), u64::MIN);
        assert_eq!(add_u64_with_i64(u64::MAX, 10), u64::MAX);
    }

    #[test]
    fn test_sanity_alphanumeric() {
        assert_eq!(sanity_alphanumeric("<script>alert()</script>123"), String::from("scriptalertscript123"));
        assert_eq!(sanity_alphanumeric("\" or 1=1--"), String::from("or11"));
    }

    #[test]
    fn test_sanity_int() {
        assert_eq!(sanity_int("<script>alert()</script>123"), String::from("123"));
        assert_eq!(sanity_int("\" or 1=1--"), String::from("11"));
    }

    // #[test]
    // fn test_sanity_sp() {
    //     assert_eq!(sanity_sp("<script>alert()</script>123"), String::from("scriptalertscript123"));
    //     assert_eq!(sanity_sp("\" or 1=1--"), String::from("or11"));
    // }

    #[test]
    fn test_sanity_dot_space() {
        assert_eq!(sanity_dot_space("   abc...123   "), String::from("abc 123"));
        assert_eq!(sanity_dot_space("....abc   123.."), String::from("abc 123"));
        assert_eq!(sanity_dot_space(" start...1...every ...  123 ..... hour "), String::from("start 1 every 123 hour")); // trimmed
        assert_eq!(sanity_dot_space("เช้า…24 ยูนิต, เย็น... 16..... ยูนิต"), String::from("เช้า…24 ยูนิต, เย็น 16 ยูนิต"));
        assert_eq!(sanity_dot_space("1.53  ps."), String::from("1.53 ps."));
        assert_eq!(sanity_dot_space(" .5 . "), String::from("5"));
        assert_eq!(sanity_dot_space(" ps. hi.. "), String::from("ps hi")); // trimmed
        assert_eq!(sanity_dot_space(""), String::from(""));
    }

    #[test]
    fn test_sanity_space() {
        assert_eq!(sanity_space("   abc...123   "), String::from("abc...123"));
        assert_eq!(sanity_space("....abc   123.."), String::from("....abc 123.."));
        assert_eq!(sanity_space(" start...1...every ...  123 ..... hour "), String::from("start...1...every ... 123 ..... hour")); // trimmed
        assert_eq!(sanity_space("เช้า…24 ยูนิต, เย็น... 16..... ยูนิต"), String::from("เช้า…24 ยูนิต, เย็น... 16..... ยูนิต"));
        assert_eq!(sanity_space("1.53  ps."), String::from("1.53 ps."));
        assert_eq!(sanity_space(" .5 . "), String::from(".5 ."));
        assert_eq!(sanity_space(" ps. hi.. "), String::from("ps. hi..")); // trimmed
        assert_eq!(sanity_space(""), String::from(""));
    }

    #[test]
    fn test_first_char_uppercase() {
        assert_eq!(first_char_uppercase("hello"), String::from("Hello"));
        assert_eq!(first_char_uppercase(" hello"), String::from(" hello"));
    }

    #[test]
    fn test_sanity_tis620() {
        assert_eq!(sanity_tis620("What 33 ? @#$%!^&*(){}[]<>,.'\""), String::from("What 33 ? @#$%!^&*(){}[]<>,.'\""));
        assert_eq!(sanity_tis620("สวัสดี ยินดีต้อนรับ"), String::from("สวัสดี ยินดีต้อนรับ"));
        assert_eq!(sanity_tis620("สวัสดี ☑ ยินดีต้อนรับ"), String::from("สวัสดี  ยินดีต้อนรับ"));
        assert_eq!(sanity_tis620("เช้า…24 ยู€‘’“”•–—นิต"), String::from("เช้า24 ยูนิต"));
        assert_eq!(sanity_tis620("BT 37.5 °C M₁ O\u{2082}sat 99% D₃ ∆P 0.3"), String::from("BT 37.5 ํC M1 O2sat 99% D3 delta-P 0.3"));
    }

//     #[test]
//     fn test_whitespace_to_space() {
//         assert_eq!(whitespace_to_space("abc\r\ndef"), String::from("abc  def"));
//         assert_eq!(whitespace_to_space("abc\rdef"), String::from("abc def"));
//         assert_eq!(whitespace_to_space("abc\ndef"), String::from("abc def"));
//         assert_eq!(whitespace_to_space(r"abc
// def"), String::from("abc def"));
//     }

    #[test]
    fn test_decimal_rescale() {
        assert_eq!(decimal_rescale(Decimal::new(999, 0), 2), Decimal::new(999, 0));
        assert_eq!(decimal_rescale(Decimal::new(994, 2), 1), Decimal::new(99, 1));
        assert_eq!(decimal_rescale(Decimal::new(995, 2), 1), Decimal::new(100, 1));
        assert_eq!(decimal_rescale(Decimal::new(-994, 2), 1), Decimal::new(-99, 1));
        assert_eq!(decimal_rescale(Decimal::new(-995, 2), 1), Decimal::new(-100, 1));
        assert_eq!(decimal_rescale(Decimal::new(999, 2), 3), Decimal::new(9990, 3));
    }

    #[test]
    fn test_f64_rescale() {
        assert_eq!(f64_rescale(999.0, 2), 999.0);
        assert_eq!(f64_rescale(9.94, 1), 9.9);
        assert_eq!(f64_rescale(9.95, 1), 10.0);
        assert_eq!(f64_rescale(-9.94, 1), -9.9);
        assert_eq!(f64_rescale(-9.95, 1), -10.0);
        assert_eq!(f64_rescale(9.99, 3), 9.990);
    }

    #[test]
    fn test_u64_to_base64() {
        assert_eq!(u64_to_base64(123456789u64), String::from("AAAAAAdbzRU="));
        assert_eq!(u64_to_base64(u64::MAX), String::from("__________8="));
    }

    #[test]
    fn test_date_8601() {
        assert_eq!(date_8601("2024-01-30"), Date::from_calendar_date(2024, Month::January, 30).ok());
        assert_eq!(date_8601("2024-01-33"), None);
    }

    #[test]
    fn test_find_decimal_in_text() {
        assert_eq!(find_decimal_in_text("34.5"), Some(Decimal::new(345, 1)));
        assert_eq!(find_decimal_in_text("stat=33%"), Some(Decimal::new(33, 0)));
        assert_eq!(find_decimal_in_text("ได้ 33.5-40.5 mg/dl"), Some(Decimal::new(335, 1)));
    }

    #[test]
    fn test_thousand() {
        assert_eq!(thousands("12"), String::from("12"));
        assert_eq!(thousands("123.4"), String::from("123.4"));
        assert_eq!(thousands("1234"), String::from("1,234"));
        assert_eq!(thousands("1234.5"), String::from("1,234.5"));
        assert_eq!(thousands("1234567890.123456789"), String::from("1,234,567,890.123456789"));
        assert_eq!(thousands("abcd.efgh"), String::from("a,bcd.efgh"));
        assert_eq!(thousands("-12"), String::from("-12"));
        assert_eq!(thousands("-123.4"), String::from("-123.4"));
        assert_eq!(thousands("-1234"), String::from("-1,234"));
        assert_eq!(thousands("-1234.5"), String::from("-1,234.5"));
        assert_eq!(thousands("-1234567890.123456789"), String::from("-1,234,567,890.123456789"));
        assert_eq!(thousands("-abcd.efgh"), String::from("-a,bcd.efgh"));
        assert_eq!(thousands("1234.5678.9"), String::from("1,234.5678.9"));
    }

    #[test]
    fn test_find_first_icd10() {
        assert_eq!(find_first_icd10("A0"), None);
        assert_eq!(find_first_icd10("a09"), Some(String::from("a09")));
        assert_eq!(find_first_icd10("any a099"), Some(String::from("a099")));
        assert_eq!(find_first_icd10("some A09.9"), Some(String::from("A099")));
        assert_eq!(find_first_icd10("both a09.0.1 and A099"), Some(String::from("a0901")));
        assert_eq!(find_first_icd10("maybe a09b or A09c"), None);
        assert_eq!(find_first_icd10("B12"), None);
        assert_eq!(find_first_icd10("B15"), None);
        assert_eq!(find_first_icd10("Vitamin b12"), None);
        assert_eq!(find_first_icd10("vitamin b15"), None);
        assert_eq!(find_first_icd10("both vitamin b15 and b99"), Some(String::from("b99")));
    }

    #[test]
    fn test_is_icd10_without_dot() {
        assert!(!is_icd10_without_dot("A"));
        assert!(!is_icd10_without_dot("Aa"));
        assert!(!is_icd10_without_dot("A9"));
        assert!(is_icd10_without_dot("a99"));
        assert!(is_icd10_without_dot("A999"));
        assert!(is_icd10_without_dot("B121"));
        assert!(is_icd10_without_dot("B151"));
        assert!(!is_icd10_without_dot("B12"));
        assert!(!is_icd10_without_dot("B15"));
        assert!(!is_icd10_without_dot("b12"));
        assert!(!is_icd10_without_dot("b15"));
        assert!(!is_icd10_without_dot("A99.9"));
        assert!(!is_icd10_without_dot("a99.9"));
        assert!(!is_icd10_without_dot("A999a"));
        assert!(!is_icd10_without_dot("A99.-"));
        assert!(!is_icd10_without_dot("A01-A99"));
        assert!(!is_icd10_without_dot("a01-a99"));
    }

    #[test]
    fn test_is_icd10_resemble() {
        assert!(!is_icd10_resemble("A"));
        assert!(!is_icd10_resemble("Aa"));
        assert!(!is_icd10_resemble("A9"));
        assert!(!is_icd10_resemble("a99"));
        assert!(!is_icd10_resemble("B12"));
        assert!(!is_icd10_resemble("B15"));
        assert!(is_icd10_resemble("A999"));
        assert!(is_icd10_resemble("A99.9"));
        assert!(is_icd10_resemble("B15.9"));
        assert!(!is_icd10_resemble("a99.9"));
        assert!(is_icd10_resemble("A999a"));
        assert!(is_icd10_resemble("A99.-"));
        assert!(is_icd10_resemble("B15.-"));
        assert!(is_icd10_resemble("A01-A99"));
        assert!(is_icd10_resemble("B12-B15"));
        assert!(!is_icd10_resemble("a01-a99"));
    }

    #[test]
    fn test_svg_to_data_url() {
        assert_eq!(
            svg_to_data_url(r#"<?xml version="1.0" encoding="UTF-8"?> <svg width="800" height="600" version="1.1" xmlns="http://www.w3.org/2000/svg"><g><path d="m278.7 2.213-4.155-2.727-4.643z"/></g></svg>"#),
            // String::from(r#"url("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iODAwIiBoZWlnaHQ9IjYwMCIgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxnPjxwYXRoIGQ9Im0yNzguNyAyLjIxMy00LjE1NS0yLjcyNy00LjY0M3oiLz48L2c+PC9zdmc+")"#),
            String::from(r#"url('data:image/svg+xml;utf8,%3Csvg%20width%3D%22800%22%20height%3D%22600%22%20version%3D%221.1%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%3E%3Cg%3E%3Cpath%20d%3D%22m278.7%202.213-4.155-2.727-4.643z%22%2F%3E%3C%2Fg%3E%3C%2Fsvg%3E')"#),
        );
        assert_eq!(
            svg_to_data_url(r#"<svg width="800" height="600" version="1.1" xmlns="http://www.w3.org/2000/svg"><g><path d="m278.7 2.213-4.155-2.727-4.643z"/></g></svg>"#),
            // String::from(r#"url("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iODAwIiBoZWlnaHQ9IjYwMCIgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxnPjxwYXRoIGQ9Im0yNzguNyAyLjIxMy00LjE1NS0yLjcyNy00LjY0M3oiLz48L2c+PC9zdmc+")"#),
            String::from(r#"url('data:image/svg+xml;utf8,%3Csvg%20width%3D%22800%22%20height%3D%22600%22%20version%3D%221.1%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%3E%3Cg%3E%3Cpath%20d%3D%22m278.7%202.213-4.155-2.727-4.643z%22%2F%3E%3C%2Fg%3E%3C%2Fsvg%3E')"#),
        );
        assert_eq!(svg_to_data_url(r#"<g><path d="m278.7 2.213-4.155-2.727-4.643z"/></g>"#), String::from("none"),);
    }

    #[test]
    fn test_raw_concat_to_comma_equal() {
        assert_eq!(raw_concat_to_comma_equal("A a B b C c"), String::from("A=a, B=b, C=c"));
        assert_eq!(raw_concat_to_comma_equal("A a B b C c D"), String::from("A=a, B=b, C=c"));

        assert_eq!(raw_concat_to_comma_equal("A^a|B^b|C^c"), String::from("A=a, B=b, C=c"));
        assert_eq!(raw_concat_to_comma_equal("A^a^1|B^b|C"), String::from("A=a, B=b"));
    }

    #[test]
    fn test_explode_cap_pipe() {
        assert_eq!(explode_cap_pipe("A^a^1|B^b|C^c|D", 2), vec![
            vec![String::from("A"), String::from("a")],
            vec![String::from("B"), String::from("b")],
            vec![String::from("C"), String::from("c")],
        ]);
        assert_eq!(explode_cap_pipe("A^a^1|B^b|C^c|D", 3), vec![
            vec![String::from("A"), String::from("a"), String::from("1")],
        ]);
    }

    #[test]
    fn test_explode_space() {
        assert_eq!(explode_space("A a B b C c D", 2), vec![
            vec![String::from("A"), String::from("a")],
            vec![String::from("B"), String::from("b")],
            vec![String::from("C"), String::from("c")],
        ]);
        assert_eq!(explode_space("A a B b C c D", 3), vec![
            vec![String::from("A"), String::from("a"), String::from("B")],
            vec![String::from("b"), String::from("C"), String::from("c")],
        ]);
    }

    #[test]
    fn test_text_to_six_digits() {
        assert_eq!(text_to_six_digits("123456"), Some([1,2,3,4,5,6]));
        assert_eq!(text_to_six_digits("000000"), Some([0,0,0,0,0,0]));
        assert_eq!(text_to_six_digits("999999"), Some([9,9,9,9,9,9]));
        assert_eq!(text_to_six_digits(""), None);
        assert_eq!(text_to_six_digits("12345"), None);
        assert_eq!(text_to_six_digits("1234567"), None);
        assert_eq!(text_to_six_digits("a23456"), None);
    }

    #[test]
    fn test_zoom_step() {
        assert_eq!(zoom_step(25.0, true), 50.0);
        assert_eq!(zoom_step(50.0, true), 75.0);
        assert_eq!(zoom_step(75.0, true), 100.0);
        assert_eq!(zoom_step(100.0, true), 125.0);
        assert_eq!(zoom_step(125.0, true), 150.0);
        assert_eq!(zoom_step(150.0, true), 175.0);
        assert_eq!(zoom_step(175.0, true), 200.0);
        assert_eq!(zoom_step(200.0, true), 250.0);
        assert_eq!(zoom_step(250.0, true), 300.0);
        assert_eq!(zoom_step(300.0, true), 350.0);
        assert_eq!(zoom_step(350.0, true), 400.0);
        assert_eq!(zoom_step(400.0, true), 450.0);
        assert_eq!(zoom_step(450.0, true), 500.0);
        assert_eq!(zoom_step(500.0, true), 500.0);

        assert_eq!(zoom_step(500.0, false), 450.0);
        assert_eq!(zoom_step(450.0, false), 400.0);
        assert_eq!(zoom_step(400.0, false), 350.0);
        assert_eq!(zoom_step(350.0, false), 300.0);
        assert_eq!(zoom_step(300.0, false), 250.0);
        assert_eq!(zoom_step(250.0, false), 200.0);
        assert_eq!(zoom_step(200.0, false), 150.0);
        assert_eq!(zoom_step(150.0, false), 125.0);
        assert_eq!(zoom_step(125.0, false), 100.0);
        assert_eq!(zoom_step(100.0, false), 75.0);
        assert_eq!(zoom_step(75.0, false), 50.0);
        assert_eq!(zoom_step(50.0, false), 25.0);
        assert_eq!(zoom_step(25.0, false), 25.0);
    }

    #[test]
    fn test_lr_int_to_quote() {
        assert_eq!(lr_int_to_quote("2","45"), String::from("2'45\""));
        assert_eq!(lr_int_to_quote("2","100"), String::from("3'40\""));
        assert_eq!(lr_int_to_quote("a","45"), String::from("45\""));
        assert_eq!(lr_int_to_quote("a","100"), String::from("1'40\""));
        assert_eq!(lr_int_to_quote("2","b"), String::from("2'"));
        assert_eq!(lr_int_to_quote("a","b"), String::new());
        assert_eq!(lr_int_to_quote("-2","45"), String::from("45\""));
        assert_eq!(lr_int_to_quote("-2","100"), String::from("1'40\""));
        assert_eq!(lr_int_to_quote("2","-45"), String::from("2'"));
        assert_eq!(lr_int_to_quote("-2","-45"), String::new());
    }

    #[test]
    fn test_lr_int_from_quote() {
        assert_eq!(lr_int_from_quote("2'45\""), (2,45));
        assert_eq!(lr_int_from_quote("2'100\""), (3,40));
        assert_eq!(lr_int_from_quote("2'"), (2,0));
        assert_eq!(lr_int_from_quote("45\""), (0,45));
        assert_eq!(lr_int_from_quote(""), (0,0));
        assert_eq!(lr_int_from_quote("a'45\""), (0,45));
        assert_eq!(lr_int_from_quote("2'b\""), (2,0));
        assert_eq!(lr_int_from_quote("a'b\""), (0,0));
        assert_eq!(lr_int_from_quote("-2'45\""), (0,45));
        assert_eq!(lr_int_from_quote("2'-45\""), (2,0));
        assert_eq!(lr_int_from_quote("-2'-45\""), (0,0));
        // supported varients
        assert_eq!(lr_int_from_quote("2'45"), (2,45));
        assert_eq!(lr_int_from_quote(" 2 ' 45 \" "), (2,45));
        assert_eq!(lr_int_from_quote("45"), (0,45));
    }

    #[test]
    fn test_get_day_dose_in_detail() {
        assert_eq!(get_day_dose_from_detail("2*3"), 6);
        assert_eq!(get_day_dose_from_detail("12-13"), 1); // overflow 50 to 1
        assert_eq!(get_day_dose_from_detail("between24hour"), 24);
        assert_eq!(get_day_dose_from_detail("3 tab 4 times a day in 2 weeks"), 12); // only first 2 number
        assert_eq!(get_day_dose_from_detail("no any number"), 1);
        assert_eq!(get_day_dose_from_detail("รับประทาน 2 เม็ด ทุก 12 ชม"), 4);
        assert_eq!(get_day_dose_from_detail("12/7/68 รับประทาน 2 เม็ด ทุก 12 ชม 30 นาที"), 4); // only number around `ทุก`
        assert_eq!(get_day_dose_from_detail("รับประทาน 2 เม็ดทุกๆ 12 ชม"), 4);
        assert_eq!(get_day_dose_from_detail("take 2 tab prn q 12 hr"), 4);
        assert_eq!(get_day_dose_from_detail("12/7/68 take 2 tab prn q 12 hr 30 minutes"), 4); // only number around ` q `
        assert_eq!(get_day_dose_from_detail("take 2 tab q12 hr"), 24); // failed ` q `, go default x*y
        assert_eq!(get_day_dose_from_detail("take 2 tabq 12hr"), 24); // failed ` q `, go default x*y
        assert_eq!(get_day_dose_from_detail("600 mg iv q 8hr"), 1); // overflow 50 to 1
    }
}
