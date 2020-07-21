#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut data = input.to_vec();
        data.sort();
        Self { data }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(&element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
            self.data.sort();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for t in &self.data {
            if !other.contains(t) {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for t in &self.data {
            if other.contains(t) {
                return false;
            }
        }
        true
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|&t| other.contains(t))
                .cloned()
                .collect::<Vec<T>>(),
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|&t| !other.contains(t))
                .cloned()
                .collect::<Vec<T>>(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut data = self.data.clone();
        data.extend(
            other
                .data
                .clone()
                .into_iter()
                .filter(|t| !self.contains(t))
                .collect::<Vec<T>>(),
        );
        data.sort();
        Self { data }
    }
}
