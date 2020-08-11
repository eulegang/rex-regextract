use regex::Regex;

pub struct Extractor {
    regex: Regex,
    headers: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Extraction<'e, 'line> {
    pub header: Vec<&'e str>,
    pub values: Vec<&'line str>,
}

impl Extractor {
    pub fn new(regex: Regex) -> Extractor {
        let headers = regex
            .capture_names()
            .filter_map(|name| name.map(ToOwned::to_owned))
            .collect();
        Extractor { regex, headers }
    }

    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    pub fn extract<'s, 'line>(&'s self, line: &'line str) -> Option<Extraction<'s, 'line>> {
        self.regex.captures(line).map(|captures| {
            let mut header = Vec::new();
            let mut values = Vec::new();

            for (i, name) in self.regex.capture_names().enumerate() {
                if let Some(name) = name {
                    header.push(name);
                    values.push(captures.get(i).unwrap().as_str());
                }
            }

            Extraction { header, values }
        })
    }
}
