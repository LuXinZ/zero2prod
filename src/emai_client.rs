use reqwest::Client;

use crate::domain::SubscriberEmail;
pub struct EmailClient {
    http_client: Client,
    sender: SubscriberEmail,
    base_url: String,
}
impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }
    // add coTypede here
    pub fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
