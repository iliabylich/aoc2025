use std::collections::HashMap;

fn main() {
    println!("{}", solve(include_str!("input1")))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Start,
    Empty,
    Splitter,
    Tachyon,
}

impl From<u8> for Cell {
    fn from(byte: u8) -> Self {
        match byte {
            b'S' => Cell::Start,
            b'.' => Cell::Empty,
            b'^' => Cell::Splitter,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    nrows: usize,
    #[allow(dead_code)]
    ncols: usize,
    cells: Vec<Vec<Cell>>,
    start: (usize, usize),
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut cells = vec![];
        let mut start = None;

        for (i, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (j, byte) in line.bytes().enumerate() {
                let cell = Cell::from(byte);
                if cell == Cell::Start {
                    assert!(start.is_none(), "two start points");
                    start = Some((i, j));
                }
                row.push(cell);
            }
            cells.push(row);
        }

        let nrows = cells.len();
        let ncols = cells[0].len();

        for row in &cells {
            assert_eq!(row.len(), ncols);
        }

        Self {
            nrows,
            ncols,
            cells,
            start: start.expect("no start point"),
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<Cell> {
        self.cells.get(row)?.get(col).copied()
    }

    fn set(&mut self, row: usize, col: usize, cell: Cell) {
        *self.cells.get_mut(row).unwrap().get_mut(col).unwrap() = cell;
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Start => write!(f, "S")?,
                    Cell::Empty => write!(f, ".")?,
                    Cell::Splitter => write!(f, "^")?,
                    Cell::Tachyon => write!(f, "|")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve(input: &str) -> u64 {
    let mut map = Map::parse(input);
    println!("{map}");

    let mut layer = HashMap::new();
    layer.insert(map.start, 1);

    loop {
        println!("Layer: {layer:?}");
        let mut next_layer = HashMap::new();

        for (row, col, num) in layer.iter().map(|((row, col), num)| (*row, *col, *num)) {
            let Some(bottom) = map.get(row + 1, col) else {
                println!("nothing found below {row} {col}");
                continue;
            };

            match bottom {
                Cell::Start => unreachable!("double start"),
                Cell::Empty | Cell::Tachyon => {
                    map.set(row + 1, col, Cell::Tachyon);
                    *next_layer.entry((row + 1, col)).or_default() += num;
                }
                Cell::Splitter => {
                    map.set(row + 1, col - 1, Cell::Tachyon);
                    map.set(row + 1, col + 1, Cell::Tachyon);
                    *next_layer.entry((row + 1, col - 1)).or_default() += num;
                    *next_layer.entry((row + 1, col + 1)).or_default() += num;
                }
            }
        }

        if next_layer.is_empty() {
            println!("{map}");
            return layer.values().copied().sum();
        }

        layer = next_layer;
    }
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 40)
}
