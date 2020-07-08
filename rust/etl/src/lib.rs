use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map = BTreeMap::new();

    for (key, value) in h.iter() {
        for c in value.iter() {
            map.insert(c.to_ascii_lowercase(), *key);
        }
    }

    map
}
