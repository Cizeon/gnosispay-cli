use crate::prelude::*;
use reqwest;

pub struct PushOverNotify {
    url: String,
    token: String,
    user: String,
}

impl PushOverNotify {
    pub fn new(token: String, user: String) -> Self {
        Self {
            url: String::from("https://api.pushover.net/1/messages.json"),
            token,
            user,
        }
    }

    /// Send a notification with title/message.
    pub async fn notify(&self, title: String, message: String) -> Result<()> {
        let content = format!(
            "token={}&user={}&title={}&message={}",
            self.token, self.user, title, message
        );

        let client = reqwest::Client::builder().build()?;

        /* Send the request. */
        let response = client
            .post(&self.url)
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(content)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {}
            _other => {
                panic!("[-] Unexpected error: {:?}", response.text().await);
            }
        }

        Ok(())
    }
}
