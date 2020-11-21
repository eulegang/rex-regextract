use super::*;

#[test]
fn new_extractor() {
    let extractor = Extractor::new(Regex::new("name: (?P<name>\\S+)").unwrap());
    assert_eq!(&["name"], &extractor.headers());
}

#[test]
fn extraction() {
    let extractor = Extractor::new(Regex::new("name: (?P<name>\\S+)").unwrap());

    assert_eq!(
        Some(Extraction::mock(vec!["name"], vec!["John"])),
        extractor.extract("name: John\tname: Jane")
    );
}
