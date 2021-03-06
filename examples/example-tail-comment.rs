//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use chrono::{
    prelude::{DateTime, Utc},
    FixedOffset, NaiveDate, NaiveDateTime, NaiveTime,
};
use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/example-tail-comment.toml");

    // Read a number.
    // 数値読取。
    assert!(doc.contains_key("age"));
    assert_eq!(doc.get_i128_by_key_v2("age"), Ok(Some(40)));
    assert_eq!(doc.get_isize_by_key_v2("age"), Ok(Some(40)));
    assert_eq!(doc.get_u128_by_key_v2("age"), Ok(Some(40)));
    assert_eq!(doc.get_usize_by_key_v2("age"), Ok(Some(40)));
    assert_eq!(doc.get_f64_by_key_v2("weight"), Ok(Some(93.5)));

    assert_eq!(doc.get_i128_by_key_v2("i32_max"), Ok(Some(2147483647)));
    assert_eq!(doc.get_i128_by_key_v2("i32_min"), Ok(Some(-2147483648)));

    assert_eq!(
        doc.get_i128_by_key_v2("i128_max"),
        Ok(Some(170_141_183_460_469_231_731_687_303_715_884_105_727))
    );
    assert_eq!(
        doc.get_i128_by_key_v2("i128_min"),
        Ok(Some(-170_141_183_460_469_231_731_687_303_715_884_105_728))
    );
    assert_eq!(
        doc.get_u128_by_key_v2("u128_max"),
        Ok(Some(340_282_366_920_938_463_463_374_607_431_768_211_455))
    );

    assert_eq!(doc.get_i128_by_key_v2("hex1"), Ok(Some(0xDEADBEEF)));
    assert_eq!(doc.get_i128_by_key_v2("hex2"), Ok(Some(0xdeadbeef)));
    assert_eq!(doc.get_i128_by_key_v2("hex3"), Ok(Some(0xdead_beef)));
    assert_eq!(doc.get_i128_by_key_v2("oct1"), Ok(Some(0o01234567)));
    assert_eq!(doc.get_i128_by_key_v2("oct2"), Ok(Some(0o755)));
    assert_eq!(doc.get_i128_by_key_v2("bin1"), Ok(Some(0b11010110)));
    assert_eq!(doc.get_f64_by_key_v2("float1"), Ok(Some(1.0)));
    assert_eq!(doc.get_f64_by_key_v2("float2"), Ok(Some(3.1415)));
    assert_eq!(doc.get_f64_by_key_v2("float3"), Ok(Some(-0.01)));
    assert_eq!(doc.get_f64_by_key_v2("float4"), Ok(Some(5e+22)));
    assert_eq!(doc.get_f64_by_key_v2("float5"), Ok(Some(1e06)));
    assert_eq!(doc.get_f64_by_key_v2("float6"), Ok(Some(-2E-2)));
    assert_eq!(doc.get_f64_by_key_v2("float7"), Ok(Some(6.626e-34)));
    assert_eq!(
        doc.get_f64_by_key_v2("float8"),
        Ok(Some(224_617.445_991_228))
    );
    assert_eq!(doc.get_f64_by_key_v2("infinite1"), Ok(Some(f64::INFINITY)));
    assert_eq!(doc.get_f64_by_key_v2("infinite2"), Ok(Some(f64::INFINITY)));
    assert_eq!(doc.get_f64_by_key_v2("infinite3"), Ok(Some(-f64::INFINITY)));
    assert!(match doc.get_f64_by_key_v2("not1") {
        Ok(_n) => true,
        Err(why) => panic!("{}", why),
    });
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not1") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not2") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not3") {
        n.is_nan() && n.is_sign_negative()
    } else {
        false
    });

    // WIP. Read a string.
    // 作業中。 文字列読取。
    assert_eq!(doc.get_string_by_key("apple"), Some("pie".to_string()));

    assert_eq!(
        doc.get_string_by_key("basic_string_letter"),
        Some("Hello, world!!".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_empty"),
        Some("".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_escape_backslash"),
        Some("\\".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_escape_double_quotation"),
        Some("\"".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_punctuation"),
        Some("., ={}[]'\"\\!?".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_tab"),
        Some("a\tb".to_string())
    );

    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_letter"),
        Some(
            "Hello,
world!!"
                .to_string()
        )
    );

    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_punctuation"),
        Some(
            "., ={}[]\"'\"\"\\
!?"
            .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_trim_start"),
        Some("The quick brown fox jumps over the lazy dog.".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_escape_double_quotation"),
        Some(
            "
\\
"
            .to_string()
        )
    );
    /*
    // Fixed.
    println!(
        "debug|multiline_basic_string_tab|{}",
        doc.get_debug_string_by_key("multiline_basic_string_tab")
    );
    */
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_tab"),
        Some(
            "
a\tb
"
            .to_string()
        )
    );

    assert_eq!(
        doc.get_string_by_key("literal_string_empty"),
        Some("".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("literal_string_letter"),
        Some("Hello, world!!".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("literal_string_punctuation"),
        Some("., ={}[]\"\\!?".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_letter"),
        Some(
            "Hello,
world!!"
                .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_punctuation"),
        Some(
            "., ={}[]'\"\\
!?"
            .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_first_newline_is_trimmed"),
        Some(
            "The first newline is
trimmed in raw strings.
All other whitespace
is preserved.
"
            .to_string()
        )
    );

    // Read a boolean.
    // 論理値読取。
    assert_eq!(doc.get_bool_by_key("adult"), Some(true));
    assert_eq!(doc.get_bool_by_key("student"), Some(false));

    // DateTime.
    // 日付と時刻。
    assert_eq!(
        doc.get_datetime_utc_by_key("dob"),
        Some(
            "1979-05-27T07:32:00-08:00"
                .parse::<DateTime<Utc>>()
                .unwrap()
        )
    );

    assert_eq!(
        doc.get_datetime_utc_by_key("odt1"),
        Some("1979-05-27T07:32:00Z".parse::<DateTime<Utc>>().unwrap())
    );

    assert_eq!(
        doc.get_datetime_fixed_offset_by_key("odt2"),
        Some(
            "1979-05-27T00:32:00-07:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        )
    );

    assert_eq!(
        doc.get_datetime_fixed_offset_by_key("odt3"),
        Some(
            "1979-05-27T00:32:00.999999-07:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        )
    );

    // TODO Local datetime
    assert_eq!(
        // "1979-05-27T07:32:00". Toml の独自書式か。該当するフォーマット定義見つからず。
        doc.get_naive_datetime_by_key("ldt1"),
        Some(
            match NaiveDateTime::parse_from_str("1979-05-27T07:32:00", "%Y-%m-%dT%H:%M:%S") {
                Ok(n) => n,
                Err(why) => panic!("{}", why),
            }
        )
    );

    assert_eq!(
        // "1979-05-27T00:32:00.999999".
        doc.get_naive_datetime_by_key("ldt2"),
        Some(
            NaiveDateTime::parse_from_str("1979-05-27T00:32:00.999999", "%Y-%m-%dT%H:%M:%S%.6f")
                .unwrap()
        )
    );

    assert_eq!(
        // "1979-05-27".
        doc.get_naive_date_by_key("ld1"),
        Some(match NaiveDate::parse_from_str("1979-05-27", "%Y-%m-%d") {
            Ok(n) => n,
            Err(why) => panic!("{}", why),
        })
    );

    assert_eq!(
        doc.get_naive_time_by_key("lt1"),
        Some(NaiveTime::parse_from_str("07:32:00", "%H:%M:%S").unwrap())
    );

    assert_eq!(
        doc.get_naive_time_by_key("lt2"),
        Some(NaiveTime::parse_from_str("00:32:00.999999", "%H:%M:%S%.6f").unwrap())
    );
}
