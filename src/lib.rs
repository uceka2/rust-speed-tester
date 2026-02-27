/// Compute the nth Fibonacci number recursively.
pub fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

/// Sort a vector of integers using an iterative merge sort.
pub fn merge_sort(mut data: Vec<i64>) -> Vec<i64> {
    let len = data.len();
    if len <= 1 {
        return data;
    }

    let mut buf = data.clone();
    let mut width = 1;

    while width < len {
        let mut i = 0;
        while i < len {
            let left = i;
            let mid = (i + width).min(len);
            let right = (i + 2 * width).min(len);

            let mut l = left;
            let mut r = mid;
            let mut k = left;

            while l < mid && r < right {
                if data[l] <= data[r] {
                    buf[k] = data[l];
                    l += 1;
                } else {
                    buf[k] = data[r];
                    r += 1;
                }
                k += 1;
            }

            while l < mid {
                buf[k] = data[l];
                l += 1;
                k += 1;
            }

            while r < right {
                buf[k] = data[r];
                r += 1;
                k += 1;
            }

            i += 2 * width;
        }

        std::mem::swap(&mut data, &mut buf);
        width *= 2;
    }

    data
}
