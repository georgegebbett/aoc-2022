pub fn sum_reducer(p: i32, c: i32) -> i32 {
    return p + c;
}

pub fn sum_vec(a: Vec<i32>) -> i32 {
    return a.into_iter().reduce(sum_reducer).unwrap();
}