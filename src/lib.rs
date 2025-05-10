use std::fs;

#[derive(Debug)]
pub struct Sequence {
    pub name: String,
    pub bases: String,
}

impl Sequence {

    pub fn length(&self) -> usize {
        self.bases.len()
    }

    pub fn calculate_gc(&self) -> f64 {

        let gc_total: usize = self.bases
            .chars()
            .fold(0, |acc, b| acc + if b == 'g' || b == 'c' {1} else {0});

        gc_total as f64 / self.length() as f64
    }
}


fn extract_sequences(fasta_string: String) -> Vec<Sequence> {

    let lines: Vec<&str> = fasta_string.lines().collect();

    let mut seq_lines: Vec<String> = vec![];
    let mut sequences: Vec<Sequence> = vec![];

    // Lines are reversed to simplify dealing with the end-of-file.
    for line in lines.into_iter().rev() {
        if line.starts_with('>') {
            let name = line.trim_matches(|c| c == ' ' || c == '>').to_string();
            let bases = seq_lines.join("");
            sequences.push(
                Sequence { name, bases }
            );
            seq_lines.clear()
        } else {
            seq_lines.insert(0, line.trim().to_lowercase());
        }
    }
    sequences
}

pub fn read(path: &str) -> Vec<Sequence> {
    let contents = fs::read_to_string(path).unwrap();
    extract_sequences(contents)
}
