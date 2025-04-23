use crate::utils::min;

const RUN: usize = 24;


/// Sort the entire `arr` in-place with InsertionSort, using a custom comparator.
/// 
/// O(n^2) average and worst case time complexity, O(1) space complexity.
///
/// For example, to sort in ascending order, use `|a, b| a < b`.\
/// To sort in descending order, use `|a, b| a > b`.\
/// To sort in a custom order, use `|a, b| a.custom_cmp(b)`. (if custom_cmp returns true, a is put before b)
pub fn insertion_sort_spec<T: Copy>(arr: &mut [T], start: usize, end: usize, cmp: fn(&T, &T) -> bool) {
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

        while j > left && arr[j - 1] > k {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = k;
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
        i += 1;
        j -= 1;
        while cmp(&arr[i], &pivot) {
            i += 1;
        }
        while cmp(&pivot, &arr[j]) {
            j -= 1;
        }
        if i >= j {
            break;
        }
        arr.swap(i, j);
    }

    arr.swap(i, right);
    internal(arr, left, i - 1, cmp);
    internal(arr, i + 1, right, cmp);
}

/// Sort the entire `arr` in-place, in ascending order, with Hoare's partitioning QuickSort.
/// 
/// O(n log(n)) average time complexity, O(n^2) worst case, O(log(n)) space complexity.
pub fn quick_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    internal(arr, 0, n - 1, |a, b| a < b);
}

/// Sort the entire `arr` in-place with Hoare's partitioning QuickSort, using a custom comparator.
///
/// O(n log(n)) average time complexity, O(n^2) worst case, O(log(n)) space complexity.
/// 
/// For example, to sort in ascending order, use `|a, b| a < b`.\
/// To sort in descending order, use `|a, b| a > b`.\
/// To sort in a custom order, use `|a, b| a.custom_cmp(b)`. (if custom_cmp returns true, a is put before b)
pub fn quick_sort_spec<T: Copy>(arr: &mut [T], cmp: fn(&T, &T) -> bool) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    internal(arr, 0, n - 1, cmp);
}


/// Sort the entire `arr` in-place, in ascending order, with TimSort.
///
/// O(n log(n)) average time complexity, O(n) best case, O(n) space complexity.
pub fn merge_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n <= RUN + 8 {
        insertion_sort(arr, 0, n - 1);
        return;
    }

    let mut i = 0;
    while i < n {
        insertion_sort(arr, i, min(i + RUN - 1, n - 1));
        i += RUN;
    }

    let mut in_buffer = false;
    let mut buffer = Vec::with_capacity(n);
    unsafe {
        buffer.set_len(n);
    };
    
    let mut src = arr;
    let mut dst = buffer.as_mut_slice();

    let mut width = RUN;
    while width < n {
        i = 0;
        while i < n {
            let mid = min(i + width, n);
            let right = min(mid + width, n);

            let mut l = i;
            let mut r = mid;
            let mut curr = i;

            while l < mid && r < right {
                if src[l] < src[r] {
                    dst[curr] = src[l];
                    l += 1;
                }
                else {
                    dst[curr] = src[r];
                    r += 1;
                }
                curr += 1;
            }
            while l < mid {
                dst[curr] = src[l];
                curr += 1;
                l += 1;
            }
            while r < right {
                dst[curr] = src[r];
                curr += 1;
                r += 1;
            }

            i += width << 1;
        }

        let tmp = src;
            src = dst;
            dst = tmp;

        in_buffer = !in_buffer;
        width <<= 1;
    }

    if in_buffer {
        dst.copy_from_slice(src);
    }
}

pub fn merge_sort_spec<T: Copy>(arr: &mut [T], cmp: fn(&T, &T) -> bool) {
    let n = arr.len();
    if n <= RUN + 8 {
        insertion_sort_spec(arr, 0, n - 1, cmp);
        return;
    }

    for i in (0..n).step_by(RUN) {
        insertion_sort_spec(arr, i, min(i + RUN - 1, n - 1), cmp);
    }

    let mut in_buffer = false;
    let mut buffer = Vec::with_capacity(n);
    unsafe {
        buffer.set_len(n);
    };
    
    let mut src = arr;
    let mut dst = buffer.as_mut_slice();

    let mut width = RUN;
    while width < n {
        let mut i = 0;
        while i < n {
            let mid = min(i + width, n);
            let right = min(mid + width, n);

            let mut l = i;
            let mut r = mid;
            let mut curr = i;

            while l < mid && r < right {
                if cmp(&src[l], &src[r]) {
                    dst[curr] = src[l];
                    l += 1;
                }
                else {
                    dst[curr] = src[r];
                    r += 1;
                }
                curr += 1;
            }
            while l < mid {
                dst[curr] = src[l];
                curr += 1;
                l += 1;
            }
            while r < right {
                dst[curr] = src[r];
                curr += 1;
                r += 1;
            }

            i += width << 1;
        }

        let tmp = src;
            src = dst;
            dst = tmp;

        in_buffer = !in_buffer;
        width <<= 1;
    }

    if in_buffer {
        dst.copy_from_slice(src);
    }
}


fn heapify<T: Copy + PartialOrd>(arr: &mut [T], n: usize, i: usize, cmp: fn(&T, &T) -> bool) {
    let hole = arr[i];
    let mut parent = i;
    let mut child = 2 * i + 1;

    while child < n {
        if child + 1 < n && cmp(&arr[child], &arr[child + 1]) {
            child += 1;
        }
        if !cmp(&hole, &arr[child]) {
            break;
        }

        arr[parent] = arr[child];
        parent = child;
        child = child + child + 1;
    }
    arr[parent] = hole;
}

/// Sort the entire `arr` in-place, in ascending order, with HeapSort.
///
/// O(n log(n)) every case, O(1) space complexity.
pub fn heap_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    for i in (0..n / 2).rev() {
        heapify(arr, n, i, |a, b| a < b);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0, |a, b| a < b);
    }
}
