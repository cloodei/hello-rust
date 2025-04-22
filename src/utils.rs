#[inline]
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
pub fn swap<T: Copy>(a: &mut T, b: &mut T) {
    let t = *a;
    *a = *b;
    *b = t;
}

pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }

    true
}

pub fn is_sorted_strict<T: Ord + Copy>(prev: &[T], latter: &[T]) -> bool {
    let len = prev.len();
    if len != latter.len() {
        return false;
    }
    if len == 0 {
        return true;
    }

    
    let mut tmp = Vec::with_capacity(len);
    unsafe {
        tmp.set_len(len);
    }
    tmp.copy_from_slice(prev);
    tmp.sort_unstable();

    for i in 0..len {
        if tmp[i] != latter[i] {
            return false;
        }
    }

    true
}
