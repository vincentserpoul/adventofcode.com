type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> ResultMain<()> {
    let seats: Vec<Seat> = include_str!("../../inputs/5.txt")
        .split('\n')
        .filter_map(|code| Seat::from_str(code).ok())
        .collect();

    // BFFFBBFRRR: row 70, column 7, seat ID 567.
    // FFFBBBFRRR: row 14, column 7, seat ID 119.
    // BBFFBBFRLL: row 102, column 4, seat ID 820.

    println!(
        "highest seat: {:#?}",
        seats
            .iter()
            .map(|s| s.get_seat_id().unwrap_or(0))
            .max()
            .unwrap()
    );

    let mut ordered_seats = seats
        .iter()
        .map(|s| s.get_seat_id().unwrap_or(0))
        .collect::<Vec<usize>>();
    ordered_seats.sort_unstable();

    let mut prev_seat = 0;
    for s in ordered_seats {
        if s > 2 && prev_seat == s - 2 {
            println!("my seat: {:?}", prev_seat + 1);
        }
        prev_seat = s;
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum RowSplit {
    F,
    B,
}

use std::str::FromStr;

impl FromStr for RowSplit {
    type Err = ();

    fn from_str(row: &str) -> Result<Self, Self::Err> {
        match row {
            "F" => Ok(RowSplit::F),
            "B" => Ok(RowSplit::B),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ColSplit {
    R,
    L,
}

impl FromStr for ColSplit {
    type Err = ();

    fn from_str(col: &str) -> Result<Self, Self::Err> {
        match col {
            "R" => Ok(ColSplit::R),
            "L" => Ok(ColSplit::L),
            _ => Err(()),
        }
    }
}

#[derive(Debug, std::cmp::PartialEq)]
struct Seat {
    row_splits: Vec<RowSplit>,
    col_splits: Vec<ColSplit>,
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let row_splits: Vec<RowSplit> = code
            .chars()
            .filter_map(|row| RowSplit::from_str(&String::from(row)).ok())
            .collect();

        let col_splits: Vec<ColSplit> = code
            .chars()
            .filter_map(|col| ColSplit::from_str(&String::from(col)).ok())
            .collect();

        Ok(Seat {
            row_splits,
            col_splits,
        })
    }
}

impl Seat {
    fn get_row(&self) -> Result<usize, ()> {
        let mut rows: Vec<usize> = (0..128).collect();

        for r in &self.row_splits {
            rows = match r {
                RowSplit::F => rows[0..rows.len() / 2].to_vec(),
                RowSplit::B => rows[rows.len() / 2..rows.len()].to_vec(),
            };
        }

        if rows.len() != 1 {
            return Err(());
        }

        Ok(rows[0])
    }

    fn get_col(&self) -> Result<usize, ()> {
        let mut cols: Vec<usize> = (0..8).collect();

        for c in &self.col_splits {
            cols = match c {
                ColSplit::L => cols[0..cols.len() / 2].to_vec(),
                ColSplit::R => cols[cols.len() / 2..cols.len()].to_vec(),
            };
        }

        if cols.len() != 1 {
            return Err(());
        }

        Ok(cols[0])
    }

    fn get_seat_id(&self) -> Result<usize, ()> {
        let row = &self.get_row()?;
        let col = &self.get_col()?;

        Ok(row * 8 + col)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_seat_from_str() {
        assert_eq!(
            Seat::from_str("BFFFBBFRRR").unwrap(),
            Seat {
                row_splits: vec![
                    RowSplit::B,
                    RowSplit::F,
                    RowSplit::F,
                    RowSplit::F,
                    RowSplit::B,
                    RowSplit::B,
                    RowSplit::F
                ],
                col_splits: vec![ColSplit::R, ColSplit::R, ColSplit::R],
            }
        );
    }

    #[test]
    fn test_seat_get_row() {
        assert_eq!(Seat::from_str("BFFFBBFRRR").unwrap().get_row().unwrap(), 70);
        assert_eq!(Seat::from_str("FFFBBBFRRR").unwrap().get_row().unwrap(), 14);
        assert_eq!(
            Seat::from_str("BBFFBBFRLL").unwrap().get_row().unwrap(),
            102
        );
    }

    #[test]
    fn test_seat_get_col() {
        assert_eq!(Seat::from_str("BFFFBBFRRR").unwrap().get_col().unwrap(), 7);
        assert_eq!(Seat::from_str("FFFBBBFRRR").unwrap().get_col().unwrap(), 7);
        assert_eq!(Seat::from_str("BBFFBBFRLL").unwrap().get_col().unwrap(), 4);
    }

    #[test]
    fn test_seat_get_seat_id() {
        assert_eq!(
            Seat::from_str("BFFFBBFRRR").unwrap().get_seat_id().unwrap(),
            567
        );
        assert_eq!(
            Seat::from_str("FFFBBBFRRR").unwrap().get_seat_id().unwrap(),
            119
        );
        assert_eq!(
            Seat::from_str("BBFFBBFRLL").unwrap().get_seat_id().unwrap(),
            820
        );
    }
}
