use super::*;

#[test]
fn from_str() {
    assert_eq!(Ok(Format::Csv), Format::from_str("csv"));
    assert_eq!(Ok(Format::CsvNoHeader), Format::from_str("csv-no-header"));

    assert_eq!(Ok(Format::Tsv), Format::from_str("tsv"));
    assert_eq!(Ok(Format::TsvNoHeader), Format::from_str("tsv-no-header"));

    assert_eq!(Ok(Format::Json), Format::from_str("json"));
    assert_eq!(Ok(Format::LDJson), Format::from_str("ldjson"));
    assert_eq!(Ok(Format::JsonSeq), Format::from_str("json-seq"));

    assert_eq!(
        Err(FormatError::DoesNotExist("foobar".to_string())),
        Format::from_str("foobar")
    );
}

#[test]
fn line_csv() {
    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John", "32"]);
    Format::Csv.line_match(&mut buf, &extraction, true).unwrap();

    assert_eq!("John,32\n", &String::from_utf8(buf).unwrap(), "normal case");

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\"Foo", "32"]);
    Format::Csv.line_match(&mut buf, &extraction, true).unwrap();

    assert_eq!(
        "\"John\"\"Foo\",32\n",
        &String::from_utf8(buf).unwrap(),
        "with double quote in it"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John,Foo", "32"]);
    Format::Csv.line_match(&mut buf, &extraction, true).unwrap();

    assert_eq!(
        "\"John,Foo\",32\n",
        &String::from_utf8(buf).unwrap(),
        "with seperator in it"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\nFoo", "32"]);
    Format::Csv.line_match(&mut buf, &extraction, true).unwrap();

    assert_eq!(
        "\"John\nFoo\",32\n",
        &String::from_utf8(buf).unwrap(),
        "with newline in it"
    );
}

#[test]
fn line_tsv() {
    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John", "32"]);
    Format::Tsv.line_match(&mut buf, &extraction, true).unwrap();

    assert_eq!(
        "John\t32\n",
        &String::from_utf8(buf).unwrap(),
        "normal case"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\"Foo", "32"]);
    Format::Tsv.line_match(&mut buf, &extraction, true).unwrap();

    assert_eq!(
        "\"John\"\"Foo\"\t32\n",
        &String::from_utf8(buf).unwrap(),
        "with double quote in it"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\tFoo", "32"]);
    Format::Tsv.line_match(&mut buf, &extraction, true).unwrap();

    assert_eq!(
        "\"John\tFoo\"\t32\n",
        &String::from_utf8(buf).unwrap(),
        "with seperator in it"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\nFoo", "32"]);
    Format::Tsv.line_match(&mut buf, &extraction, true).unwrap();

    assert_eq!(
        "\"John\nFoo\"\t32\n",
        &String::from_utf8(buf).unwrap(),
        "with newline in it"
    );
}

#[test]
fn line_ldjson() {
    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John", "32"]);
    Format::LDJson
        .line_match(&mut buf, &extraction, true)
        .unwrap();

    assert_eq!(
        "{\"name\":\"John\",\"age\":\"32\"}\n",
        &String::from_utf8(buf).unwrap(),
        "normal case"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\"Doe", "32"]);
    Format::LDJson
        .line_match(&mut buf, &extraction, true)
        .unwrap();

    assert_eq!(
        "{\"name\":\"John\\\"Doe\",\"age\":\"32\"}\n",
        &String::from_utf8(buf).unwrap(),
        "normal case"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\nDoe", "32"]);
    Format::LDJson
        .line_match(&mut buf, &extraction, true)
        .unwrap();

    assert_eq!(
        "{\"name\":\"John\\nDoe\",\"age\":\"32\"}\n",
        &String::from_utf8(buf).unwrap(),
        "normal case"
    );
}

#[test]
fn line_jsonseq() {
    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John", "32"]);
    Format::JsonSeq
        .line_match(&mut buf, &extraction, true)
        .unwrap();

    assert_eq!(
        "\x1e{\"name\":\"John\",\"age\":\"32\"}\n",
        &String::from_utf8(buf).unwrap(),
        "normal case"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\"Doe", "32"]);
    Format::JsonSeq
        .line_match(&mut buf, &extraction, true)
        .unwrap();

    assert_eq!(
        "\x1e{\"name\":\"John\\\"Doe\",\"age\":\"32\"}\n",
        &String::from_utf8(buf).unwrap(),
        "normal case"
    );

    let mut buf = Vec::new();
    let extraction = Extraction::mock(vec!["name", "age"], vec!["John\nDoe", "32"]);
    Format::JsonSeq
        .line_match(&mut buf, &extraction, true)
        .unwrap();

    assert_eq!(
        "\x1e{\"name\":\"John\\nDoe\",\"age\":\"32\"}\n",
        &String::from_utf8(buf).unwrap(),
        "normal case"
    );
}

#[test]
fn err_msg() {
    assert_eq!(
        &format!("{}", FormatError::DoesNotExist("foobar".to_string())),
        "foobar is not a valid format"
    );
}
