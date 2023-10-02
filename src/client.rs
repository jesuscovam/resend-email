use crate::error::Error;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.resend.com/emails";

#[derive(Debug, Deserialize)]
pub struct Email {
    pub id: String,
}
pub struct ResendClient<'a> {
    auth_token: &'a str,
}

impl<'a> ResendClient<'a> {
    pub fn new(auth_token: &'a str) -> Self {
        ResendClient { auth_token }
    }

    pub async fn send(&self, mail: &impl Serialize) -> Result<Email, Error> {
        let resp = reqwest::Client::new()
            .post(API_URL)
            .bearer_auth(self.auth_token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(mail)
            .send()
            .await
            .unwrap();

        match resp.status().is_success() {
            true => {
                let email: Email = resp.json().await.unwrap();
                Ok(email)
            }
            false => Err(Error::ResendError(resp.text().await.unwrap())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::mail::{Attachment, MailHtml, MailText};
    use super::ResendClient;
    use dotenv::dotenv;
    use std::env;
    use tokio;

    fn get_token() -> String {
        dotenv().ok();
        env::var("AUTH_TOKEN").unwrap()
    }

    #[tokio::test]
    async fn send_mail_text() {
        let mail = MailText {
            from: "Acme <onboarding@resend.dev>",
            to: vec!["jesuscovam@gmail.com"],
            subject: "a",
            text: "a",
            attachments: None,
        };

        let auth_token = get_token();

        let client = ResendClient::new(&auth_token);
        let resp = client.send(&mail).await.unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn send_mail_html() {
        let mail = MailHtml {
            from: "Acme <onboarding@resend.dev>",
            to: vec!["jesuscovam@gmail.com"],
            subject: "a",
            html: "<p>hola</p>",
            attachments: None,
        };

        let auth_token = get_token();

        let client = ResendClient::new(&auth_token);
        let resp = client.send(&mail).await.unwrap();
        println!("{:?}", resp);
    }

    #[test]
    fn format() {
        let mail = MailText {
            from: "a",
            to: vec!["a"],
            subject: "a",
            text: "a",
            attachments: None,
        };

        assert_eq!(
            serde_json::to_string(&mail).unwrap(),
            r#"{"from":"a","to":["a"],"subject":"a","text":"a","attachments":null}"#
        );
    }

    #[test]
    fn format_attachment() {
        let image = Attachment {
            content: vec![],
            filename: "image.png",
        };

        let mail = MailText {
            from: "a",
            to: vec!["a"],
            subject: "a",
            text: "a",
            attachments: Some(vec![image]),
        };

        assert_eq!(
            serde_json::to_string(&mail).unwrap(),
            r#"{"from":"a","to":["a"],"subject":"a","text":"a","attachments":[{"content":[],"filename":"image.png"}]}"#
        );
    }
}
