pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let (mut lo, mut hi) = (0, array.len() - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if array[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    // the algorithm is implemented as bisect_left. lo is the index where the
    // key will be inserted at the first position where key <= array[i]
    if array[lo] == key {
        Some(lo)
    } else {
        None
    }
}

// https://github.com/python/cpython/blob/main/Lib/bisect.py
