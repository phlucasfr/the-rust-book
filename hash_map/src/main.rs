use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 500);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("the {} score is: {}", team_name, score);

    for (key, value) in &scores  {
        println!("key: {}, value: {}", key, value)
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point
    // println!("{}", field_name); borrow of moved value: `field_name`

    map.insert(String::from("Hated color"), String::from("Blue"));
    map.insert(String::from("Hated color"), String::from("Green"));
}

