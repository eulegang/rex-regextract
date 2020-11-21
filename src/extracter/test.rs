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

#[test]
fn extraction_multi() {
    let extractor = Extractor::new(Regex::new("name: (?P<name>\\S+)").unwrap());

    assert_eq!(
        vec![
            Extraction::mock(vec!["name"], vec!["John"]),
            Extraction::mock(vec!["name"], vec!["Jane"])
        ],
        extractor.extract_multi("name: John\tname: Jane")
    );
}
