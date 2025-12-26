use calamine::{Data, RangeDeserializerBuilder, Reader, Xlsx, open_workbook_from_rs};
use database::models::{threat::ThreatCreateDTO, threat_status::ThreatStatus};
use std::io::Cursor;

use crate::utils::excel_date_to_utc::excel_date_to_utc;

pub async fn parse_threat_xlsx_to_threat(bytes: Vec<u8>) -> Result<Vec<ThreatCreateDTO>, String> {
    let cursor = Cursor::new(bytes);
    let mut workbook: Xlsx<_> =
        open_workbook_from_rs(cursor).map_err(|e| format!("Ошибка чтения файла: {}", e))?;

    let mut threats: Vec<ThreatCreateDTO> = Vec::new();

    let range = workbook
        .worksheet_range("Sheet")
        .map_err(|e| format!("Ошибка получения листа Sheet: {}", e))?;

    let mut iter = RangeDeserializerBuilder::new()
        .from_range(&range)
        .map_err(|e| format!("Ошибка чтения workbook: {}", e))?;

    // пропускаем заголовки
    iter.next();
    iter.next();

    for row in iter {
        let row: (
            String,
            String,
            Option<String>,
            String,
            String,
            String,
            String,
            String,
            Data,
            Data,
            String,
            Option<String>,
        ) = match row {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Пропущена строка из-за ошибки: {}", e);
                continue;
            }
        };

        let id_from_source: String = row.0;
        let title: String = row.1;
        let description: Option<String> = row.2;
        let adding_in_source_raw: Data = row.8.clone();
        let last_update_in_source_raw: Data = row.9.clone();
        let status_raw: String = row.10.clone();
        let comment: Option<String> = row.11;

        let adding_in_source = excel_date_to_utc(&adding_in_source_raw);
        let last_update_in_source = excel_date_to_utc(&last_update_in_source_raw);

        let status = ThreatStatus::from_ru_status(&status_raw);

        let source_seq: i32 = id_from_source
            .trim()
            .parse()
            .map_err(|_| "Некорректный числовой идентификатор")?;

        let link = format!(
            "https://bdu.fstec.ru/threat/ubi.{}",
            format!("{:03}", source_seq)
        );

        let threat = ThreatCreateDTO {
            id_from_source,
            title,
            description,
            adding_in_source,
            last_update_in_source,
            status,
            comment,
            link: Some(link),
        };

        threats.push(threat);
    }

    Ok(threats)
}
