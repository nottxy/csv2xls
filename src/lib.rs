use anyhow::{anyhow, Result};
use std::ffi::OsStr;
use std::path::Path;
use xlsxwriter::Workbook;

pub fn convert<T, P1, P2>(titles: &[T], csv_files: &[P1], xls: P2) -> Result<()>
where
    T: AsRef<str>,
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    // open workbook
    let xls_path = build_out(xls.as_ref())?;
    let mut workbook = Workbook::new(xls_path);

    // save sheets
    for (index, csv_file) in csv_files.into_iter().enumerate() {
        let csv_path = csv_file.as_ref();
        let title = build_title(index, titles, csv_path);
        save_sheet(&mut workbook, title, csv_path)?;
    }

    // save workbook
    workbook.close().map_err(Into::into)
}

fn save_sheet(workbook: &mut Workbook, title: Option<&str>, csv_path: &Path) -> Result<()> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(csv_path)?;

    let mut sheet = workbook.add_worksheet(title)?;
    for (row_index, row_result) in csv_reader.records().enumerate() {
        let row = row_result?;

        for (column_index, cell_value) in row.iter().enumerate() {
            sheet.write_string(row_index as u32, column_index as u16, cell_value, None)?;
        }
    }

    Ok(())
}

fn build_out(xls: &Path) -> Result<&str> {
    match xls.to_str() {
        Some(xls_path) => Ok(xls_path),
        None => return Err(anyhow!("Invalid xls error: {:?}", xls)),
    }
}

fn build_title<'a, 'b: 'a, T>(index: usize, titles: &'a [T], path: &'b Path) -> Option<&'a str>
where
    T: AsRef<str>,
{
    match titles.get(index) {
        Some(title) => Some(title.as_ref()),
        None => path.file_name().and_then(OsStr::to_str),
    }
}
