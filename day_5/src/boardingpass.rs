use num::pow;

fn decode_row(input: &str) -> i32 {
    let mut count = 0;
    for bit in 0..7 {
        match input.as_bytes()[bit] as char {
            'B' => { count += pow(2i32, 6 - bit); }
            'F' => {}
            _ => { panic!("Unrecognized character"); }
        }
    }
    count
}

fn decode_column(input: &str) ->i32{
    let mut count = 0;
    for bit in 0..3 {
        match input.as_bytes()[bit] as char {
            'R' => { count += pow(2i32, 2 - bit); }
            'L' => {}
            _ => { panic!("Unrecognized character"); }
        }
    }
    count
}

pub fn seat_id(input: &str)->i32{
    let row = decode_row(&input[0..7]);
    let col = decode_column(&input[7..10]);

    row*8 + col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_row() {
        assert_eq!(decode_row("FBFBBFF"), 44);
        assert_eq!(decode_row("FFFBBBF"), 14);
        assert_eq!(decode_row("BFFFBBF"), 70);
        assert_eq!(decode_row("BBFFBBF"), 102);
    }

    #[test]
    fn test_decode_column() {
        assert_eq!(decode_column("RLR"), 5);
        assert_eq!(decode_column("RRR"), 7);
        assert_eq!(decode_column("RLL"), 4);
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);

        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}