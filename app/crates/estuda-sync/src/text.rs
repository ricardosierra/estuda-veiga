//! Utilidades de texto determinísticas (sem I/O).

use sha2::{Digest, Sha256};
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

/// Minúsculas, sem acentos e com espaços colapsados. Usado só para comparar rótulos.
pub fn fold(input: &str) -> String {
    let stripped: String = input
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .flat_map(char::to_lowercase)
        .collect();
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// `true` se `ch` conta como parte de um identificador (letra/dígito).
pub fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_strips_accents_case_and_spaces() {
        assert_eq!(fold("  Espaço   CURSO "), "espaco curso");
        assert_eq!(fold("Ciência da Computação"), "ciencia da computacao");
    }

    #[test]
    fn hash_is_stable() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
