extern crate log;
extern crate simplelog;

use clap::{App, Arg, ArgMatches};
use simplelog::*;
use std::io::Write;

fn main() {
    // initialize logger
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());
    let commands: ArgMatches = App::new("acr_crash_table")
        .version("0.2")
        .author("Mark F Rodriguez")
        .about("ACR Crash Table Generator")
        .arg(
            Arg::with_name("source")
                .help("Crash data source")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("destination")
                .help("Crash data destination")
                .required(false)
                .index(1),
        )
        .get_matches();
    let source = commands.value_of("source").unwrap_or("");
    log::info!("Source CSV file: {}", source);
    let destination = commands.value_of("destination").unwrap_or("./");
    log::info!("Output path: {}", destination);
    crash_generate_table(source, destination);
    log::info!("Table generation done.");
}

/// CSV field positions
const FIXED_WING_G_MIN_POSITION: usize = 2;
const FIXED_WING_G_MAX_POSITION: usize = 3;
const HELICOPTER_G_MIN_POSITION: usize = 4;
const HELICOPTER_G_MAX_POSITION: usize = 5;
const AFTWARD_G_MIN_POSITION: usize = 6;
const AFTWARD_G_MAX_POSITION: usize = 7;

fn crash_generate_table(in_file: &str, out_file: &str) {
    log::info!("Generating crash data array...");
    // pass file into CSV reader for processing
    let reader = csv::Reader::from_path(in_file);
    match reader {
        Ok(mut rdr) => {
            let mut fixed_wing_table: String = String::new();
            let mut helicopter_table: String = String::new();
            let mut aftward_table: String = String::new();

            let mut c_file = std::fs::File::create(out_file).expect("create failed");
            c_file
                .write_all("#include \"crash_table.h\"\n".as_bytes())
                .expect("write failed");
            c_file
                .write_all("#include \"crash.h\"\n".as_bytes())
                .expect("write failed");
            c_file.write_all("\n".as_bytes()).expect("write failed");
            fixed_wing_table
                .push_str("const crash_table_record fixed_wing_table[ACCEL_FIFO_AXIS_SIZE] = {\n");
            helicopter_table
                .push_str("const crash_table_record helicopter_table[ACCEL_FIFO_AXIS_SIZE] = {\n");
            aftward_table.push_str("const crash_table_record aftward_table[ACCEL_FIFO_AXIS_SIZE] = {\n");
            for result in rdr.records() {
                // The iterator yields Result<StringRecord, Error>, so we check the
                // error here.
                let record = result.expect("Bad crash record");
                // fixed wing table
                let output_line = format!(
                    "    {{{}, {}}},\n",
                    record[FIXED_WING_G_MIN_POSITION]
                        .parse::<String>()
                        .unwrap_or("0".to_string()),
                    record[FIXED_WING_G_MAX_POSITION]
                        .parse::<String>()
                        .unwrap_or("0".to_string())
                );
                fixed_wing_table.push_str(&output_line);
                // helicopter table
                let output_line = format!(
                    "    {{{}, {}}},\n",
                    record[HELICOPTER_G_MIN_POSITION]
                        .parse::<String>()
                        .unwrap_or("0".to_string()),
                    record[HELICOPTER_G_MAX_POSITION]
                        .parse::<String>()
                        .unwrap_or("0".to_string())
                );
                helicopter_table.push_str(&output_line);
                // aftward table
                let output_line = format!(
                    "    {{{}, {}}},\n",
                    record[AFTWARD_G_MIN_POSITION]
                        .parse::<String>()
                        .unwrap_or("0".to_string()),
                    record[AFTWARD_G_MAX_POSITION]
                        .parse::<String>()
                        .unwrap_or("0".to_string())
                );
                aftward_table.push_str(&output_line);
            }

            fixed_wing_table.push_str("};\n\n");
            helicopter_table.push_str("};\n\n");
            aftward_table.push_str("};\n");

            // write tables out to file
            c_file
                .write_all(fixed_wing_table.as_bytes())
                .expect("write failed");
            c_file
                .write_all(helicopter_table.as_bytes())
                .expect("write failed");
            c_file
                .write_all(aftward_table.as_bytes())
                .expect("write failed");
        }
        _ => {
            log::error!("File not found or not valid data log");
        }
    };
}

// Local tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_header() {
        crash_generate_table("./acr_crash_analysis.csv", "crash_table.c");
        assert_eq!(true, true);
    }
}
