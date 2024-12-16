use evtx::EvtxParser;
use std::path::PathBuf;

fn main() {
    // Define the path to the .evtx file
    let fp = PathBuf::from(format!(
        "{}/samples/security.evtx",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ));

    // Create a parser for the .evtx file
    let mut parser = EvtxParser::from_path(fp).expect("Failed to open the .evtx file");

    // Iterate through the records
    for record in parser.records() {
        match record {
            Ok(r) => {
                println!("Record ID: {}\nData: {}", r.event_record_id, r.data);
            }
            Err(e) => {
                eprintln!("Error parsing record: {}", e);
            }
        }
    }
}
