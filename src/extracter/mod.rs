use regex::Regex;

#[cfg(test)]
mod test;

pub struct Extractor {
    regex: Regex,
    headers: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Extraction<'e, 'line> {
    header: Vec<&'e str>,
    values: Vec<&'line str>,
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

    pub fn extract_multi<'s, 'line>(&'s self, line: &'line str) -> Vec<Extraction<'s, 'line>> {
        self.regex
            .captures_iter(line)
            .map(|captures| {
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
            .collect()
    }
}

impl<'extractor, 'line> Extraction<'extractor, 'line> {
    #[allow(dead_code)]
    pub(crate) fn mock(
        header: Vec<&'extractor str>,
        values: Vec<&'line str>,
    ) -> Extraction<'extractor, 'line> {
        Extraction { header, values }
    }

    pub fn pairs(&self) -> Vec<(&'extractor str, &'line str)> {
        let mut buf = Vec::with_capacity(self.header.len());

        for i in 0..self.header.len() {
            buf.push((self.header[i], self.values[i]));
        }

        buf
    }

    pub fn values(&self) -> &[&'line str] {
        &self.values
    }
}
