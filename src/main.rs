use chrono::{NaiveDate, Local, Duration, Datelike};
use std::process::exit;

fn main() {
    let epoch = get_command_line_args();

    match epoch.0 {
        Some(distance) => {
            if let Some(formatted_epoch) = get_epoch_days(distance, epoch.1.unwrap_or_else(|| "iso".to_string())) {
                println!("{}", formatted_epoch);
            } else {
                eprintln!("Error formatting the date.");
                exit(-1);
            }
        },
        None => {
            eprintln!("No valid date calculation requested.");
            exit(-1);
        }
    }
}

fn get_epoch_days(distance: i64, format: String) -> Option<String> {
    let current_date = Local::now().naive_local().date();
    let future_date = current_date + Duration::days(distance);
    return format_date(future_date, &format);
}

fn format_date(date: NaiveDate, format: &str) -> Option<String> {
    match format {
        "gen" => Some(
            format!("{:02}/{:02}/{}", date.day(), date.month(), date.year())
            ),
        "us" => Some(
            format!("{:02}/{:02}/{}", date.month(), date.day(), date.year())
            ),
        "iso" => Some(date.to_string()),
        _ => {
            eprintln!("Unknown format type.");
            return None;
        }
    }
}

fn parse_days(args: &[String], i: &mut usize) -> Option<i64> {
    if *i + 1 < args.len() {
        match args[*i + 1].parse::<i64>() {
            Ok(value) => {
                *i += 1;
                Some(value)
            },
            Err(_) => {
                eprintln!("Invalid value for --days: '{}'. Please provide a valid integer.", args[*i + 1]);
                exit(-1);
            }
        }
    } else {
        eprintln!("Invalid or missing value for --days <number>.");
        exit(-1);
    }
}

fn get_command_line_args() -> (Option<i64>, Option<String>) {
    let args: Vec<String> = std::env::args().collect();
    let mut days: Option<i64> = None;
    let mut format: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--days" | "-d" => {
                days = parse_days(&args, &mut i);
            },
            "--format" | "-f" => {
                if i + 1 < args.len() {
                    format = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for --format.");
                    std::process::exit(-1);
                }
            },
            "--help" | "-h" => {
                display_help();
                return (None, None);
            }
            _ => {
                if days.is_none() {
                    match args[i].parse::<i64>() {
                        Ok(value) => days = Some(value),
                        Err(_) => {
                            eprintln!("Invalid value for days: '{}'. Please provide a valid integer.", args[i]);
                            exit(-1);
                        }
                    }
                } else {
                    eprintln!("Unknown argument: {}", args[i]);
                    exit(-1);
                }
            },
        }
        i += 1;
    }

    return (days, format);
}

fn display_help() {
    println!(
        r#"
NAME
    dtell - Calculate dates based on a number of days relative to today.

SYNOPSIS
    dtell [--days | -d <number>] [--format | -f <type>] [<number>]

DESCRIPTION
    dtell calculates the date a specified number of days in the future or past relative to the current date.

    If the --days or -d option is provided, it specifies the number of days to add or subtract.

    If the --format or -f option is provided, it specifies the format in which the date should be displayed. Supported formats include:
    
        gen    - The date is displayed in the format DD/MM/YYYY.
        us     - The date is displayed in the format MM/DD/YYYY.
        iso    - The date is displayed in ISO format (YYYY-MM-DD). This is the default if no format is specified.

    If no command-line options are provided, the first argument is assumed to be the number of days.

OPTIONS
    --days, -d <number>   Specify the number of days to add or subtract relative to today.
    --format, -f <type>   Specify the output format for the date. Supported types are gen, us, iso.

EXAMPLES
    dtell --d 5 --f iso
        Displays the date 5 days in the future in ISO format.

    dtell 5
        Displays the date 5 days in the future in ISO format (default).

    dtell --days -10 --format us
        Displays the date 10 days in the past in US format (MM/DD/YYYY).

    dtell -d cat
        Displays an error message, as 'cat' is not a valid integer.

    dtell --days 7
        Displays the date 7 days in the future in ISO format (default format).

DIAGNOSTICS
    Invalid or missing value for --days | -d
        Occurs when the value following the --days or -d option is not a valid integer.
    
    Invalid value for days
        Occurs when the first positional argument is not a valid integer.
    
    Missing value for --format | -f
        Occurs when the --format or -f flag is provided without a corresponding format type.
    
    Unknown format type
        Occurs when the value of the --format or -f option is not one of the supported formats.
    
    Unknown argument
        Occurs when an unrecognized argument is provided.

SEE ALSO
    date(1)

HISTORY
    dtell written by Daksh Kaul / DriftingOtter, © 2025, and licensed under GNU General Public License version 3 (GPLv3).

BUGS
    No known bugs at this time.
"#
    );
}

