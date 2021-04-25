use std::env;
use std::path::Path;
use std::time::Instant;

use calamine::{open_workbook, Reader, Xlsx};

fn func(filepath: &Path, sheet_name: &str) {
    let mut excel: Xlsx<_> = open_workbook(filepath).unwrap();
    // let mut lastcell = String::new();

    // let mut count = 0;
    if let Some(Ok(r)) = excel.worksheet_range(&sheet_name) {
        for row in r.rows() {
            for _ in row.iter() {
                // let ptr = &cell.to_string();
                // lastcell.clone_from(ptr);
                // count += 1;
            }
        }
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
    println!("{}", timeit(&test));
}
