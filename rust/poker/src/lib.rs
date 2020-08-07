// /// Given a list of poker hands, return a list of those hands which win.
// ///
// /// Note the type signature: this function should return _the same_ reference to
// /// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
mod card;
mod hand;
use hand::Hand;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let new_hands: Vec<Hand> = hands.iter().map(|hand| Hand::from(hand)).collect();

    let highest = new_hands.iter().max().unwrap();

    let winners = new_hands
        .iter()
        .enumerate()
        .filter(|(_, hand)| hand == &highest);

    let mut result = Vec::new();
    for (i, _) in winners {
        result.push(hands[i]);
    }

    Some(result)
}
