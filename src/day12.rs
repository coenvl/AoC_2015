use serde_json::{from_str, Value};

fn count(data: &Value, ignore_red: bool) -> i64 {
    if data.is_array() {
        data.as_array().unwrap().iter().map(|x| count(x, ignore_red)).sum()
    } else if data.is_object() {
        let map = data.as_object().unwrap();
        if ignore_red && map.values().any(|x| x == "red") {
            0
        } else {
            map.values().map(|x| count(x, ignore_red)).sum()
        }
    } else if data.is_number() {
        data.as_i64().unwrap()
    } else if data.is_string() {
        0
    } else {
        panic!("not implemented: {:?}", data);
    }
}

pub fn day12(input: &str) -> (String, String) {
    let data: Value = from_str(input).unwrap();
    let part1 = count(&data, false);
    let part2 = count(&data, true);
    (format!("{part1}"), format!("{part2}"))
}