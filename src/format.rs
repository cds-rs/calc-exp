use std::fmt;

pub fn format_binary<T: fmt::UpperHex>(val: T, bits: usize) -> String {
    let hex = format!("{:0width$X}", val, width = bits / 4);

    let bin: String = hex.chars()
        .map(|c| match c {
            '0' => " 0000", '1' => " 0001", '2' => " 0010", '3' => " 0011",
            '4' => " 0100", '5' => " 0101", '6' => " 0110", '7' => " 0111",
            '8' => " 1000", '9' => " 1001", 'A' => " 1010", 'B' => " 1011",
            'C' => " 1100", 'D' => " 1101", 'E' => " 1110", 'F' => " 1111",
            _ => unreachable!()
        })
        .collect();

    format!("0b{}", bin)
}
