use crate::validations::validate_email_list;
use async_trait::async_trait;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use serde::Deserialize;
use serde_json::Value;
use tracing::info;
use validator::Validate;
use crate::errors::AppError;
use crate::models::{ProviderType};
use crate::models::ProviderType::EmailSmtp;
use crate::notification::core::{AlertType};
use crate::notification::plugin_registry::NotificationPlugin;

pub struct EmailPlugin;

#[derive(Debug, Validate, Deserialize)]
struct Config {
    #[validate(length(min = 4, message = "host must be at least 4 characters long"))]
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ignore_tls_verification: bool,
    #[validate(length(min = 1, message = "vec must contain at least one address"))]
    #[validate(custom(function = "validate_email_list"))]
    pub to_addresses: Vec<String>,
    #[validate(email)]
    pub from_address: String,
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

        Ok(())
    }


    async fn send(&self, app_name: &String, job_name: &String, run_id_opt: Option<String>, stage_name: &String, message_opt: Option<String>, config: &Value, alert_type: AlertType) -> Result<(), AppError> {
        let _config: Config = serde_json::from_value(config.clone()).map_err(|e| {
            AppError::BadRequest(format!("invalid config provided {}", e))
        })?;

        let mailer = build_transport(&_config)
            .map_err(|e| AppError::InternalError(format!("unable to connect to email server {}", e)))?;

        let (subject, body) = render_message(alert_type, app_name, job_name, run_id_opt,stage_name, message_opt);

        let email = Message::builder()
            .from(_config.from_address.parse().unwrap())
            .to(_config.to_addresses.join(",").parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();

        match mailer.send(&email) {
            Ok(_) => info!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }

        Ok(())
    }
}

fn build_transport(_config: &Config) -> Result<SmtpTransport, lettre::transport::smtp::Error> {
    // 1. Choose the builder strategy based on the flag
    let mut builder = if _config.ignore_tls_verification {
        // "Dangerous" mode: Allows port 25 plain text or self-signed certs
        SmtpTransport::builder_dangerous(&_config.host)
    } else {
        // "Relay" mode: Enforces valid public SSL certificates (Gmail, AWS, etc.)
        // Note: .relay() validates the host string immediately, so it returns a Result
        SmtpTransport::relay(_config.host.as_str())?
    };

    // 2. Set the port explicitly
    builder = builder.port(_config.port);

    // 3. Conditionally add credentials
    if let (Some(user), Some(pass)) = (&_config.username, &_config.password) {
        let creds = Credentials::new(user.clone(), pass.clone());
        builder = builder.credentials(creds);
    }

    // 4. Build and return the final transport
    Ok(builder.build())
}

fn render_message(alert_type: AlertType, app_name: &String, job_name: &String, run_id_opt: Option<String>, stage: &str, message_opt: Option<String>) -> (String, String) {
    match alert_type {
        AlertType::Error => (
            "[watchdog]: [{app_name}] [{job_name}] [{stage}]: Runtime Error Occurred"
                .replace("{app_name}", app_name)
                .replace("{job_name}", job_name)
                .replace("{stage}", stage),
            "Watchdog Error Alert\nApplication: {app_name}\nJob Name: {job_name} \nStage Name: {stage}\n*Run Id*: {run_id}\nMessage: {message}"
                .replace("{app_name}", app_name)
                .replace("{job_name}", job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &run_id_opt.unwrap_or("NA".to_string()))
                .replace("{message}", &message_opt.unwrap_or("".to_string()))
        ),
        AlertType::Timeout => (
            "[{app_name}]: [{job_name}] Dag Timeout Alert from Watchdog".replace("{app_name}", app_name).replace("{job_name}", job_name),
            "Airflow Stage Timeout Alert\nApplication: {app_name}\nJob Name: {job_name} \nStage Name: {stage}\nRun Id: {run_id}"
                .replace("{app_name}", app_name)
                .replace("{job_name}", job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &run_id_opt.unwrap_or("NA".to_string()))
        ),
        AlertType::Failed => (
            "[{app_name}]: [{job_name}] Job Failed Alert from Watchdog".replace("{app_name}", app_name).replace("{job_name}", job_name),
            "Airflow Stage Failed Alert\nApplication: {app_name}\nJob Name: {job_name} \nStage Name: {stage}\nEvent Id: {event}\nMessage: {message}"
                .replace("{app_name}", app_name)
                .replace("{job_name}", job_name)
                .replace("{stage}", stage)
                .replace("{run_id}", &run_id_opt.unwrap_or("NA".to_string()))
                .replace("{message}", &message_opt.unwrap_or("".to_string()))
        ),
    }
}
