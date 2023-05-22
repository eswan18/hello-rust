use std::collections::HashMap;

pub fn find_median(v: &Vec<i32>) -> Result<i32, String> {
    if v.len() == 0 {
        return Err(String::from("Vector must have non-zero length"));
    }
    // sort the vector
    let mut cloned = v.clone();
    cloned.sort();
    if cloned.len() % 2 != 0 {
        return Err(String::from("Vector length is not even"));
    }
    let halfway = cloned.len() / 2;
    let median = cloned[halfway];
    Ok(median)
}

pub fn find_mode(v: &Vec<i32>) -> Result<i32, String> {
    if v.len() == 0 {
        return Err(String::from("Vector must have non-zero length"));
    }
    let mut map = HashMap::new();
    for i in v {
        map.entry(*i).or_insert(0);
        map.entry(*i).and_modify(|v| {*v += 1} );
    }
    // Figure out which value occurs most.
    let mut max_v = -1;
    let mut winner: Option<i32> = None;
    for (k, v) in map.iter() {
        if v >= &max_v {
            max_v = *v;
            winner = Some(*k);
        }
    }
    match winner {
        Some(k) => return Ok(k),
        None => return Err(String::from("couldn't find a mode"))
    };
}