//! For those who are struggling with Rust's cool syntax, our goal is to provide a TOML parser that's as easy as pointing to a menu and eating fast food.  
//! Rustのイケてる構文に難儀している人のために、メニューを指差してファーストフードを食べるぐらい簡単な操作のTOMLパーサーを提供することを目標とします。  

// Publish:
//
// (1) `cargo test`
// (2a) `cargo run --example example`
// (2b) `cargo run --example example-tail-comment`
// (2c) `cargo run --example toml-io-en-a-quick-tour-of-toml-v1-0-0rc3`
// (2d) `cargo run --example toml-io-en-v1-0-0rc3-full-speck`
// (3) Open auto-generated log file. I check it.
// (4) Remove the log file.
// (5) Update `README.md`.
// (6) Version up on Cargo.toml.
// (7) `cargo doc --open`
// (8) Comit to Git-hub.
// (9) `cargo publish --dry-run`
// (10) `cargo publish`

// #[macro_use]
// extern crate lazy_static;
extern crate chrono;
extern crate look_ahead_items;
extern crate num_traits;
extern crate rand;

pub mod model;
mod parser;
mod util;

use crate::model::layer310::TomlDocument;
use crate::parser::phase200::{layer210::PResult, layer310::DocumentP};
use casual_logger::{ArrayOfTable, Log, Table};
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// TOML.  
/// トムル。  
pub struct Toml {}
impl Toml {
    /// Line scan.
    /// 行走査。
    pub fn from_file(path: &str) -> TomlDocument {
        let mut error_tables = Vec::<Table>::new();
        let mut output_document = TomlDocument::default();
        match File::open(path) {
            Ok(file) => {
                let mut document_p = DocumentP::default();
                for (i, line) in BufReader::new(file).lines().enumerate() {
                    let row_number = i + 1;
                    let line = match line {
                        Ok(line) => line,
                        Err(why) => panic!(Log::fatal(&format!("{}", why))),
                    };
                    // Log::trace(&format!("from_file/line=|{}|", line));

                    match document_p.scan_line(&line.chars().collect(), &mut output_document) {
                        PResult::End => {} // Ignored it.
                        PResult::Err(table) => {
                            error_tables.push(
                                Table::default()
                                    .str("via", "lib.rs.65.")
                                    .int(
                                        "row_number",
                                        if let Ok(n) = row_number.try_into() {
                                            n
                                        } else {
                                            -1
                                        },
                                    )
                                    .str("line", &format!("{}", line))
                                    .sub_t("table", &table)
                                    .sub_t("document_p", &document_p.log())
                                    .clone(),
                            );
                        }
                        PResult::Ongoing => {} // Ignored it.
                    }
                }
            }
            Err(why) => panic!("{}", why),
        }

        if !error_tables.is_empty() {
            let mut error_aot = ArrayOfTable::default();
            for err_tbl in error_tables {
                error_aot.table(&err_tbl);
            }
            Log::error_t(
                "List if error exists.",
                Table::default().sub_aot("error_aot", &error_aot),
            );
        }

        output_document
    }
}
