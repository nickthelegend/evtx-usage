use evtx::EvtxParser;
use std::path::PathBuf;
use quick_xml::de::from_str;

fn main() {
    // Path to the .evtx file
    let fp = PathBuf::from(format!(
        "{}/samples/security.evtx",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ));

    // Open the .evtx file and create a parser
    let mut parser = EvtxParser::from_path(fp).expect("Failed to open the .evtx file");

    // Iterate through the records in the .evtx file
    for record in parser.records() {
        match record {
            Ok(r) => {
                // Parse the XML data of the record
                let data = r.data;

                // Check for Event ID 4624 and Logon Type 2
                if is_interactive_logon(&data) {
                    println!("Record ID: {}\nDate: {}", r.event_record_id, r.timestamp);
                }
            }
            Err(e) => {
                eprintln!("Error parsing record: {}", e);
            }
        }
    }
}

/// Function to check if the record contains Event ID 4624 with Logon Type 2
fn is_interactive_logon(xml: &str) -> bool {
    if let Ok(event) = from_str::<Event>(xml) {
        if event.system.event_id == "4624" {
            for data in event.event_data.data {
                if let Some(name) = &data.name {
                    if name == "LogonType" {
                        if let Some(value) = &data.value {
                            return value == "5"; // Check for LogonType 2
                        }
                    }
                }
            }
        }
    }
    false
}


/// Struct to deserialize specific fields from the event data
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Event {
    #[serde(rename = "System")]
    system: System,
    #[serde(rename = "EventData")]
    event_data: EventData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct System {
    #[serde(rename = "EventID")]
    event_id: String,
    #[serde(rename = "EventRecordID")]
    event_record_id: String,
    #[serde(rename = "TimeCreated")]
    time_created: TimeCreated,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TimeCreated {
    #[serde(rename = "SystemTime")]
    system_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EventData {
    #[serde(rename = "Data", default)]
    data: Vec<Data>,
}

#[derive(Debug, Deserialize)]
struct Data {
    #[serde(rename = "Name", default)]
    name: Option<String>,
    #[serde(rename = "$value", default)]
    value: Option<String>,
}
