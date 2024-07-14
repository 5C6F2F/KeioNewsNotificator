use crate::error::NotifyOnLineError;

#[derive(Deserialize)]
pub struct NotifyOnLine {
    line_api_url: String,
    line_api_token: String,
}

impl NotifyOnLine {
    pub fn send(&self, message: String) -> Result<(), NotifyOnLineError> {
        let mut token_str = "Bearer ".to_string();
        token_str.push_str(&self.line_api_token);
        let token = reqwest::header::HeaderValue::from_str(&token_str)?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", token);

        let mut payload = std::collections::HashMap::new();
        payload.insert("message", message);

        let line_api_url = self.line_api_url.clone();

        std::thread::spawn(move || -> Result<(), NotifyOnLineError> {
            let client = reqwest::blocking::Client::new();
            let res = client
                .post(line_api_url)
                .headers(headers)
                .form(&payload)
                .send()?;

            match res.status() {
                reqwest::StatusCode::OK => Ok(()),
                _ => Err(NotifyOnLineError::UnsuccessResponse(res)),
            }
        })
        .join()
        .unwrap()?;

        Ok(())
    }
}
