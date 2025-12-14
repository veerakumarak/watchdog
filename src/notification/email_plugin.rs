use async_trait::async_trait;
use lettre::Message;
use lettre::transport::smtp::authentication::Credentials;
use serde_json::Value;
use tracing::info;
use crate::errors::AppError;
use crate::models::ProviderType;
use crate::models::ProviderType::EmailSmtp;
use crate::notification::core::{AlertEvent};
use crate::notification::plugin_registry::NotificationPlugin;

pub struct EmailPlugin;

#[async_trait]
impl NotificationPlugin for EmailPlugin {
    fn provider_type(&self) -> ProviderType {
        EmailSmtp
    }

    fn validate_config(&self, config: &Value) -> Result<(), AppError> {
        if config.get("smtp_host").is_some()
            && config.get("smtp_port").is_some()
            && config.get("to_addresses").is_some_and(|v| v.is_array())
        {
            Ok(())
        } else {
            Err(AppError::BadRequest(
                "Missing smtp_host, smtp_port, or to_addresses array".into(),
            ))
        }
    }

    async fn send(&self, alert: &AlertEvent, config: &Value) -> Result<(), AppError> {
        let host = config["smtp_host"].as_str().unwrap_or("localhost");
        let to = config["to_addresses"].as_array().unwrap();
        info!(
            "[Email Plugin] Connecting to SMTP server at {}. Sending email to {:?}.\n\tSubject: Alert {}\n\tBody: {}",
            host, to, alert.id, alert.message
        );


        // let email = Message::builder()
        //     .from("Sender <sender@gmail.com>".parse().unwrap())
        //     .to("Receiver <receiver@gmail.com>".parse().unwrap())
        //     .subject("Sending email with Rust")
        //     .body(String::from("This is my first email"))
        //     .unwrap();
        //
        // let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());
        //
        // // Open a remote connection to gmail
        // let mailer = SmtpTransport::relay("smtp.gmail.com")
        //     .unwrap()
        //     .credentials(creds)
        //     .build();
        //
        // // Send the email
        // match mailer.send(&email) {
        //     Ok(_) => println!("Email sent successfully!"),
        //     Err(e) => panic!("Could not send email: {:?}", e),
        // }

        // Simulate network I/O delay
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        Ok(())
    }
}
