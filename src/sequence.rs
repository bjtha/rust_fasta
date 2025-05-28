#[derive(PartialEq, Debug)]
pub struct Sequence {

    pub name: String,
    pub bases: String,
}

impl Sequence {

    pub fn length(&self) -> usize {
        self.bases.len()
    }

    /// Returns the proportion of G or C residues in the sequence as a decimal.
    pub fn calculate_gc(&self) -> f64 {

        let gc_total: usize = self.bases
            .chars()
            .fold(0, |acc, b| acc + if b == 'g' || b == 'c' {1} else {0});

        gc_total as f64 / self.length() as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_sequence() -> Sequence {
        Sequence {
            name: String::from("TEST"),
            bases: String::from("atgcca")
        }
    }

    #[test]
    fn gc_calculation() {
        let seq = example_sequence();
        assert_eq!(seq.calculate_gc(), 0.5)
    }
}