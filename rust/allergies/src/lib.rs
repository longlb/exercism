#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    Fail,
}

pub struct Allergies {
    allergies: Vec<Allergen>,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            allergies: Allergies::calc_allergies_from(score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }

    fn calc_allergies_from(score: u32) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        for i in 0..8 {
            let n = (2_u32).pow(i);
            println!("{:?}", score & n);
            if score & n != 0 {
                allergies.push(Allergies::from_num(n));
            }
        }
        println!("{:?}", allergies);
        allergies
    }

    fn from_num(num: u32) -> Allergen {
        match num {
            1 => Allergen::Eggs,
            2 => Allergen::Peanuts,
            4 => Allergen::Shellfish,
            8 => Allergen::Strawberries,
            16 => Allergen::Tomatoes,
            32 => Allergen::Chocolate,
            64 => Allergen::Pollen,
            128 => Allergen::Cats,
            _ => Allergen::Fail,
        }
    }
}
