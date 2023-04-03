use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::api_models::DeployMessage;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contract {
    //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    pub id: Option<ObjectId>,
    pub code_id: String,
    pub metadata: String,
    pub wasm: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Deployment {
    //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    pub id: Option<ObjectId>,
    pub contract_name: Option<String>,
    pub address: String,
    pub network: String,
    pub code_id: String,
}

impl Deployment {
    pub fn new(deploy_message: &DeployMessage) -> Self {
        Deployment {
            id: None,
            contract_name: deploy_message.contract_name.clone(),
            address: deploy_message.address.clone(),
            network: deploy_message.network.clone(),
            code_id: deploy_message.code_id.clone(),
        }
    }
}
