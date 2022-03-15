mod circuit_config;
mod parser;
mod table_data;
mod translator;
mod transpiler;

pub use circuit_config::CircuitConfig;
pub use table_data::TableData;
pub use translator::core::to_jedec;

pub use parser::OGal;
pub use transpiler::wincupl::to_wincupl;

pub fn parse(code: &str) -> Result<Vec<TableData>, String> {
    match parser::parse(code) {
        Err(error) => Err(format!("{:?}", error)),
        Ok(td_vec) => Ok(td_vec),
    }
}

// when parsing pin the number comes first
// e.g. if NUM_FIRST == true `pin 1 = a;` else `pin a = 1;`
pub const NUM_FIRST: bool = true;
pub const COUNT_VERTICAL: bool = false;

// symbols for logical operators
// pub const AND: &str = "&";
// pub const OR: &str = "|";
// pub const XOR: &str = "?";
// pub const NOT: &str = "!";

//	Version string for JEDEC file
// pub const OPENGAL_VERSION: &str = "open-gal 0.1.0"; is uesd in translator/jedec.rs
