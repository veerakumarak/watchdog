use crate::validations::validate_name;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;
use crate::models::{Channel, ProviderType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChannelResponseDto {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub configuration: Value,
}

impl From<Channel> for ChannelResponseDto {
    fn from(channel: Channel) -> Self {
        Self {
            id: channel.id,
            name: channel.name,
            provider_type: channel.provider_type,
            configuration: channel.configuration,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, PartialEq)]
pub struct ChannelCreateRequest {
    #[validate(custom(function = "validate_name"))]
    pub id: String,
    #[validate(custom(function = "validate_name"))]
    pub name: String,
    pub provider_type: ProviderType,
    pub configuration: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, PartialEq)]
pub struct ChannelUpdateRequest {
    #[validate(custom(function = "validate_name"))]
    pub name: String,
    pub provider_type: ProviderType,
    pub configuration: Value,
}