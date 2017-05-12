pub fn evenly_split_width(width: usize, cols: usize) -> Vec<usize> {
    if cols == 0 {
        return vec![];
    }

    let mut rv = Vec::with_capacity(cols);
    let mut error = 0.0f32;
    let mut total_size = 0;

    for idx in 0..cols {
        let t = width as f32 / cols as f32;
        let mut w = t as usize;
        error += t.fract();
        while error >= 1.0 {
            w += 1;
            error -= 1.0;
        }
        total_size += w;
        rv.push(w);
    }

    let last_idx = rv.len() - 1;
    rv[last_idx] += width - total_size;
    rv
}

#[test]
fn test_evently_split_width() {
    assert_eq!(evenly_split_width(20, 2), vec![10, 10]);
    assert_eq!(evenly_split_width(19, 2), vec![9, 10]);
    assert_eq!(evenly_split_width(19, 4), vec![4, 5, 5, 5]);
    assert_eq!(evenly_split_width(18, 4), vec![4, 5, 4, 5]);
}
