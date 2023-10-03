

fn insertion_sort(v : &mut Vec<i8>) -> Vec<i8> {
    for i in 0..v.len() - 1 {
        let mut smallest = i;
        for j in (i+1)..v.len() {
            if v[j] < v[smallest] {
                smallest = j
            }
        }
        v.swap(smallest, i);
    }
    v.to_vec()
}

fn bubble_sort(v : &mut Vec<i8>) -> Vec<i8> {
    let mut sorted = true;
    for _ in 1..=v.len() - 1 {
        sorted = true;
        for j in 0..=v.len() - 2 {
            if v[j] > v[j+1] {
                v.swap(j, j+1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
    v.to_vec()
}

fn merge(v: &mut [i8], mid: usize) -> Vec<i8> {
    let left = v[..mid].to_vec();
    let right = v[mid..].to_vec();
    let mut l = 0;
    let mut r = 0;
    for val in &mut *v {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
    v.to_vec()
}

fn merge_sort(v : &mut [i8]) -> Vec<i8> {
    if v.len() > 1 {
        let mid = v.len() / 2;
        merge_sort(&mut v[..mid]);
        merge_sort(&mut v[mid..]);
        merge(v, mid);
    }
    v.to_vec()
}

fn partition(v : &mut [i8], start: usize, end: usize) -> usize {
    let mut i = start;
    let pivot = end;
    for j in start..=end - 1 {
        if v[j] < v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

fn quick_sort_helper(v : &mut [i8], start: usize, end: usize) -> Vec<i8> {
    if start < end {
        let part = partition(v, start, end);
        quick_sort_helper(v, start, part - 1);
        quick_sort_helper(v, part + 1, end);
    }
    v.to_vec()
}

fn quick_sort(v : &mut [i8]) -> Vec<i8> {
    quick_sort_helper(v, 0, v.len() - 1)
}


fn main() {
    
    let mut arr: Vec<i8> = vec![7, 5, 6, 4, 9, 3, 1, 8, 2];
    println!("{:?}", arr);
    //insertion_sort(&mut arr);
    //bubble_sort(&mut arr);
    //merge_sort(&mut arr);
    quick_sort(&mut arr);
    println!("{:?}", arr);
}
