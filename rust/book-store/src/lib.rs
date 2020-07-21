pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }
    Cart::from(books).sum()
}

struct Cart {
    book_sets: Vec<Vec<u32>>,
}

impl Cart {
    fn from(books: &[u32]) -> Self {
        let mut book_sets: Vec<Vec<u32>> = Vec::new();

        'outer: for book in books {
            for set in book_sets.iter_mut() {
                if !set.contains(book) {
                    set.push(*book);
                    if book_sets[0].len() == 4 {
                        book_sets.rotate_left(1);
                    }
                    continue 'outer;
                }
            }
            book_sets.push(vec![*book]);
        }

        println!("{:?}", book_sets);

        Self { book_sets }
    }

    fn sum(&self) -> u32 {
        self.book_sets
            .iter()
            .fold(0, |acc, x| acc + self.pricing(x.len()))
    }

    fn pricing(&self, num_books: usize) -> u32 {
        match num_books {
            5 => 3000,
            4 => 2560,
            3 => 2160,
            2 => 1520,
            1 => 800,
            _ => 0,
        }
    }
}
