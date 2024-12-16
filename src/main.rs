use std::process::Command;

fn export_logs(logs: &[&str], output_dir: &str) {
    for log in logs {
        let output_file = format!("{}/{}.evtx", output_dir, log);

        println!("Exporting log: {} to {}", log, output_file);

        let result = Command::new("wevtutil")
            .args(&["epl", log, &output_file])
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    println!("Successfully exported: {}", log);
                } else {
                    eprintln!(
                        "Failed to export: {}. Error: {}",
                        log,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => eprintln!("Error running command for {}: {}", log, e),
        }
    }
}

fn main() {
    // List of logs to export
    let logs = ["Application", "Security", "System"];
    
    // Output directory for exported logs
    let output_dir = "C:/Logs";

    // Create output directory if needed
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Export the logs
    export_logs(&logs, output_dir);
}
