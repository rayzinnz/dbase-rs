// cargo run --features "yore"
// https://dbfread.readthedocs.io/en/latest/field_types.html

use std::path::PathBuf;

fn main() {
    let fp = PathBuf::from(r"C:\Users\hrag\Sync\Programming\rust\dbf_reader\dev\R_EUNZAU_EUNZWE_20261013_132749593.dbf");

    // Basic iteration over records
    let mut reader = dbase::Reader::from_path(fp).unwrap();
    for record_result in reader.iter_records() {
        let record = record_result.unwrap();
        for (name, value) in record {
            println!("Field: {} -> Value: {:?}", name, value);
        }
    }

}
