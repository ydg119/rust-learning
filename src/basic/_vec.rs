pub fn test() {
    let mut v = vec![1, 2, 3, 4, 5];
    let a = &v[3];
    println!("{a}");

    v.push(6);
    println!("{v:?}");
}

pub fn test_for() {
    let v = vec!["aaa", "bbb", "ccc"];
    for i in &v {
        println!("{i}")
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i = *i + 10;
    }
    println!("{v:?}")
}
