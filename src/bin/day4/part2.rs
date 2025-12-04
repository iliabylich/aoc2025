fn main() {
    println!("{}", solve(include_str!("input1")))
}

struct Matrix {
    nrows: usize,
    ncols: usize,
    data: Vec<Vec<u8>>,
}

impl Matrix {
    fn new(input: &str) -> Self {
        let mut data = vec![];

        for line in input.lines() {
            let row = line.as_bytes().to_vec();
            data.push(row)
        }

        let nrows = data.len();
        let ncols = data[0].len();
        assert!(data.iter().all(|row| row.len() == ncols));

        Self { nrows, ncols, data }
    }

    fn at(&self, row: usize, col: usize) -> Option<u8> {
        self.data.get(row)?.get(col).copied()
    }
}

const ADJACENT: &[(isize, isize)] = &[
    (-1, -1), // top left
    (-1, 0),  // top
    (-1, 1),  // top right
    (0, -1),  // left
    (0, 1),   // right
    (1, -1),  // bottom left
    (1, 0),   // bottom
    (1, 1),   // bottom right
];
fn adjacent_cells(row: usize, col: usize, nrows: usize, ncols: usize) -> Vec<(usize, usize)> {
    let row: isize = row.try_into().unwrap();
    let col: isize = col.try_into().unwrap();
    let mut out = vec![];

    for (drow, dcol) in ADJACENT {
        let crow = row + drow;
        if crow < 0 {
            continue;
        }
        let crow = crow as usize;
        if crow >= nrows {
            continue;
        }

        let ccol = col + dcol;
        if ccol < 0 {
            continue;
        }
        let ccol = ccol as usize;
        if ccol >= ncols {
            continue;
        }

        out.push((crow, ccol));
    }

    out
}

fn find_rolls_to_remove(m: &Matrix) -> Vec<(usize, usize)> {
    let mut out = vec![];

    for row in 0..m.nrows {
        for col in 0..m.ncols {
            if m.at(row, col) != Some(b'@') {
                continue;
            }

            let mut count = 0;
            for (arow, acol) in adjacent_cells(row, col, m.nrows, m.ncols) {
                if let Some(b) = m.at(arow, acol)
                    && b == b'@'
                {
                    count += 1;
                }
            }

            if count < 4 {
                out.push((row, col));
            }
        }
    }

    out
}

fn solve(input: &str) -> u64 {
    let mut m = Matrix::new(input);
    let mut out = 0;

    loop {
        let to_remove = find_rolls_to_remove(&m);
        if to_remove.is_empty() {
            break;
        }
        out += to_remove.len() as u64;

        for (row, col) in to_remove {
            m.data[row][col] = b'.';
        }
    }

    out
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 43)
}
