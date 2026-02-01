use crate::validations::{validate_name, validate_config_json};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::{Channel, ProviderType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChannelResponseDto {
    pub name: String,
    pub provider_type: ProviderType,
    pub configuration: String,
}

impl From<Channel> for ChannelResponseDto {
    fn from(channel: Channel) -> Self {
        Self {
            name: channel.name,
            provider_type: channel.provider_type,
            configuration: channel.configuration.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, PartialEq)]
pub struct ChannelCreateRequest {
    #[validate(custom(function = "validate_name"))]
    pub name: String,
    pub provider_type: ProviderType,
    #[validate(custom(function = "validate_config_json"))]
    pub configuration: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, PartialEq)]
pub struct ChannelUpdateRequest {
    pub provider_type: ProviderType,
    #[validate(custom(function = "validate_config_json"))]
    pub configuration: String,
}