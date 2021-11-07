#!/usr/bin/env run-cargo-script

fn main() {
    assert_eq!(pairwise(vec![1, 4, 2, 3, 0, 5], 7), 11);
    assert_eq!(pairwise(vec![1, 3, 2, 4], 4), 1);
    assert_eq!(pairwise(vec![1, 1, 1], 2), 1);
    assert_eq!(pairwise(vec![0, 0, 0, 0, 1, 1], 1), 10);
    assert_eq!(pairwise(vec![0; 0], 100), 0);
}

fn pairwise(x: Vec<i32>, n: i32) -> i32 {
    let xn = x.len();
    let mut finds = vec![0 as usize; 0];

    for i in 0..xn {
        for j in 0..xn {
            if (i != j) && (x[i] + x[j] == n) && !&finds.iter().any(|f| f.eq(&i) || f == &j) {
                finds.push(i);
                finds.push(j);
            }
        }
    }

    finds.iter().map(|&i| i as i32).sum()
}
