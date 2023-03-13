pub fn exercise_string() {
    let s1 = String::from("first");
    let s2 = String::from("apple");

    assert_eq!("irst-fay", pig_latin(&s1));
    assert_eq!("apple-hay", pig_latin(&s2));
}

fn pig_latin(s: &String) -> String {
    let mut ret = String::from(s);
    match ret.chars().nth(0) {
        Some('a' | 'e' | 'i' | 'o' | 'u') => ret.push_str("-hay"),
        Some(c) => {
            ret.push_str(&format!("-{}ay", c).to_string());
            ret = ret[1..].to_string()
        }
        None => (),
    }
    ret
}
