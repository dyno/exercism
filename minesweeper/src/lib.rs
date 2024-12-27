
use std::cmp::min;

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
        for i in x.saturating_sub(1)..=min(x + 1, m - 1) {
            for j in y.saturating_sub(1)..=min(y + 1, n - 1) {
                if minefield[i].as_bytes()[j] == b'*' {
                    count += 1;
                }
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