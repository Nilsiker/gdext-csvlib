use csv::{ReaderBuilder, WriterBuilder};
use godot::{
    engine::{file_access::ModeFlags, FileAccess},
    prelude::*,
};
use std::fs::File;

#[derive(Debug)]
struct CsvRow {
    keys: String,
    values: Vec<String>,
}

#[derive(GodotClass, Debug)]
#[class(no_init)]
struct TranslationCsv {
    headers: Vec<String>,
    rows: Vec<CsvRow>,
}

#[godot_api]
impl TranslationCsv {
    #[func]
    fn from_path(path: String) -> Gd<Self> {
        let f = FileAccess::open(path.clone().into(), ModeFlags::READ).expect("existing file");
        let path = f.get_path_absolute().to_string();
        let mut rdr = ReaderBuilder::new()
            .flexible(true)
            .from_path(path)
            .expect("file exists");
        let headers = rdr
            .headers()
            .expect("has headers")
            .clone()
            .into_iter()
            .skip(1)
            .map(String::from)
            .collect();
        let mut rows = Vec::new();

        for result in rdr.records() {
            let record = result.expect("has record");
            let keys = record.get(0).unwrap().to_string();
            let values = record.iter().skip(1).map(String::from).collect();
            rows.push(CsvRow { keys, values });
        }

        Gd::from_object(TranslationCsv { headers, rows })
    }

    #[func]
    fn write_csv(&self, path: String) {
        let f = FileAccess::open(path.into(), ModeFlags::WRITE).expect("can open");
        let path = f.get_path_absolute().to_string();
        let TranslationCsv { headers, rows } = self;

        let file = File::create(path).expect("can create file at path");
        let mut wtr = WriterBuilder::new().flexible(true).from_writer(file);

        // Write headers
        let mut csv_headers = vec!["keys".to_string()];
        csv_headers.extend(headers.iter().cloned());
        wtr.write_record(&csv_headers).expect("can write headers");

        // Write rows
        for row in rows {
            let mut record = vec![row.keys.clone()];
            record.extend(row.values.iter().cloned());
            wtr.write_record(&record).expect("can write record");
        }

        wtr.flush().expect("can flush writer");
    }

    #[func]
    fn replace_value(&mut self, key: String, language: String, new_value: String) {
        let lang_index = self
            .headers
            .iter()
            .position(|h| *h == language)
            .expect("Language not found");
        for row in self.rows.iter_mut() {
            if row.keys.clone() == key && lang_index < row.values.len() {
                row.values[lang_index] = new_value.to_string();
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    #[test]
    fn run_in_out() -> Result<(), Box<dyn Error>> {
        let file_path = "in.csv";
        let mut translation = TranslationCsv::from_path(file_path.into());

        // Replace a value (e.g., replacing the English value)
        translation
            .bind_mut()
            .replace_value("GREET".into(), "en".into(), "HOWDY!!".into());

        // Write back to CSV
        translation.bind().write_csv("out.csv".into());

        Ok(())
    }
}
