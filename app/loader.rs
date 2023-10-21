mod engine;

use crate::engine::plugin_manager;
use crate::engine::utility::json_parser;
use crate::engine::utility::unique_id_generator::id;

fn main() {
    plugin_manager::manager();
    json_parser::parse_date();
    id();
    println!("Hello, world!");
}