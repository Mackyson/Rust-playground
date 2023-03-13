use std::collections::HashMap;

pub fn exercise_vector() {
    let v1 = vec![1, 2, 2, 3, 3, 8];
    let v2 = vec![1];
    // let v3: Vec<i32> = vec![];

    assert_eq!(calc_sum(&v1), 19);
    assert_eq!(calc_mean(&v1), (19 / 6) as f32);
    assert_eq!(calc_median(&v1), 2.5);
    assert_eq!(calc_mode(&v1), vec![2, 3]);

    assert_eq!(calc_sum(&v2), 1);
    assert_eq!(calc_mean(&v2), 1.0);
    assert_eq!(calc_median(&v2), 1.0);
    assert_eq!(calc_mode(&v2), vec![1]);

    return;
}

fn calc_sum(v: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for elm in v {
        sum += elm;
    }
    sum
}
fn calc_mean(v: &Vec<i32>) -> f32 {
    let sum = calc_sum(&v);
    (sum / v.len() as i32) as f32
}
fn calc_median(v: &Vec<i32>) -> f32 {
    if v.len() <= 0 {
        return 0.0;
    }
    if v.len() % 2 == 0 {
        //when even
        let len = v.len() / 2;
        (v[len - 1] + v[len]) as f32 / 2.0
    } else {
        //when odd
        v[v.len() / 2] as f32
    }
}

fn calc_mode(v: &Vec<i32>) -> Vec<i32> {
    let mut hash = HashMap::new();
    for elm in v {
        let count = hash.entry(*elm).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    for (_, v) in &hash {
        max_count = if max_count < *v { *v } else { max_count }
    }
    let mut result = Vec::<i32>::new();
    for (k, v) in &hash {
        if *v == max_count {
            result.push(*k)
        }
    }
    result.sort();
    return result;
}
