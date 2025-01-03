use crate::translator::*;
use crate::{CircuitConfig, TableData};

use super::dnf::Expression;

pub fn to_jedec(
    truth_tables: &Vec<TableData>,
    config: &CircuitConfig,
    head: Option<String>,
) -> Result<String, String> {
    let mut exprs = Vec::new();
    for truth_table in truth_tables {
        exprs.push(Expression::new(truth_table, config)?);
    }

    let fuses = fuses::build(&exprs, config)?;

    Ok(jedec::jedec(config.num_pins, config.num_fuses, fuses, head))
}
