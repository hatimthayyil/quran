mod common;

use common::{ayahs, lines};

fn whole(surah: u16, count: u16, basmala: bool) -> Vec<String> {
    let mut out = Vec::new();
    if basmala {
        out.push(format!("b{surah}"));
    }
    out.extend(ayahs(surah, 1..=count));
    out
}

#[test]
fn surah_one_has_no_separate_basmala() {
    assert_eq!(lines("1"), ayahs(1, 1..=7));
}

#[test]
fn surah_range_prints_each_whole_surah_with_basmala() {
    let expected = [whole(1, 7, false), whole(2, 286, true), whole(3, 200, true)].concat();
    assert_eq!(lines("1-3"), expected);
}

#[test]
fn surah_nine_has_no_basmala() {
    assert_eq!(lines("9"), whole(9, 129, false));
}

#[test]
fn single_ayah() {
    assert_eq!(lines("2:255"), ["2:255"]);
}

#[test]
fn ayah_range_with_end_shorthand() {
    assert_eq!(lines("2:12-16"), ayahs(2, 12..=16));
    assert_eq!(lines("2:12-2:16"), ayahs(2, 12..=16));
}

#[test]
fn range_across_surahs() {
    let expected = [ayahs(2, 282..=286), ayahs(3, 1..=5)].concat();
    assert_eq!(lines("2:282-3:5"), expected);
}

#[test]
fn range_covering_a_whole_surah_prints_its_basmala() {
    let expected = [ayahs(2, 282..=286), whole(3, 200, true), ayahs(4, 1..=5)].concat();
    assert_eq!(lines("2:282-4:5"), expected);
    assert_eq!(lines("2:1-286"), whole(2, 286, true));
}

#[test]
fn comma_union_in_query_order() {
    assert_eq!(lines("2:255,3:18"), ["2:255", "3:18"]);
    assert_eq!(lines("3:18,2:255"), ["3:18", "2:255"]);
}

#[test]
fn location_after_comma_is_complete() {
    let expected = [vec!["2:2".to_owned()], whole(3, 200, true)].concat();
    assert_eq!(lines("2:2,3"), expected);
}

#[test]
fn sets() {
    assert_eq!(lines("2:{3,255}"), ["2:3", "2:255"]);
    assert_eq!(
        lines("2:{1,5,10-12}"),
        ["2:1", "2:5", "2:10", "2:11", "2:12"]
    );
    assert_eq!(lines("2:{255,3}"), ["2:255", "2:3"]);
}

#[test]
fn set_above_a_range() {
    let expected = [ayahs(2, 1..=5), ayahs(3, 1..=5)].concat();
    assert_eq!(lines("{2,3}:1-5"), expected);
}

#[test]
fn whitespace_and_comma_mean_the_same() {
    assert_eq!(lines("2:3 2:255"), lines("2:3,2:255"));
    assert_eq!(lines("2:3 2:255"), lines("2:{3,255}"));
    assert_eq!(lines("  2:3 ,  2:255 "), ["2:3", "2:255"]);
}

#[test]
fn wildcard_surah() {
    let out = lines("*:1");
    assert_eq!(out.len(), 114);
    assert_eq!(out[..2], ["1:1", "2:1"]);
    assert_eq!(out[113], "114:1");
}

#[test]
fn wildcard_skips_missing_positions() {
    let out = lines("*:4");
    assert_eq!(out.len(), 111);
    for surah in [103, 108, 110] {
        assert!(!out.contains(&format!("{surah}:4")));
    }
}

#[test]
fn wildcard_with_range_skips_missing_positions() {
    let out = lines("*:2-4");
    assert_eq!(out.len(), 114 * 3 - 3);
    assert!(out.contains(&"103:3".to_owned()));
    assert!(!out.contains(&"103:4".to_owned()));
}

#[test]
fn set_of_surahs_skips_missing_positions() {
    assert_eq!(lines("{2,103}:4"), ["2:4"]);
}

#[test]
fn ayah_wildcard_is_the_whole_surah() {
    assert_eq!(lines("2:*"), lines("2"));
    assert_eq!(lines("2:*")[0], "b2");
}

#[test]
fn duplicates_repeat() {
    assert_eq!(lines("2:255 2:255"), ["2:255", "2:255"]);
}

#[test]
fn surah_set() {
    assert_eq!(lines("{112-114}").len(), 3 + 4 + 5 + 6);
}
