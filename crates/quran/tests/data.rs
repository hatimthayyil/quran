use quran::{Quran, Text};
use sha2::{Digest, Sha256};

fn sha256(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
fn embedded_files_are_the_pinned_downloads() {
    assert_eq!(
        sha256(Text::Uthmani.source()),
        "203f0f1bf3158b1e5be4ab9f8f6870e570aab6d9a626fe6192a70b75d4afe0fd"
    );
    assert_eq!(
        sha256(Text::SimpleClean.source()),
        "140787b7c399b11d54bfa72c237075916084232efa8a4cddf704cc8a2a74b1fe"
    );
}

#[test]
fn both_texts_load_with_full_counts() {
    for text in [Text::Uthmani, Text::SimpleClean] {
        let quran = Quran::load(text).expect("loads");
        let ayahs: usize = (1..=114).map(|s| usize::from(quran.ayah_count(s))).sum();
        assert_eq!(ayahs, 6236);
        assert_eq!(quran.ayah_count(2), 286);
        assert_eq!(quran.ayah_count(103), 3);
    }
}

#[test]
fn basmala_comes_from_the_source() {
    let quran = Quran::load(Text::Uthmani).expect("loads");
    assert_eq!(quran.basmala(1), None);
    assert_eq!(quran.basmala(9), None);
    let with_basmala = (1..=114).filter(|&s| quran.basmala(s).is_some()).count();
    assert_eq!(with_basmala, 112);
    assert_ne!(quran.basmala(95), quran.basmala(2));
    assert_ne!(quran.basmala(97), quran.basmala(2));
    assert_eq!(quran.ayah(2, 1), "الٓمٓ");
}

#[test]
fn notice_is_the_verbatim_copyright_block() {
    let notice = Text::Uthmani.notice();
    assert!(notice.starts_with("# PLEASE DO NOT REMOVE OR CHANGE THIS COPYRIGHT BLOCK"));
    assert!(notice.contains("Tanzil Quran Text (Uthmani, Version 1.1)"));
    assert!(Text::Uthmani.source().contains(notice));
    assert!(
        Text::SimpleClean
            .notice()
            .contains("Simple Clean, Version 1.1")
    );
}
