use core::fmt::Debug;
mod b_rand;

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for r in (1..v.len()).rev() {
        for j in (v.len() - r..v.len()).rev() {
            if v[j] < v[j - 1] {
                v.swap(j, j - 1);
            }
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // bring them together again
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();

    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if a_val < b_val {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    } else {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },

            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(0, p);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(i, p + 1);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    quick_sort(&mut v[..p]);
    quick_sort(&mut v[p + 1..]);
}

pub fn threaded_quick_sort<T: PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    rayon::join(
        || threaded_quick_sort(a),
        || threaded_quick_sort(&mut b[1..]),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let v = vec![
            16, 0, 1, 5, 8, 2, 3, 20, 12, 10, 7, 9, 6, 4, 11, 17, 19, 18, 15, 14, 13,
        ];
        let v = merge_sort(v);
        assert_eq!(
            v,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
    }

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![
            16, 0, 1, 5, 8, 2, 3, 20, 12, 10, 7, 9, 6, 4, 11, 17, 19, 18, 15, 14, 13,
        ];
        bubble_sort(&mut v);
        assert_eq!(
            v,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![
            16, 0, 1, 5, 8, 2, 3, 20, 12, 10, 7, 9, 6, 4, 11, 17, 19, 18, 15, 14, 13,
        ];
        let p = pivot(&mut v);
        for x in 1..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![
            16, 0, 1, 5, 8, 2, 3, 20, 12, 10, 7, 9, 6, 4, 11, 17, 19, 18, 15, 14, 13,
        ];
        quick_sort(&mut v);
        assert_eq!(
            v,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
    }

    #[test]
    fn test_threaded_quick_sort() {
        let mut v = vec![
            16, 0, 1, 5, 8, 2, 3, 20, 12, 10, 7, 9, 6, 4, 11, 17, 19, 18, 15, 14, 13,
        ];
        threaded_quick_sort(&mut v);
        assert_eq!(
            v,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
    }
}
