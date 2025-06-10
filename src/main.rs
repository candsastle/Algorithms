fn merge_sort(arr: &[i32]) -> Vec<i32> {
    let arr_len = arr.len();
    if arr_len < 2 {
        return arr.to_vec();
    }
    if arr_len == 2 {
        if arr[0] > arr[1] {
            return vec![arr[1], arr[0]];
        } else {
            return arr.to_vec();
        };
    }
    let mid = arr_len / 2;
    let lhs = merge_sort(&arr[0..mid]);
    let rhs = merge_sort(&arr[mid..arr_len]);
    let mut result_arr = vec![0; arr_len];
    let mut i = 0;
    let mut j = mid;

    for k in 0..arr_len {
        if lhs[i] > rhs[j] {
            result_arr[k] = rhs[j];
            j += 1;
        } else {
            result_arr[k] = lhs[i];
            i += 1;
        }
    }
    return result_arr;
}

fn main() {
    let arr = vec![38, 27, 43, 3, 9, 82, 10];
    let result = merge_sort(&arr);
    dbg!(result);
}

// [3 38 43 |  X 9 10 82] 27
