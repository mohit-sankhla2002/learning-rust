use std::error::Error;
use csv;

fn main() {
    if let Err(e) = read_from_file("./demo.csv") {
        eprintln!("{e}");
    }
}

fn read_from_file(filepath: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(filepath)?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

