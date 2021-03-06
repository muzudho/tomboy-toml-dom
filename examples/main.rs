//! Test.
//! テスト。
//!
//! `cargo run --example main`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::{
    model::{layer225::Val, layer230::Expression},
    Toml,
};

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-main");
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/edit-1.type.toml";
    let doc = Toml::from_file(toml_file);
    Log::info_toml_document(toml_file, &doc);

    for elem in doc.elements {
        match elem {
            Expression::HeaderOfArrayOfTable(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("HeaderOfArrayOfTable", &format!("{}", m)),
                );
            }
            Expression::EmptyLine(ws, comment) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default()
                        .str("EmptyLine", "")
                        .str("ws", &ws.to_string())
                        .str(
                            "comment",
                            &if let Some(comment) = comment.as_ref() {
                                comment.to_string()
                            } else {
                                "".to_string()
                            },
                        ),
                );
            }
            Expression::Keyval(_ws1, keyval, _ws2, _comment) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("Keyval", &format!("{}", keyval)),
                );
                match *keyval.val {
                    Val::Array(m) => Log::info(&format!("{}", m)),
                    Val::BasicString(m) => Log::info(&format!("{}", m)),
                    Val::InlineTable(m) => Log::info(&format!("{}", m)),
                    Val::LiteralValue(m) => Log::info(&format!("{}", m)),
                    Val::LiteralString(m) => Log::info(&format!("{}", m)),
                }
            }
            Expression::HeaderOfTable(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("Table", &format!("{}", m)),
                );
            }
        }
    }

    Log::flush();
}
