use assert_cmd::Command;
use quran::{Quran, Text};

fn quran() -> Command {
    Command::cargo_bin("quran").expect("binary builds")
}

fn stdout(args: &[&str]) -> String {
    let output = quran().args(args).assert().success().get_output().clone();
    String::from_utf8(output.stdout).expect("utf-8")
}

fn uthmani() -> Quran {
    Quran::load(Text::Uthmani).expect("loads")
}

#[test]
fn prints_one_ayah_with_its_location() {
    let expected = format!("2:255\t{}\n", uthmani().ayah(2, 255));
    assert_eq!(stdout(&["2:255"]), expected);
}

#[test]
fn whole_surah_starts_with_an_unprefixed_basmala() {
    let quran = uthmani();
    let out = stdout(&["112"]);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 5);
    assert_eq!(Some(lines[0]), quran.basmala(112));
    assert_eq!(lines[1], format!("112:1\t{}", quran.ayah(112, 1)));
}

#[test]
fn arguments_are_joined_by_spaces() {
    assert_eq!(stdout(&["2:3", "2:255"]), stdout(&["2:{3,255}"]));
}

#[test]
fn simple_clean_text() {
    assert_eq!(
        stdout(&["--text", "simple-clean", "1:1"]),
        "1:1\tبسم الله الرحمن الرحيم\n"
    );
}

#[test]
fn errors_go_to_stderr_with_a_code() {
    let assert = quran().arg("2:300").assert().failure().stdout("");
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).expect("utf-8");
    assert!(stderr.contains("quran::bounds::ayah"));
    assert!(stderr.contains("surah 2 has 286 ayahs"));
}

#[test]
fn license_prints_the_tanzil_notice() {
    let out = stdout(&["--license"]);
    assert!(out.contains("Tanzil Quran Text (Uthmani, Version 1.1)"));
    assert!(out.contains("https://tanzil.net"));
}

#[test]
fn help_names_tanzil() {
    assert!(stdout(&["--help"]).contains("Tanzil"));
}
