use crate::{
    client::{Client, Response},
    error::NovuError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeType {
    Feed,
    MessageTemplate,
    Layout,
    DefaultLayout,
    NotificationTemplate,
    NotificationGroup,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub _id: String,
    pub _creator_id: String,
    pub _environment_id: String,
    pub _organization_id: String,
    pub _entity_id: String,
    pub _parent_id: String,
    pub enabled: bool,
    pub created_at: String,
    // pub change: String,
    #[serde(rename = "type")]
    pub change_type: ChangeType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangesResponse {
    pub page: u32,
    pub total_count: u32,
    pub page_size: u32,
    pub data: Vec<Change>,
}

pub struct Changes {
    client: Client,
}

// A utility function to generate the query string for multiple parameters
fn generate_query_string(params: &HashMap<&str, Option<impl ToString>>) -> String {
    let mut query_string = String::new();

    for (key, value_option) in params {
        if let Some(value) = value_option {
            if !query_string.is_empty() {
                query_string.push('&');
            }
            query_string.push_str(&format!("{}={}", key, value.to_string()));
        }
    }

    query_string
}

impl Changes {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        promoted: bool,
    ) -> Result<ChangesResponse, NovuError> {
        let mut params = HashMap::new();
        params.insert("page", page.map(|p| p.to_string()));
        params.insert("limit", limit.map(|l| l.to_string()));
        params.insert(
            "promoted",
            Some(match promoted {
                true => "true".to_string(),
                false => "false".to_string(),
            }),
        );

        let result: Response<ChangesResponse> = self
            .client
            .get(format!("/changes/?{}", generate_query_string(&params)))
            .await?;

        match result {
            crate::client::Response::Success(data) => Ok(data.data),
            crate::client::Response::Error(err) => todo!("{:?}", err),
            crate::client::Response::Messages(err) => todo!("{:?}", err),
        }
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_list_changes() {
    let changes = Changes::new(Client::new("", Some("")).unwrap());

    let result = changes.list(None, Some(10), false).await;
    assert!(result.is_err());
}
