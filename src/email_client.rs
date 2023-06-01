use crate::domain::SubscriberEmail;

pub struct EmailClient {
    _sender: SubscriberEmail,
}

impl EmailClient {
    pub async fn send_email(
        &self,
        _recipient: SubscriberEmail,
        _subject: &str,
        _html_content: &str,
        _text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
