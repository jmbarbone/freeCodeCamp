#!/usr/bin/env rustc

fn main() {
    let x: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![5, 2, 1, 4]];
    assert_eq!(sym_diff(&x), vec![3, 4, 5]);

    let x: Vec<Vec<i32>> = vec![vec![1, 2, 3, 3], vec![5, 2, 1, 4]];
    assert_eq!(sym_diff(&x), vec![3, 4, 5]);

    let x: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![5, 2, 1, 4, 5]];
    assert_eq!(sym_diff(&x), vec![3, 4, 5]);

    let x: Vec<Vec<i32>> = vec![vec![1, 2, 5], vec![2, 3, 5], vec![3, 4, 5]];
    assert_eq!(sym_diff(&x), vec![1, 4, 5]);

    let x: Vec<Vec<i32>> = vec![vec![1, 1, 2, 5], vec![2, 2, 3, 5], vec![3, 4, 5, 5]];
    assert_eq!(sym_diff(&x), vec![1, 4, 5]);

    let x: Vec<Vec<i32>> = vec![vec![3, 3, 3, 2, 5], vec![2, 1, 5, 7], vec![3, 4, 6, 6], vec![1, 2, 3], vec![5, 3, 9, 8], vec![1]];
    assert_eq!(sym_diff(&x), vec![1, 2, 4, 5, 6, 7, 8, 9]);
}


fn sym_diff(x: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];

    for i in 0..x.len() {
        res = do_sym_diff(res, x[i].clone());
    }

    res
}

fn do_sym_diff(x: Vec<i32>, y: Vec<i32>) -> Vec<i32> {
    let mut a = x
        .iter()
        .filter(|&&i| y.iter().all(|&j| i != j))
        .cloned()
        .collect::<Vec<i32>>();

    let mut b = y
        .iter()
        .filter(|&&j| x.iter().all(|&i| j != i))
        .cloned()
        .collect::<Vec<i32>>();

    a.append(&mut b);
    a.sort();
    a.dedup();
    a
}
