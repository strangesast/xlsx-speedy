use std::env;
use std::path::Path;
use std::time::Instant;

use calamine::{open_workbook, Reader, Xlsb};

const HEADER: [&str; 96] = [
    "Autofill Ind",
    "AWP",
    "Bil Patient Pay Amt",
    "Birth Dte",
    "BPL",
    "Cap Applied Amt",
    "Carrier",
    "Carrier Name",
    "Channel",
    "Claim Count",
    "Client Elig Membership ID",
    "Client Membership ID",
    "COB Payment",
    "Coinsurance Amt",
    "Compound Ind",
    "Contract",
    "Copay",
    "Copay BG Difference Paid",
    "Copay Cost Tier Seq",
    "Copayment Amt",
    "Cost Basis Cde",
    "Cost Basis Dsc",
    "Date of Service",
    "DAW Code",
    "Days Supply",
    "DEA Code",
    "Deductible",
    "Dep SSN",
    "Dispensing Fee",
    "Drug Name",
    "Drug Type",
    "Effective Date",
    "End Date",
    "External Group",
    "Fill QTY",
    "First Name",
    "Formulary Ind",
    "GCN",
    "Gender",
    "Generic Mfr Quantity Cde",
    "Generic Name",
    "Gross Cost",
    "Group",
    "Group Name",
    "Health Plan Funded Assist Amt",
    "Incentive Fee",
    "Ingredient Cost",
    "Invoice Date",
    "Last Name",
    "Locator",
    "Mail Service Rx Nbr",
    "Maintenance Drug",
    "Med B Ind",
    "Med D Ind",
    "Member ID",
    "Most Common Indication",
    "NDC",
    "Net Plan Cost",
    "NPI Nbr",
    "OOP Applied Amt",
    "Over Benefit Limit Amt",
    "Pass Through Txt",
    "Patient Brand Selection Amt",
    "Patient Coverage Gap Amt",
    "Patient ID",
    "Patient Level Auth Ind",
    "Patient Network Selection Amt",
    "Patient Non-Pref Brand Amt",
    "Patient Non-Pref Formulary Amt",
    "Patient Sales Tax Amt",
    "Person Nbr",
    "Pharmacy Claim ID",
    "Pharmacy Name",
    "Pharmacy Override Description",
    "Pharmacy RX No",
    "Phcy Override Cde",
    "PLA Auth Nbr",
    "Prescriber First Name",
    "Prescriber Last Name",
    "Prescriber NPI Nbr",
    "Prior Auth Ind",
    "Prior Auth Nbr",
    "Prior Auth Type Cde",
    "Processor Fee Amt",
    "Relshp Cde",
    "RRA Penalty Applied",
    "Rx Refill Nbr",
    "Sales Tax",
    "Specialty Drug",
    "STC",
    "STC Dsc",
    "Telecomm Version Cde",
    "Total Patient Cost",
    "Transaction Type Cde",
    "U&C Cost",
    "ZBL Excess Copay Paid",
];

// get buffer of xlsx,xlsb,xls

// get list of columns

// scan first row
//   headers = input header values
//   header = document header, trimmed
//   i = 0
//   j = 0 # scan input headers
//   headers = [...]
//   header = [...]
//   indexes = [...]
//   while j < headers.length && i < header.length
//     if header[i] != headers[j]
//       i++
//       continue
//     indexes.push(i)
//     i++
//     j++
//
//   if indexes.length != i
//     throw missing header: headers[j]

// scan each row after first
//
//   for row in rows[1:]
//     yield row[i] for i in indexes

fn func(filepath: &Path, headers: Vec<&str>) -> Option<()> {
    let mut excel: Xlsb<_> = open_workbook(filepath).unwrap();

    // let mut count = 0;
    // let mut s: String = String::from("");
    // let mut values: Vec<Vec<(&str, String, (usize, usize))>> = vec![];
    // let values: Vec<Vec<String>> = vec![];
    // if let Some(Ok(r)) = excel.worksheet_range(&sheet_name) {

    // let sheet_names = excel.sheet_names();
    // let sheet_name = sheet_names.get(0)?.clone();
    let sheet_name = "CLAIM DETAIL";

    let mut output = Vec::new();
    if let Some(Ok(r)) = excel.worksheet_range(&sheet_name) {
        println!("size {:?}", r.get_size());
        let mut it = r.rows();
        let header = it.nth(0).unwrap(); // TODO handle error

        let header_values: Vec<String> = header
            .iter()
            .map(|c| c.to_string().to_uppercase().trim().to_string())
            .collect();

        let indexes: Vec<usize> = headers
            .iter()
            .map(|s| {
                header_values
                    .iter()
                    .position(|ss| s.to_uppercase().eq(ss))
                    .unwrap()
            })
            .collect();

        println!("indexes {:?}", indexes);

        let m = headers.len();

        let mut ind: Vec<usize> = indexes.clone();
        ind.sort();

        for row in it {
            let mut out: Vec<String> = Vec::new();
            let mut r = row.iter().enumerate();

            let mut i = 0;
            while i < m {
                if let Some(t) = r.next() {
                    let (jj, c) = t;
                    if jj == ind[i] {
                        out.push(c.to_string());
                        i += 1;
                        continue;
                    }
                } else {
                    break;
                }
            }
            output.push(out);
        }
        println!("count {}", output.len());
    } else {
        println!("no sheet with name: {}", sheet_name);
    }
    None
}

fn test() {
    let s = env::var("FILEPATH").unwrap_or("../lookback-claims.xlsb".to_string());
    let filepath = Path::new(&s);
    // let filepath = Path::new("../schedule.xlsm");
    // let sheet_name = "CLAIM DETAIL";

    // let headers = vec!["one", "two", "four"];
    func(filepath, HEADER.to_vec());
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
