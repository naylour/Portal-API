use repos::utils::{
    download_xlsx::download_xlsx, parse_threat_xlsx_to_threat::parse_threat_xlsx_to_threat,
};

#[tokio::main]
async fn main() {
    let url = "https://bdu.fstec.ru/files/documents/thrlist.xlsx";

    match download_xlsx(url).await {
        Ok(bytes) => {
            println!("Успешно скачано!");
            println!("Размер файла: {} байт", bytes.len());

            match parse_threat_xlsx_to_threat(bytes).await {
                Ok(threats) => {
                    println!("Успешная обработка!!!!!");
                    println!("Результат: {:#?}", threats);
                }
                Err(error) => {
                    eprintln!("Ошибка парсинга: {}", error);
                }
            }
        }
        Err(error) => {
            eprintln!("Ошибка загрузки: {}", error);
        }
    }
}
