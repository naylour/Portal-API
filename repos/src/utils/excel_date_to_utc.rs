use calamine::Data;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

pub fn excel_date_to_utc(cell: &Data) -> Option<DateTime<Utc>> {
    match cell {
        // Excel-дата как число
        Data::Float(days) => {
            let base = NaiveDate::from_ymd_opt(1899, 12, 30)?;
            let date = base.checked_add_days(chrono::Days::new(*days as u64))?;
            let naive = date.and_hms_opt(0, 0, 0)?;
            Some(Utc.from_utc_datetime(&naive))
        }

        // Строка "08.02.2019"
        Data::String(s) => {
            let naive = NaiveDate::parse_from_str(s.trim(), "%d.%m.%Y")
                .ok()?
                .and_hms_opt(0, 0, 0)?;

            Some(Utc.from_utc_datetime(&naive))
        }

        _ => None,
    }
}
