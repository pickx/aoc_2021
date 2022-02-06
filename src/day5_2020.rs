static INPUT: &str = include_str!("input_2020/day5");

fn row_and_col(directions: &str) -> (u32, u32) {
    let (mut lo, mut hi) = (0, 127);
    for c in directions[..7].chars() {
        match c {
            'F' => hi = (lo + hi) / 2,
            'B' => lo = 1 + (lo + hi) / 2,
            other => panic!("illegal char {other}"),
        }
    }
    let row = lo;

    let (mut lo, mut hi) = (0, 7);

    for c in directions[7..].chars() {
        match c {
            'L' => hi = (lo + hi) / 2,
            'R' => lo = 1 + (lo + hi) / 2,
            other => panic!("illegal char {other}"),
        }
    }

    let col = lo;

    (row, col)
}

fn seat_id((row, col): (u32, u32)) -> u32 {
    8 * row + col
}

pub fn main() {
    let mut seat_ids: Vec<_> = INPUT.lines().map(row_and_col).map(seat_id).collect();

    seat_ids.sort_unstable();

    let highest_id = seat_ids.last().copied().unwrap();

    let first_id = seat_ids.first().copied().unwrap();
    let missing_id = seat_ids
        .into_iter()
        .zip(first_id..)
        .find(|(actual, expected)| expected != actual)
        .map(|(_, expected)| expected)
        .unwrap();

    println!("Highest seat ID is {highest_id} and the missing ID is {missing_id}");
}
