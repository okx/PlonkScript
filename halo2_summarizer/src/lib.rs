#![allow(dead_code)]

pub fn trim(mock_prover_output: &String) -> String {
    trim_start(mock_prover_output, 0)
}

pub fn trim_start(mock_prover_output: &String, start: i32) -> String {
    trim_start_end(mock_prover_output, start, start + 1024)
}

enum ReadingStatus {
    Skipping,
    Inside(String, String),
}

const SPACE4: &str = "    ";
const SPACE8: &str = "        ";
const SPACE12: &str = "            ";
const SPACE16: &str = "                ";

pub fn trim_start_end(mock_prover_output: &String, start: i32, end: i32) -> String {
    let mut status = ReadingStatus::Skipping;
    let mut inside_strs = Vec::<&str>::new();
    let mut s = "".to_string();
    let mut after_first_line = false;
    let mut meta_inserted = false;

    for line in mock_prover_output.lines() {
        match status {
            ReadingStatus::Skipping => {
                if after_first_line && !meta_inserted {
                    s.push_str(format!("    start: {start},\n").as_str());
                    s.push_str(format!("    end: {end},\n").as_str());
                    meta_inserted = true;
                }

                match line {
                    "            cells: [" => {
                        status = ReadingStatus::Inside("cells".to_string(), format!("{SPACE12}],"));
                        inside_strs = Vec::<&str>::new();
                    }
                    "    fixed: [" | "    advice: [" | "    instance: [" | "    selectors: [" => {
                        status =
                            ReadingStatus::Inside("columns".to_string(), format!("{SPACE4}],"));
                        inside_strs = Vec::<&str>::new();
                    }
                    "        mapping: [" => {
                        status =
                            ReadingStatus::Inside("mapping".to_string(), format!("{SPACE8}],"));
                        inside_strs = Vec::<&str>::new();
                    }
                    "        aux: [" | "        sizes: [" => {
                        status = ReadingStatus::Inside("remove".to_string(), format!("{SPACE8}],"));
                        inside_strs = Vec::<&str>::new();
                    }
                    _ => (),
                }

                s.push_str(line);
                s.push_str("\n");
                after_first_line = true;
            }
            ReadingStatus::Inside(ref field, ref end_line) => {
                if line == *end_line {
                    match field.as_str() {
                        "cells" => {
                            let mut tmp = Vec::<&str>::new();
                            std::mem::swap(&mut tmp, &mut inside_strs);

                            for l in trim_cells(tmp, start, end) {
                                s.push_str(l);
                                s.push_str("\n");
                            }
                        }
                        "columns" => {
                            let mut tmp = Vec::<&str>::new();
                            std::mem::swap(&mut tmp, &mut inside_strs);

                            for l in trim_columns(tmp, start, end) {
                                s.push_str(l);
                                s.push_str("\n");
                            }
                        }
                        "mapping" => {
                            let mut tmp = Vec::<&str>::new();
                            std::mem::swap(&mut tmp, &mut inside_strs);

                            for l in trim_mappings(tmp, start, end) {
                                s.push_str(l);
                                s.push_str("\n");
                            }
                        }
                        "remove" => (),
                        _ => (),
                    }
                    s.push_str(line);
                    s.push_str("\n");
                    status = ReadingStatus::Skipping;
                    continue;
                }

                inside_strs.push(line);
            }
        }
    }

    s
}

fn trim_cells(lines: Vec<&str>, start: i32, end: i32) -> Vec<&str> {
    let mut arr = Vec::<&str>::new();
    let mut pos = 0;

    while pos < lines.len() {
        let row = lines[pos + 5]
            .trim_matches(',')
            .trim()
            .parse::<i32>()
            .unwrap();

        // println!("{}, {}", row, pos);

        if row >= start && row < end {
            for i in 0..7 {
                arr.push(lines[pos + i]);
            }
        }
        pos += 7;
    }

    arr
}

fn trim_columns(lines: Vec<&str>, start: i32, end: i32) -> Vec<&str> {
    let mut arr = Vec::<&str>::new();
    let mut col = Vec::<&str>::new();

    for line in lines {
        if line == "        [" {
            continue;
        }
        if line == "        ]," {
            let mut tmp = Vec::<&str>::new();
            std::mem::swap(&mut tmp, &mut col);
            arr.push("        [");
            for c in trim_column(tmp, start, end) {
                arr.push(c);
            }

            arr.push("        ],");
            continue;
        }

        col.push(line);
    }

    arr
}

fn trim_column(lines: Vec<&str>, start: i32, end: i32) -> Vec<&str> {
    let mut arr = Vec::<&str>::new();
    let mut pos = 0;
    let mut line_num = 0;

    while pos < lines.len() && line_num < end {
        let count = if lines[pos].contains(",") { 1 } else { 3 };
        if line_num >= start {
            for i in 0..count {
                arr.push(lines[pos + i]);
            }
        }

        pos += count;
        line_num += 1;
    }

    arr
}

fn trim_mappings(lines: Vec<&str>, start: i32, end: i32) -> Vec<&str> {
    let mut arr = Vec::<&str>::new();
    let mut col = Vec::<&str>::new();

    for line in lines {
        if line == "            [" {
            continue;
        }
        if line == "            ]," {
            let mut tmp = Vec::<&str>::new();
            std::mem::swap(&mut tmp, &mut col);
            arr.push("            [");
            for c in trim_mapping(tmp, start, end) {
                arr.push(c);
            }

            arr.push("            ],");
            continue;
        }

        col.push(line);
    }

    arr
}

fn trim_mapping(lines: Vec<&str>, start: i32, end: i32) -> Vec<&str> {
    let mut arr = Vec::<&str>::new();
    let mut pos = 0;
    let mut line_num = 0;
    let count = 4;

    while pos < lines.len() && line_num < end {
        if line_num >= start {
            for i in 0..count {
                arr.push(lines[pos + i]);
            }
        }

        pos += count;
        line_num += 1;
    }

    arr
}
