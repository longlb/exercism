type Domino = (u8, u8);

pub fn chain(input: &[Domino]) -> Option<Vec<(u8, u8)>> {
    if input.len() == 0 {
        return Some(Vec::new());
    }
    let mut input = input.to_vec();
    let mut result = Vec::new();
    rec_chain(&mut input, &mut result);
    match result.len() {
        0 => None,
        _ => Some(result),
    }
}

fn rec_chain(input: &mut Vec<Domino>, result: &mut Vec<Domino>) -> bool {
    if input.len() == 0 {
        return is_valid_chain(result);
    }

    for _ in 0..input.len() {
        let domi = input.remove(0);
        result.push(domi);
        match rec_chain(input, result) {
            true => return true,
            false => {
                result.pop();
                ()
            }
        }
        result.push(flip(domi));
        match rec_chain(input, result) {
            true => return true,
            false => {
                result.pop();
                ()
            }
        }
        input.push(domi);
    }
    false
}

fn flip(dom: Domino) -> Domino {
    (dom.1, dom.0)
}

fn is_valid_chain(chain: &[Domino]) -> bool {
    let first = chain[0];
    let mut last = chain[0];
    for domino in chain[1..].iter() {
        if domino.0 != last.1 {
            return false;
        }
        last = *domino;
    }
    first.0 == last.1
}
