pub fn number(user_number: &str) -> Option<String> {
    let user_num = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>();

    match user_num.len() {
        10 => {
            if n_dig(user_num[0]) || n_dig(user_num[3]) {
                return None;
            }
            Some(user_num.into_iter().collect())
        }
        11 => {
            if user_num[0] != '1' || n_dig(user_num[1]) || n_dig(user_num[4]) {
                return None;
            }
            Some(user_num.into_iter().skip(1).collect())
        }
        _ => None,
    }
}

fn n_dig(c: char) -> bool {
    "01".contains(c)
}
