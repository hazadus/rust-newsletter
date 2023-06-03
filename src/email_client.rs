use crate::domain::SubscriberEmail;
use reqwest::Client;

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        _recipient: SubscriberEmail,
        _subject: &str,
        _html_content: &str,
        _text_content: &str,
    ) -> Result<(), String> {
        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: _recipient.as_ref().to_owned(),
            subject: _subject.to_owned(),
            html_body: _html_content.to_owned(),
            text_body: _text_content.to_owned(),
        };

        let builder = self.http_client.post(&url).json(&request_body);

        Ok(())
    }
}

#[derive(serde::Serialize)]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        // Spin up the server on a background thread, ready to listen for incoming requests.
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            // Use our mock HTTP server as base
            mock_server.uri(),
            sender,
        );

        Mock::given(any()) // on any request...
            .respond_with(ResponseTemplate::new(200)) // respond with `200 OK`
            .expect(1) // NB: Not the same as `expect()` on `Result`
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        // Assert
    }
}
