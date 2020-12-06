use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ROW_LOWER_CHAR: char = 'F';
const ROW_UPPER_CHAR: char = 'B';
const COL_LOWER_CHAR: char = 'L';
const COL_UPPER_CHAR: char = 'R';

/// Find number of passports, separated by blank lines, with all required fields
fn main() {
    let mut max_seat_id: i32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(record) = line {
                let (mut row_lo, mut row_hi) = (0, 127);
                let (mut col_lo, mut col_hi) = (0, 7);
                for (i, c) in record.chars().enumerate() {
                    if i < 7 {
                        let row_half_diff = (row_hi - row_lo) / 2;
                        match c {
                            ROW_LOWER_CHAR => row_hi = row_lo + row_half_diff,
                            ROW_UPPER_CHAR => row_lo = row_hi - row_half_diff,
                            _ => panic!("invalid row code: {}", c)
                        }
                    } else {
                        let col_half_diff = (col_hi - col_lo) / 2;
                        match c {
                            COL_LOWER_CHAR => col_hi = col_lo + col_half_diff,
                            COL_UPPER_CHAR => col_lo = col_hi - col_half_diff,
                            _ => panic!("invalid col code: {}", c)
                        }
                    }
                }
                assert_eq!(row_lo, row_hi, "rows not equal: {}, {}", row_lo, row_hi);
                assert_eq!(col_lo, col_hi, "cols not equal: {}, {}", col_lo, col_hi);
                let seat_id = row_lo * 8 + col_lo;
                if seat_id > max_seat_id {
                    max_seat_id = seat_id;
                }
            }
        }
    }
    println!("max_seat: {}", max_seat_id);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
