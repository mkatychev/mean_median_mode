use std::collections::HashMap;

pub fn mean(list: &[i32]) -> i32 {
    let lister = list.iter();
    let mut total = 0i32;
    for num in lister {
        total += num;
    }
    // imperfect mean
    total / (list.len() as i32)
}

pub fn median(list: &[i32]) -> i32 {
    let mut list = list.to_vec();
    list.sort();
    list[list.len() / 2]
}

pub fn mode(list: &[i32]) -> i32 {
    let list = list.to_vec();
    let mut map = HashMap::new();
    // map loop to get occurence dict
    for num in list {
        let greatest = map.entry(num).or_insert(0);
        *greatest += 1;
    }
    let mut mapper = map.iter();
    let (mut number, mut greatest) = mapper.next().unwrap();
    for (key, occurence) in mapper {
        if occurence > greatest {
            greatest = occurence;
            number = key;
        }
    }
    *number
}
