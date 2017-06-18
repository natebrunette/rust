use std::collections::HashMap;

pub struct CodonInfo<'a> {
    actual_codons: HashMap<&'a str, &'a str>
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonInfo<'a> {
    CodonInfo{
        actual_codons: pairs.into_iter().collect()
    }
}

impl<'a> CodonInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Result<&'a str, &'static str> {
        match self.actual_codons.get(&codon) {
            Some(name) => Ok(name),
            None => Err("Invalid")
        }
    }

    pub fn of_rna(&self, strand: &str) -> Result<Vec<&'a str>, &'static str> {
        strand.chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|chars| self.name_for(&chars.iter().collect::<String>()))
            .take_while(|result| result.is_err() || result.unwrap() != "stop codon")
            .collect()
    }
}
