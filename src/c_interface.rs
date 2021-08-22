use crate::parser;
use crate::utils;
use std::mem;
extern crate libc;

#[no_mangle]
pub unsafe extern "C" fn parse_file(input: *mut std::os::raw::c_char) -> TransferTableDataArr {
    let data = utils::read_file(char_ptr_2_str(input).as_str());
    let table_data = parser::parse(data);
    convert_vec_table_data(table_data)
}

// ---------------------------------------------------------------------- TableData ----------------------------------------------------------------------
/*
*   This data structure contains following data from processed expressions.
*
*   - "m_InputPins" stores all the input pins which are used in the expression
*   - "m_OutputPin" stores the output pin
*   - "m_Table" contains the truth table for the expression and is used to generate a dnf expression later on
*   - "m_EnableDFlipFlop" holds a boolean which decides if the output pin should have its flip flop turned on.
*/

#[derive(PartialEq, Debug, Clone)]
pub struct TableData {
    pub input_pins: Vec<u32>,
    pub output_pin: u32,
    pub table: Vec<bool>,
    pub enable_flip_flop: bool,
}

#[derive(PartialEq, Debug, Clone)]
#[repr(C)]
pub struct TransferU32Vec {
    arr: *mut u32,
    len: usize,
}

#[derive(PartialEq, Debug, Clone)]
#[repr(C)]
pub struct TransferBoolVec {
    arr: *mut bool,
    len: usize,
}

#[derive(PartialEq, Debug, Clone)]
#[repr(C)]
pub struct TransferTableData {
    input_pins: TransferU32Vec,
    output_pin: u32,
    table: TransferBoolVec,
    enable_flip_flop: bool,
}

#[derive(PartialEq, Debug, Clone)]
#[repr(C)]
pub struct TransferTableDataArr {
    arr: *mut TransferTableData,
    len: usize,
}

unsafe fn convert_vec_u32(vec: Vec<u32>) -> TransferU32Vec {
    let arr: *mut u32 =
        libc::malloc((mem::size_of::<u32>() as libc::size_t) * vec.len()) as *mut u32;

    for (i, v) in vec.iter().enumerate() {
        *(arr.add(i)) = v.clone();
    }

    TransferU32Vec {
        arr,
        len: vec.len(),
    }
}

unsafe fn convert_vec_bool(vec: Vec<bool>) -> TransferBoolVec {
    let arr: *mut bool = libc::malloc((mem::size_of::<bool>()) * vec.len()) as *mut bool;
    for (i, v) in vec.iter().enumerate() {
        *(arr.add(i)) = v.clone();
    }
    TransferBoolVec {
        arr,
        len: vec.len(),
    }
}

unsafe fn convert_table_data(td: TableData) -> TransferTableData {
    TransferTableData {
        input_pins: convert_vec_u32(td.input_pins),
        output_pin: td.output_pin,
        table: convert_vec_bool(td.table),
        enable_flip_flop: td.enable_flip_flop,
    }
}

unsafe fn convert_vec_table_data(vec: Vec<TableData>) -> TransferTableDataArr {
    let arr: *mut TransferTableData =
        libc::malloc((mem::size_of::<TransferTableData>()) * vec.len()) as *mut TransferTableData;
    for (i, v) in vec.iter().enumerate() {
        *(arr.add(i)) = convert_table_data(v.clone());
    }
    TransferTableDataArr {
        arr,
        len: vec.len(),
    }
}

unsafe fn char_ptr_2_str(input: *mut std::os::raw::c_char) -> String {
    let mut output = String::new();
    let mut i = 0;
    while *(input.add(i)) != 0 {
        output.push(*(input.add(i)) as u8 as char);
        i += 1;
    }
    output
}
