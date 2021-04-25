use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::env;
use std::path::Path;

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

#[pyfunction]
/// Run xlsx test
fn test() {
    let s = env::var("FILEPATH").unwrap_or("../schedule.xlsm".to_string());
    let filepath = Path::new(&s);
    // let filepath = Path::new("../schedule.xlsm");
    let sheet_name = "PRODUCTION SCHEDULE";

    func(filepath, sheet_name);
}

#[pyfunction]
/// Formats the sum of two numbers as string.
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
/// A Python module implemented in Rust.
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(test))?;

    Ok(())
}
