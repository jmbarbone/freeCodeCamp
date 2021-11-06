#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! permute = "*"
//! ```
extern crate permute;


fn main() {
    assert_eq!(perm_alone("aab"), 2);
    assert_eq!(perm_alone("aaa"), 0);
    assert_eq!(perm_alone("aabb"), 8);
    assert_eq!(perm_alone("abcdefa"), 3600);
    assert_eq!(perm_alone("zzzzzzzz"), 0);
    assert_eq!(perm_alone("a"), 1);
    assert_eq!(perm_alone("aaab"), 0);
    assert_eq!(perm_alone("aaabb"), 12);
}

fn perm_alone(x: &str) -> i32 {
    let mut letters: Vec<char> = x.to_string().chars().collect();
    
    // println!("Entry for {:?}", letters);

    if  letters.len() == 1 {
        return 1;
    }

    // doesn't matter if this gets sorted
    letters.sort_unstable();

    let perms = permute::permute(letters);

    let mut counter = 0;
    for i in 0..perms.len() {
        let mut dupes = false;
        let vec = &perms[i];
        for j in 0..(vec.len() - 1) {
            if vec[j] == vec[j + 1] {
                dupes = true;
            }
        }
        if !dupes {
            counter = counter + 1;
        }
    }
    counter
}
