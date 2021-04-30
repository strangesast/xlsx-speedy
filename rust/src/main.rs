use std::env;
use std::path::Path;
use std::time::Instant;

use calamine::{open_workbook, Reader, Xlsx};

const HEADER: [(&str, &str); 18] = [
    ("CUSTOMER", "customer"),
    ("TOP ASSY\r\nDESCRIPTION", "top_assembly_description"),
    ("TOP ASSY \r\nP/N", "top_assembly_part_number"),
    ("SALES \r\nORDER #", "order_number"),
    ("QTY", "qty"),
    ("DUE \r\nDATE", "due_date"),
    (".", ""),
    ("PART\r\nDESCRIPTION", "part_description"),
    ("PART NUMBER", "part_number"),
    ("MATERIALS", "materials"),
    ("SAW", "status_saw"),
    ("LATHE", "status_lathe"),
    ("MILL", "status_mill"),
    ("WELD", "status_weld"),
    ("Outside\r\nService", "status_outside_service"),
    ("PACK / ASSY", "status_assembly"),
    ("LOCATION", "location"),
    ("NOTES", "notes"),
];

const EMPTY_THRESHOLD: usize = 4;

fn func(filepath: &Path, sheet_name: &str) {
    let mut excel: Xlsx<_> = open_workbook(filepath).unwrap();
    // let mut lastcell = String::new();

    // let mut count = 0;
    // let mut s: String = String::from("");
    let mut values: Vec<Vec<(&str, String, (usize, usize))>> = vec![];
    if let Some(Ok(r)) = excel.worksheet_range(&sheet_name) {
        let mut rows = r.rows().enumerate().skip(2);

        // check for a valid header
        assert!(
            rows.next()
                .unwrap()
                .1
                .iter()
                .map(|c| c.to_string())
                .zip(HEADER.iter().map(|p| p.0))
                .all(|(a, b)| a == b),
            "invalid header!"
        );

        let header_keys = HEADER.iter().map(|p| p.1);

        let mut empty_cnt = 0;
        'outer: for (j, row) in rows {
            let first_cell = row[0].to_string();
            if first_cell == "" {
                empty_cnt += 1;
                if empty_cnt >= EMPTY_THRESHOLD {
                    break 'outer;
                }
                continue;
            } else {
                empty_cnt = 0;
            }

            values.push(
                row.iter()
                    .enumerate()
                    .zip(header_keys.clone())
                    .filter(|(_, h)| *h != "")
                    .map(|((i, cell), h)| (h, cell.to_string(), (i, j)))
                    .collect(),
            )
        }

        println!("length: {}", values.len());
    }
}

fn test() {
    let s = env::var("FILEPATH").unwrap_or("../schedule.xlsm".to_string());
    let filepath = Path::new(&s);
    // let filepath = Path::new("../schedule.xlsm");
    let sheet_name = "PRODUCTION SCHEDULE";

    func(filepath, sheet_name);
}

fn timeit(f: &dyn Fn()) -> f64 {
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    return duration.as_nanos() as f64 / 1e9f64;
}

fn main() {
    // let s = env::var("FILEPATH").unwrap_or("../schedule.xlsm".to_string());
    // let filepath = Path::new(&s);
    // // let filepath = Path::new("../schedule.xlsm");
    // let sheet_name = "PRODUCTION SCHEDULE";

    println!("{}", timeit(&test));
}
