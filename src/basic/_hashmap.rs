use std::collections::HashMap;
pub fn test() {
    let mut scores = HashMap::new();
    scores.insert(String::from("red"), 10);
    scores.insert(String::from("blue"), 20);
    println!("{:?}", scores);

    let team_name = String::from("red");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    scores.insert(String::from("red"), 255);
    println!("{:?}", scores);

    // when key exists, or_insert will not change the value
    scores.entry(String::from("red")).or_insert(50);
    // when key not exists, or_insert will insert a new value
    scores.entry(String::from("yellow")).or_insert(50);
    println!("{:?}", scores);

    let mut map = HashMap::new();
    let field_name = String::from("custom_collor");
    let field_value = String::from("pink");
    map.insert(&field_name, &field_value);
    println!("{:?}", map);
    println!("{field_name}: {field_value}");
}
