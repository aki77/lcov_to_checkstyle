use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn parse_lcov(file_path: &str) -> Result<BTreeMap<String, Vec<u32>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut uncovered_files: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    let mut current_file = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("SF:") {
            current_file = Some(line[3..].trim().to_string());
        } else if line.starts_with("DA:") {
            let parts: Vec<&str> = line[3..].split(',').collect();
            let line_number: u32 = parts[0].parse().unwrap();
            let hit_count: u32 = parts[1].parse().unwrap();
            if hit_count == 0 {
                if let Some(file) = &current_file {
                    uncovered_files
                        .entry(file.clone())
                        .or_insert_with(Vec::new)
                        .push(line_number);
                }
            }
        }
    }

    uncovered_files.retain(|_, lines| !lines.is_empty());
    Ok(uncovered_files)
}

fn group_consecutive_lines(lines: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut grouped_lines = Vec::new();
    let mut current_group = Vec::new();

    for &line in lines {
        if current_group.is_empty() {
            current_group.push(line);
        } else if line == current_group.last().unwrap() + 1 {
            current_group.push(line);
        } else {
            grouped_lines.push(current_group);
            current_group = vec![line];
        }
    }

    if !current_group.is_empty() {
        grouped_lines.push(current_group);
    }

    grouped_lines
}

fn convert_to_checkstyle_format(uncovered_files: BTreeMap<String, Vec<u32>>) -> Vec<u8> {
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 4);
    let mut checkstyle_start = BytesStart::new("checkstyle");
    checkstyle_start.push_attribute(("version", "4.3"));
    writer.write_event(Event::Start(checkstyle_start)).unwrap();

    for (file, lines) in uncovered_files {
        let mut file_start = BytesStart::new("file");
        file_start.push_attribute(("name", file.as_str()));
        writer.write_event(Event::Start(file_start)).unwrap();

        let grouped_lines = group_consecutive_lines(&lines);

        for group in grouped_lines {
            let message = if group.len() > 1 {
                format!("Lines {}-{} are not covered", group[0], group[group.len() - 1])
            } else {
                format!("Line {} is not covered", group[0])
            };

            let line = group[group.len() - 1];
            let mut error_start = BytesStart::new("error");
            error_start.push_attribute(("line", line.to_string().as_str()));
            error_start.push_attribute(("severity", "warning"));
            error_start.push_attribute(("message", message.as_str()));
            error_start.push_attribute(("source", "coverage"));
            writer.write_event(Event::Empty(error_start)).unwrap();
        }

        writer.write_event(Event::End(BytesEnd::new("file"))).unwrap();
    }

    writer.write_event(Event::End(BytesEnd::new("checkstyle"))).unwrap();
    writer.into_inner()
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_lcov.info>", args[0]);
        std::process::exit(1);
    }

    let lcov_file_path = &args[1];
    let uncovered_files = parse_lcov(lcov_file_path)?;
    let checkstyle_output = convert_to_checkstyle_format(uncovered_files);
    println!("{}", String::from_utf8(checkstyle_output).unwrap());

    Ok(())
}
