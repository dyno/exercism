const NEIGHBOURHOOD_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    if minefield[0].is_empty() {
        return vec!["".to_string(); minefield.len()];
    }

    let m = minefield.len();
    let n = minefield[0].len();

    let count_mines = |x: usize, y: usize| {
        if minefield[x].as_bytes()[y] == b'*' {
            return '*';
        }

        let count = NEIGHBOURHOOD_OFFSETS
            .iter()
            .filter(|(ox, oy)| {
                let i = x as i32 + ox;
                let j = y as i32 + oy;
                i >= 0 && i < m as i32 && j >= 0 && j < n as i32 
                    && minefield[i as usize].as_bytes()[j as usize] == b'*'
            })
            .count();

        if count == 0 {
            ' '
        } else {
            (b'0' + count as u8) as char
        }
    };

    (0..m)
        .map(|i| {
            (0..n)
                .map(|j| count_mines(i, j))
                .collect::<String>()
        })
        .collect()
}
