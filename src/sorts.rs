const RUN: usize = 24;

pub fn insertion_sort_spec<T: Copy>(arr: &mut [T], start: usize, end: usize, cmp: impl Fn(&T, &T) -> bool) {
    for i in (start + 1)..=end {
        let key = arr[i];
        let mut j = i;

        while j > start && !cmp(&arr[j - 1], &key) {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
    }
}

pub fn insertion_sort<T: Copy + PartialOrd>(arr: &mut [T], left: usize, right: usize) {
    for i in (left + 1)..=right {
        let k = arr[i];
        let mut j = i;

        while j > left && arr[j - 1] < k {
            j -= 1;
        }
    }
}


fn internal<T: Copy>(arr: &mut [T], left: usize, right: usize, cmp: fn(&T, &T) -> bool) {
    if right - left < RUN {
        insertion_sort_spec(arr, left, right, cmp);
        return;
    }

    let mid = (left + right) >> 1;
    if cmp(&arr[right], &arr[left]) {
        arr.swap(right, left);
    }
    if cmp(&arr[mid], &arr[left]) {
        arr.swap(mid, left);
    }
    if cmp(&arr[mid], &arr[right]) {
        arr.swap(right, mid);
    }

    let pivot = arr[right];
    let mut i = left;
    let mut j = right;

    loop {
        i -= 1;
        j -= 1;
        while cmp(&arr[i], &pivot) { i += 1; }
        while cmp(&pivot, &arr[j]) { j -= 1; }
        if i >= j { break; }
        arr.swap(i, j);
    }

    arr.swap(i, right);
    internal(arr, left, i - 1, cmp);
    internal(arr, i + 1, right, cmp);
}

pub fn quick_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    internal(arr, 0, n - 1, |a, b| a < b);
}

pub fn quick_sort_spec<T: Copy>(arr: &mut [T], cmp: fn(&T, &T) -> bool) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    internal(arr, 0, n - 1, cmp);
}


fn heapify<T: Copy + PartialOrd>(arr: &mut [T], n: usize, i: usize) {
    let hole = arr[i];
    let mut parent = i;
    let mut child = 2 * i + 1;

    while child < n {
        if child + 1 < n && arr[child] < arr[child + 1] {
            child += 1;
        }
        if hole >= arr[child] {
            break;
        }
        arr[parent] = arr[child];
        parent = child;
        child = child + child + 1;
    }
    arr[parent] = hole;
}

pub fn heap_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}
