pub struct CsvParser {}

#[cfg(test)]
mod tests {
    #[test]
    fn can_parse_csv() {
        assert!(false);
    }

    use csv::{ReaderBuilder, Trim};
    use std::error::Error;
    use std::io::Cursor;

    fn main() -> Result<(), Box<dyn Error>> {
        // Your CSV data
        let data = r#"Code 1, Code 2, Code 3, Category
1,,,ASSETS,,
,,,,,
11,,,ASSETS,,
,,,,,
111,,,LIQUID ASSETS,,
,,,,,
1110,,,LIQUID ASSETS,,
    ,,,,,
    ,01,,CASH,,
,,0101,Head Office - Local Currency,,
,,0102,Head Office - Foreign Currency,,
,,0201,Branches - Local Currency,,
,,0202,Branches - Foreign Currency,,"#;

        // Create a CSV reader with appropriate configuration
        let mut rdr = ReaderBuilder::new()
            .flexible(true) // Handle records with different field counts
            .trim(Trim::All) // Trim whitespace from fields
            .from_reader(Cursor::new(data));

        // Read the header record
        let headers = rdr.headers()?.clone();
        println!("Headers: {:?}", headers);

        // Process each record
        for result in rdr.records() {
            match result {
                Ok(record) => {
                    // Skip empty rows (all fields empty)
                    if record.iter().all(|field| field.is_empty()) {
                        continue;
                    }

                    let code1 = record.get(0).unwrap_or("").trim();
                    let code2 = record.get(1).unwrap_or("").trim();
                    let code3 = record.get(2).unwrap_or("").trim();
                    let category = record.get(3).unwrap_or("").trim();

                    println!(
                        "Code1: '{:10}' | Code2: '{:5}' | Code3: '{:5}' | Category: '{}'",
                        code1, code2, code3, category
                    );
                }
                Err(e) => eprintln!("Error reading record: {}", e),
            }
        }

        Ok(())
    }
}
