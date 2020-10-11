//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value`

use casual_logger::{Level, Log, Table};
use tomboy_toml_dom::Toml;

pub trait LogExt {
    fn println(s: &str);
    fn println_t(s: &str, t: &mut Table);
}
impl LogExt for Log {
    /// Info level logging and add print to stdout.
    fn println(s: &str) {
        if Log::enabled(Level::Info) {
            println!("{}", s);
        }
        Log::infoln(s);
    }

    /// Info level logging and add print to stdout.
    fn println_t(s: &str, t: &mut Table) {
        if Log::enabled(Level::Info) {
            println!("{}", s);
        }
        Log::infoln_t(s, t);
    }
}

fn main() {
    Log::println("Start.");
    Log::set_file_name("key-value");
    Log::remove_old_logs();
    let doc = Toml::from_file("./resource/key-value.toml");
    Log::println_t(
        "Count document elements.",
        Table::default().uint("DocumentElementsCount", doc.elements.len() as u128),
    );

    // Test.
    let a_name = "int_1";
    let a_value = if let Some(elem) = doc.child(a_name) {
        format!("{:?}", elem)
    } else {
        format!("NotFound")
    };
    Log::println_t("Find int_1=", Table::default().str(a_name, &a_value));
    // Test.
    let a_name = "float_1";
    let a_value = if let Some(elem) = doc.child(a_name) {
        format!("{:?}", elem)
    } else {
        format!("NotFound")
    };
    Log::println_t("Find float_1=", Table::default().str(a_name, &a_value));
    // Test.
    let a_name = "sqstr_1";
    let a_value = if let Some(elem) = doc.child(a_name) {
        format!("{:?}", elem)
    } else {
        format!("NotFound")
    };
    Log::println_t("Find sqstr_1=", Table::default().str(a_name, &a_value));

    Log::flush();
    Log::println("Finished.");
}
