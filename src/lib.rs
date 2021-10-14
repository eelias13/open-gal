mod constants;
mod parser;
mod table_data;

pub use table_data::TableData;
pub fn parse(data: Vec<String>) -> Vec<TableData> {
    parser::core::parse(data).unwrap()
}
