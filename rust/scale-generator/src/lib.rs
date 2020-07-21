// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
pub type Error = ();

const SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];
const SHARP_MAP: [&str; 14] = [
    "C", "G", "D", "A", "E", "B", "F#", "a", "e", "b", "f#", "c#", "g#", "d#",
];
pub struct Scale {
    scale: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale_base = match SHARP_MAP.contains(&tonic) {
            true => SHARPS,
            false => FLATS,
        }
        .iter()
        .cloned()
        .cycle()
        .skip_while(|&note| note.to_lowercase() != tonic.to_lowercase())
        .take(12)
        .collect::<Vec<&str>>();

        let intervals = intervals
            .chars()
            .map(|i| match i {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => 0,
            })
            .collect::<Vec<usize>>();

        let mut scale = Vec::new();
        let mut index = 0;
        for i in intervals {
            scale.push(scale_base[index].to_string());
            index += i;
            if index > 11 {
                break;
            }
        }

        Ok(Self { scale })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.clone()
    }
}
