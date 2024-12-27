
const NEIGHBOURHOOD_OFFSETS: &[(i32, i32)] = &[
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

        let mut count = 0;
        for &(ox, oy) in NEIGHBOURHOOD_OFFSETS {
            let i = x as i32 + ox;
            let j = y as i32 + oy;
            if i < 0 || i >= m as i32 || j < 0 || j >= n as i32 {
                continue;
            }
            if minefield[i as usize].as_bytes()[j as usize] == b'*' {
                count += 1;
            }
        }
        if count == 0 {
            ' '
        } else {
            (b'0' + count) as char
        }
    };

    let mut ret = Vec::with_capacity(m);
    for i in 0..m {
        let mut s = String::with_capacity(n);
        for j in 0..n {
            s.push(count_mines(i, j));
        }
        ret.push(s);
    }
    ret
}
