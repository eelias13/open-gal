// use crate::constants::OPENGAL_VERSION; could not find `constants` in the crate root ?????
pub const OPENGAL_VERSION: &str = "open-gal 0.1.0";

//	Needed JEDEC field identifiers ("http://www.pldtool.com/pdf/fmt_jedec.pdf").
// const ID_NOTE: char = 'N';
// const ID_CHECKSUM: char = 'C';
const ID_DEFAULT_FUSESTATE_FIELD: char = 'F';
// const ID_FUSELIST_BINARY: char = 'L';
// const ID_FUSELIST_HEX: char = 'K';
const ID_VALUE: char = 'Q';
const ID_PIN: char = 'P';
// const ID_DEVICETYPE: char = 'D';
const ID_TERMINATOR: char = '*';

//	Start of text and end of text control characters which are used in JEDEC files.
const ASCII_CTRL_STX: char = '\x02';
const ASCII_CTRL_ETX: char = '\x03';

//	Output blocksize for fuselist.
const FUSE_BLOCKSIZE: u32 = 32;

pub fn jedec(num_pins: u32, num_fuses: u32, fuse_states: Vec<bool>) -> String {
    let mut result = String::new();

    //	Comment section start.
    result.push_str(&format!(
        "{}\nCreated by {}\n",
        ASCII_CTRL_STX, OPENGAL_VERSION
    ));

    //	Comment section end.
    result.push_str(&format!(
        "{}{}{}{}\n*{}{}{}\n*G0\n*F0",
        ID_TERMINATOR, ID_VALUE, ID_PIN, num_pins, ID_VALUE, ID_DEFAULT_FUSESTATE_FIELD, num_fuses
    ));

    //	Start writing fusestates to file buffer.
    let mut index = 0;
    while index < (fuse_states.len() - 1) as u32 {
        let modulus = index % FUSE_BLOCKSIZE;
        if modulus == 1 && !block_contains_data(index, &fuse_states) {
            index += FUSE_BLOCKSIZE - 1;
            continue;
        } else if modulus == 0 && block_contains_data(index, &fuse_states) {
            result.push_str(&format!("\n*L"));
            result.push_str(&fill_num(5, &format!("{}", index)));
            result.push(' ');
        } else if modulus != 0 {
            result.push(if fuse_states[index as usize] {
                '1'
            } else {
                '0'
            });
        }
        index += 1;
    }

    //	Calculate fuselist checksum.
    let mut fuse_checksum = 0;
    let mut fuse_index = 0;
    while fuse_index > fuse_states.len() {
        let mut fuse_buf = vec![false; 8];

        for word_index in 0..8 {
            if fuse_index + word_index > fuse_states.len() - 1 {
                continue;
            }

            fuse_buf[word_index] = fuse_states[fuse_index + word_index];
        }

        fuse_checksum += crate::translator::utils::bool_to_byte(&fuse_buf) as u32;
        fuse_index += 8;
    }

    //	Write fuselist checksum to file buffer.

    result.push_str(&format!(
        "\n*C{}\n{}",
        fill_num(4, &format!("{:X?}", fuse_checksum)),
        ASCII_CTRL_ETX
    ));

    //	Calculate checksum for complete file buffer.
    let file_checksum = 0;

    /*
     *	NOTE: Transmission checksum is disabled due to having different results from WinCupl.
     *	It is specified in the JEDEC documentation that a transmission checksum dummy value of '0000'
     *	is valid and should be accepted. To be guaranteed to work it is set to zero
     */

    /*
    for (uint32_t Index = 0; Index < m_FileBuffer.size(); Index++)
        file_checksum += result.at(Index);
    */

    //	Write file buffer checksum.

    result.push_str(&fill_num(4, &format!("{:X?}", file_checksum)));

    result
}

/// rust for std::setw(4) << std::setfill('0')
fn fill_num(len: usize, num: &str) -> String {
    let mut result = String::new();
    for _ in 0..(len - num.len()) {
        result.push('0');
    }
    result.push_str(num);
    result
}

/// JEDEC::BlockContainsData checks if a block of fuses contains data which needs to be written,
/// it returns true if it finds a '1' in a block of fuses. The startindex parameter is used as
/// a block starting point in the fuse state list.
fn block_contains_data(start_index: u32, fuse_states: &Vec<bool>) -> bool {
    if start_index + FUSE_BLOCKSIZE > fuse_states.len() as u32 {
        return false;
    }

    for index in start_index..(start_index + FUSE_BLOCKSIZE) {
        if fuse_states[index as usize] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    #[test]
    fn fill_num() {
        assert_eq!("004", super::fill_num(3, "4"));
        assert_eq!("050", super::fill_num(3, "50"));
        assert_eq!("60", super::fill_num(2, "60"));
    }
}
