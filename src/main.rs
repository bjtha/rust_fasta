use std::fs;

fn main() {
    let seqs = read("./data/adh_dna.fasta");
    println!("{:?}", seqs)
}

#[derive(Debug)]
pub struct Sequence {
    pub name: String,
    pub bases: String,
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

fn read(path: &str) -> Vec<Sequence> {
    let contents = fs::read_to_string(path).unwrap();
    extract_sequences(contents)
}
