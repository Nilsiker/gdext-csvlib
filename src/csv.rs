use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Object, init)]
struct TranslationCsv {
    headers: Array<GString>,
    records: Array<Array<GString>>,
}

#[godot_api]
impl TranslationCsv {
    #[func]
    fn debug() {
        godot_print!("debug from csvlib")
    }

    pub fn parse(path: String) -> Self {
        let mut reader = csv::Reader::from_path(path).unwrap();
        let headers = reader
            .headers()
            .into_iter()
            .flatten()
            .map(|f| f.into())
            .collect();
        let records = reader
            .into_records()
            .flatten()
            .map(|r| r.into_iter().map(|s| s.into()).collect())
            .collect();

        Self { headers, records }
    }

    pub fn update_value(&mut self, key: String, locale: String) {}
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    const CSV: &str = r"
keys,en,de,fr
GREET,Hello,Hallo,Bonjour
GOODBYE,Bye,Tschüss,Au revoir
";

    #[test]
    fn parse_example_csv() -> anyhow::Result<()> {
        let csv = super::TranslationCsv::parse("res/example.csv".to_string());
        println!("{csv:#?}");
        Ok(())
    }

    #[test]
    fn update_line() -> anyhow::Result<()> {
        Ok(())
    }
}
