use crate::validations::validate_email_list;
use async_trait::async_trait;
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use serde_json::Value;
use tracing::info;
use validator::Validate;
use crate::errors::AppError;
use crate::models::{JobConfig, JobRun, ProviderType};
use crate::models::ProviderType::EmailSmtp;
use crate::notification::core::{AlertEvent, AlertType};
use crate::notification::plugin_registry::NotificationPlugin;

pub struct EmailPlugin;

#[derive(Debug, Validate, Deserialize)]
struct Config {
    #[validate(length(min = 4, message = "host must be at least 4 characters long"))]
    pub smtp_host: String,
    pub smtp_port: u16,
    #[validate(length(min = 1, message = "vec must contain at least one address"))]
    #[validate(custom(function = "validate_email_list"))]
    pub to_addresses: Vec<String>,
    // pub from_address: String,
    // pub subject: String,
}
#[async_trait]
impl NotificationPlugin for EmailPlugin {
    fn provider_type(&self) -> ProviderType {
        EmailSmtp
    }

    fn validate_config(&self, config: &Value) -> Result<(), AppError> {
        let _config: Config = serde_json::from_value(config.clone()).map_err(|e| {
            AppError::BadRequest(format!("invalid config provided {}", e))
        })?;
        _config.validate()?;

        print!("{:?}", config);

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


        // Simulate network I/O delay
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        Ok(())
    }

    async fn send2(&self, job_config: &JobConfig, job_run: &JobRun, config: &Value, alert_type: AlertType) -> Result<(), AppError> {
        let host = config["smtp_host"].as_str().unwrap_or("localhost");
        let to = config["to_addresses"].as_array().unwrap();
        // info!(
        //     "[Email Plugin] Connecting to SMTP server at {}. Sending email to {:?}.\n\tSubject: Alert {}\n\tBody: {}",
        //     host, to, alert.id, alert.message
        // );


        let (subject, body) = render_message(alert_type, job_config, job_run,"");

        let email = Message::builder()
            .from("Sender <sender@gmail.com>".parse().unwrap())
            .to("Receiver <receiver@gmail.com>".parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();

        let creds = Credentials::new("username".to_string(), "password".to_string());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }

        Ok(())
    }
}

fn render_message(alert_type: AlertType, job_config: &JobConfig, job_run: &JobRun, stage: &str) -> (String, String) {
    match alert_type {
        // AlertType::Error => (
        //     "[{app_name}]: [{job_name}] [{stage}]: Runtime Error Occurred",
        //     "Watchdog Error Alert\nApplication: {application}\nJob Name: {dag} \nStage Name: {stage}\nMessage: {message}"
        // ),
        AlertType::Timeout => (
            "[{app_name}]: [{job_name}] Job Timeout Alert from Watchdog".replace("{app_name}", &job_config.app_name).replace("{job_name}", &job_config.job_name),
            "Airflow Stage Timeout Alert\nApplication: {app_name}\nJob Name: {job_name} \nStage Name: {stage}\nRun Id: {run_id}"
                .replace("{app_name}", &job_config.app_name)
                .replace("{job_name}", &job_config.job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &job_run.id.to_string())

        ),
        AlertType::Failed => (
            "[{app_name}]: [{job_name}] Job Failed Alert from Watchdog".replace("{app_name}", &job_config.app_name).replace("{job_name}", &job_config.job_name),
            "Airflow Stage Failed Alert\nApplication: {app_name}\nJob Name: {job_name} \nStage Name: {stage}\nEvent Id: {event}\nMessage: {message}"
                .replace("{app_name}", &job_config.app_name)
                .replace("{job_name}", &job_config.job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &job_run.id.to_string())
        ),
    }
}

/*
send_failed,
send_error,
send_timeout

only variable is vec<channel_names>

each plugin impl: use the template to render the message

each channel use the message and to address to send it

 */