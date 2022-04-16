use core::iter::Iterator;

use chrono::FixedOffset;
use chrono_tz::Tz;

use super::record::Record;

//
pub fn zone_names_from_gmt_offset<'a, I>(gmt_offset: FixedOffset, iter: I) -> Vec<(Tz, &'a str)>
where
    I: Iterator<Item = &'a Record>,
{
    use ip2location_country_information::RECORDS_COUNTRY_CODE_MAP;

    let mut records = iter
        .filter(|x| x.gmt_offset == gmt_offset)
        .filter_map(|x| {
            RECORDS_COUNTRY_CODE_MAP
                .get(&x.country_code)
                .map(|country_info| {
                    (
                        x.zone_name,
                        x.country_code.as_ref(),
                        country_info.population,
                    )
                })
        })
        .collect::<Vec<_>>();

    // max -> min
    records.sort_by(|a, b| b.2.cmp(&a.2));

    records
        .into_iter()
        .map(|(zone_name, country_code, _)| (zone_name, country_code))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::time_zone::csv_format::Records;

    #[test]
    fn test_from_csv() {
        let csv = include_str!("../../data/time_zone.csv");
        let records = Records::from_csv(csv.as_bytes()).unwrap();

        let zone_names = zone_names_from_gmt_offset(FixedOffset::east(8 * 3600), records.iter());

        println!("{:?}", zone_names);
        assert_eq!(zone_names[0], (Tz::Asia__Shanghai, "CN"));
    }
}
