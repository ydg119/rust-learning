pub fn test() {
    let mut s1 = String::from("aaa");
    s1.push_str("bbb");
    println!("{s1}");

    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = format!("{s1}-{s2}");
    println!("{s3}");

    let s1 = String::from("abcdefghijklmnopqrstuvwxyz");
    let s2 = &s1[0..5];
    println!("{s2}");

    for c in "12ab中国".chars() {
        println!("{c}")
    }

    for b in "12ab中国".bytes() {
        println!("{b}")
    }
    
}
