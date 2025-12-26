use repos::utils::download_xlsx;

#[tokio::main]
async fn main() {
    let url = "https://bdu.fstec.ru/files/documents/thrlist.xlsx";

    match download_xlsx::download_xlsx(url).await {
        Ok(bytes) => {
            println!("Успешно скачано!");
            println!("Размер файла: {} байт", bytes.len());
            println!("Первые 20 байт (hex): {:02x?}", &bytes[..20]);

            // Сохраним на диск для проверки глазами
            std::fs::write("downloaded_test.xlsx", &bytes).unwrap();
            println!("Файл сохранён как downloaded_test.xlsx — откройте в Excel");
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
