fn main() {

    use fasta::read;

    let seqs = read("./data/adh_dna.fasta");
    let first = seqs.first().unwrap();
    println!("{}", first.calculate_gc())
}
