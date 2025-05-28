use std::fs;
mod sequence;

use sequence::Sequence;

/// Checks that a string slice contains only valid DNA symbols.
fn validate_dna(bases: &str) -> bool {
    bases.chars().all(|c| matches!(c, 'a' | 'c' | 'g' | 't'))
}

/// Parses a set of sequences formatted as a fasta file into Sequence structs.
/// Returns an Err if an invalid sequence is encountered.
fn extract_sequences(fasta_string: String) -> Result<Vec<Sequence>, String> {

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
            let clean_line = line.trim().to_lowercase();
            if validate_dna(&clean_line) {
                seq_lines.insert(0, clean_line)
            } else {
                return Err(String::from("File contains invalid character"))
            }
        }
    }
    Ok(sequences)
}

/// Read a fasta-formatted string into a vector of Sequence structs.
pub fn reads(fasta_string: String) -> Vec<Sequence> {

    match extract_sequences(fasta_string) {
        Ok(seqs) => seqs,
        Err(msg) => panic!("{}", msg)
    }
}


/// Read a .fasta file into a vector of Sequence structs.
pub fn read(path: &str) -> Vec<Sequence> {

    let contents = fs::read_to_string(path).unwrap();
    reads(contents)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_string() {
        let fasta = String::from(">A\naattggcc");
        let sequences = reads(fasta);
        let first = sequences.first().unwrap();

        assert_eq!(first.bases, "aattggcc");
    }

    #[test]
    fn empty_file() {
        let fasta = String::from("");
        let sequences = reads(fasta);
        assert!(sequences.is_empty())
    }

    #[test]
    fn bad_character() {
        let fasta = String::from(">A\nD");
        assert!(extract_sequences(fasta).is_err())
    }
}
