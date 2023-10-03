# resend-email

Library for sending emails with [Resend](https://resend.com)

Create your emails `mail::MailText` and `mail::MailHtml` for each case

Send files with `mail::Attachment`

Send emails with `client::ResendClient`

Read the id of your new email from `client::Email`

## Examples
### send text email
```rust
use resend_email::client::ResendClient;
use resend_email::email::MailText;

 async fn send_mail_text() {
        let mail = MailText {
            from: "Acme <onboarding@resend.dev>",
            to: vec!["test_email@gmail.com"],
            subject: "a",
            text: "a",
            attachments: None,
        };

        let client = ResendClient::new(YOUR_RESEND_API_TOKEN);
        let resp = client.send(&mail).await.unwrap();
        println!("{:?}", resp) // client::Email;
    }
```

### send html email
```rust
use resend_email::client::ResendClient;
use resend_email::email::MailHtml;

   async fn send_mail_html() {
        let mail = MailHtml {
            from: "Acme <onboarding@resend.dev>",
            to: vec!["test_email@gmail.com"],
            subject: "a",
            html: "<p>hola</p>",
            attachments: None,
        };

        let client = ResendClient::new(YOUR_RESEND_API_TOKEN);
        let resp = client.send(&mail).await.unwrap();
        println!("{:?}", resp) // client::Email;
    }
```

### send email with attachment
```rust
use resend_email::client::ResendClient;
use resend_email::email::{Attachment, MailHtml};

   async fn send_mail_attachtment() {
        let image = Attachment {
            content: vec![],
            filename: "image.png",
        };
        
        let mail = MailHtml {
            from: "Acme <onboarding@resend.dev>",
            to: vec!["test_email@gmail.com"],
            subject: "a",
            html: "<p>hola</p>",
            attachments: Some(vec![image]),
        };

        let client = ResendClient::new(YOUR_RESEND_API_TOKEN);
        let resp = client.send(&mail).await.unwrap();
        println!("{:?}", resp) // client::Email;
    }
```
