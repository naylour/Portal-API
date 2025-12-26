use reqwest::{
    Client,
    header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT},
};

pub async fn download_xlsx(url: &str) -> Result<Vec<u8>, String> {
    let mut headers = HeaderMap::new();

    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/html, */*"),
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7"),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Ошибка создания клиента: {:?}", e))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| format!("Ошибка при отправке запроса: {:?}", error))?;

    if !response.status().is_success() {
        return Err(format!("Ошибка скачивания: status {}", response.status()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|error| format!("Ошибка чтения байтов: {:?}", error))?;

    Ok(bytes.to_vec())
}
