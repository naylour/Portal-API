use chrono::{DateTime, NaiveDate, TimeZone, Utc};

pub fn naive_date_to_utc(date_str: &str) -> Result<DateTime<Utc>, String> {
    let naive_date = NaiveDate::parse_from_str(date_str, "%d.%m.%Y")
        .map_err(|e| format!("Ошибка парсинга даты: {}", e))?;

    let naive_datetime = naive_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| "Некорректное время при конвертации даты".to_string())?;

    Ok(Utc.from_utc_datetime(&naive_datetime))
}
