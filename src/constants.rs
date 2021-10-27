// when parsing pin the number comes first
// e.g. if NUM_FIRST == true `pin 1 = a;` else `pin a = 1;`
pub const NUM_FIRST: bool = true;
pub const COUNT_VERTICAL: bool = false;

// symbols for logical operators
pub const AND: char = '&';
pub const OR: char = '|';
pub const XOR: char = '?';
pub const NOT: char = '!';

//	Version string for JEDEC file
// pub const OPENGAL_VERSION: &str = "open-gal 0.1.0"; is uesd in translator/jedec.rs
