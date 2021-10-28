mod circuit_config;
mod constants;
mod parser;
mod table_data;
mod translator;
mod transpiler;

pub use circuit_config::CircuitConfig;
pub use table_data::TableData;
pub use translator::core::to_jedec;
pub use transpiler::wincupl::to_wincupl;

pub fn parse(data: Vec<&str>) -> Result<Vec<TableData>, String> {
    match parser::core::parse(data) {
        Err(parsing_error) => Err(format!("{}", parsing_error)),
        Ok(td_vec) => Ok(td_vec),
    }
}
