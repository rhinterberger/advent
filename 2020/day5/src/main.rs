use std::fs;

fn main() {
    let max_id = read_input("input.txt")
        .lines()
        .fold(0,|current_max, seat|
            current_max.max(get_seat_id(seat))
        );

    println!("{}", max_id);

    let mut seats:Vec<i32> = read_input("input.txt")
        .lines()
        .map(get_seat_id)
        .collect();

    println!("{}",find_seat(&mut seats).unwrap());
}

fn read_input(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
}

fn get_seat_id(seat: &str) -> i32 {
    let seat_position = get_seat(seat);
    seat_position.0 * 8 + seat_position.1
}

fn find_seat(seats: &mut Vec<i32>) -> Option<i32> {
    seats.sort();

    for myseat in seats[0]..seats[seats.len() - 1] {
        if seats[myseat as usize - seats[0] as usize] != myseat {
            return Some(myseat);
        }
    }
    None
}

fn get_seat(seat: &str) -> (i32,i32) {
    let mut col = (7,0);
    let mut row =(127,0);

    seat.chars().for_each(|chr| {
       match chr {
           'B' => { row = get_upper(row); },
           'F' => { row = get_lower(row);},
           'R' => { col = get_upper(col);},
           'L' => { col = get_lower(col);},
           _ => {}
       }
    });

    (row.0, col.0)
}

fn get_lower(range: (i32,i32)) -> (i32,i32) {
    let mid = (range.0 - range.1) >> 1;
    let top = range.0 - mid - 1;
    (top, range.1)
}

fn get_upper(range: (i32,i32)) -> (i32,i32) {
    let mid = (range.0 - range.1) >> 1;
    let bottom = range.1 + mid + 1;
    (range.0, bottom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_half() {
        let mut range = (127,0);
        assert_eq!(get_lower(range),(63,0));
        range=(63,0);
        assert_eq!(get_lower(range),(31,0));
        range=(127,64);
        assert_eq!(get_lower(range),(95,64));
    }

    #[test]
    fn test_upper_half() {
        let mut range = (127,0);
        assert_eq!(get_upper(range),(127,64));
        range=(127,64);
        assert_eq!(get_upper(range),(127,96));
        range=(64,33);
        assert_eq!(get_upper(range),(64,49));
    }

    #[test]
    fn test_seat_poition() {
        assert_eq!(get_seat("FBFBBFFRLR"),(44,5));
        assert_eq!(get_seat("BFFFBBFRRR"),(70,7));
        assert_eq!(get_seat("FFFBBBFRRR"),(14,7));
        assert_eq!(get_seat("BBFFBBFRLL"),(102,4));
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(get_seat_id("FBFBBFFRLR"),357);
        assert_eq!(get_seat_id("BFFFBBFRRR"),567);
        assert_eq!(get_seat_id("FFFBBBFRRR"),119);
        assert_eq!(get_seat_id("BBFFBBFRLL"),820);
    }

    #[test]
    #[should_panic]
    fn input_not_readable() {
        read_input("non_existing_file.txt");
    }
}