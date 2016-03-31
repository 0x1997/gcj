use std::cmp::Ordering;

/// O(n)
pub fn heapify_by<T, F>(h: &mut [T], cmp: &mut F)
    where F: FnMut(&T, &T) -> Ordering
{
    let n = h.len();
    if n <= 1 {
        return;
    }
    let mut i = (n - 2) >> 1;
    loop {
        fix_down_by(h, i, cmp);
        if i == 0 {
            break;
        }
        i -= 1;
    }
}

/// O(log(n))
pub fn fix_by<T, F>(h: &mut [T], i: usize, cmp: &mut F)
    where F: FnMut(&T, &T) -> Ordering
{
    fix_down_by(h, i, cmp);
    fix_up_by(h, i, cmp);
}

/// O(log(n))
pub fn fix_up_by<T, F>(h: &mut [T], mut i: usize, cmp: &mut F)
    where F: FnMut(&T, &T) -> Ordering
{
    if i < 1 {
        return;
    }
    loop {
        let j = (i - 1) >> 1;
        if j == 0 || cmp(&h[i], &h[j]) == Ordering::Less {
            break;
        }
        h.swap(i, j);
        i = j;
    }
}

/// O(log(n))
pub fn fix_down_by<T, F>(h: &mut [T], mut i: usize, cmp: &mut F)
    where F: FnMut(&T, &T) -> Ordering
{
    let n = h.len();
    if n <= 1 {
        return;
    }
    while i <= (n - 2) >> 1 {
        let left = (i << 1) + 1;
        let right = left + 1;
        let j = if right < n && cmp(&h[left], &h[right]) == Ordering::Greater {
            right
        } else {
            left
        };
        if cmp(&h[i], &h[j]) == Ordering::Less {
            break;
        }
        h.swap(i, j);
        i = j;
    }
}
