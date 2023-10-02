use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Attachment<'a> {
    pub content: Vec<u8>,
    pub filename: &'a str,
}

#[derive(Debug, Serialize)]
pub struct MailText<'a> {
    pub from: &'a str,
    pub to: Vec<&'a str>,
    pub subject: &'a str,
    pub text: &'a str,
    pub attachments: Option<Vec<Attachment<'a>>>,
}

#[derive(Debug, Serialize)]
pub struct MailHtml<'a> {
    pub from: &'a str,
    pub to: Vec<&'a str>,
    pub subject: &'a str,
    pub html: &'a str,
    pub attachments: Option<Vec<Attachment<'a>>>,
}
