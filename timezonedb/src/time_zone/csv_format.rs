use core::{fmt, ops::Deref};
use std::io::Read;

use csv::{Error as CsvError, ReaderBuilder, StringRecord};

use super::record::Record;

//
pub const CSV_HEADER: &[&str] = &[
    "zone_name",
    "country_code",
    "abbreviation",
    "time_start",
    "gmt_offset",
    "dst",
];

//
#[derive(Debug, Clone)]
pub struct Records(pub Vec<Record>);

impl Deref for Records {
    type Target = Vec<Record>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//
impl Records {
    pub fn from_csv<R: Read>(rdr: R) -> Result<Self, RecordsFromCsvError> {
        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(rdr);

        let header = StringRecord::from(CSV_HEADER);

        let mut inner = vec![];

        for record in rdr.records() {
            let record = record.map_err(RecordsFromCsvError::CsvParseFailed)?;
            let row: Record = record
                .deserialize(Some(&header))
                .map_err(RecordsFromCsvError::RecordDeFailed)?;
            inner.push(row);
        }

        Ok(Self(inner))
    }
}

//
#[derive(Debug)]
pub enum RecordsFromCsvError {
    CsvParseFailed(CsvError),
    RecordDeFailed(CsvError),
}

impl fmt::Display for RecordsFromCsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RecordsFromCsvError {}

#[cfg(feature = "_integration_tests")]
#[cfg(test)]
pub static RECORDS: once_cell::sync::Lazy<Records> = once_cell::sync::Lazy::new(|| {
    let csv = include_str!("../../data/time_zone.csv");
    let records = Records::from_csv(csv.as_bytes()).unwrap();
    records
});

#[cfg(test)]
mod tests {

    #[cfg(feature = "_integration_tests")]
    #[test]
    fn test_static() {
        use super::*;

        assert_eq!(RECORDS.iter().len(), 160704);
    }
}
