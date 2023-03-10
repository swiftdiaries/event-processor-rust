use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlushEventPayload {
    pub bucket_name: String,
    pub topic_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestionEventPayload {
    pub channel: String,
    pub context: Context,
    #[serde(rename = "type")]
    pub type_field: String,
    pub message_id: String,
    pub original_timestamp: String,
    pub anonymous_id: String,
    pub user_id: String,
    pub event: String,
    pub properties: Properties,
    pub integrations: Integrations,
    pub sent_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Context {
    pub app: App,
    pub traits: Traits,
    pub library: Library,
    pub user_agent: String,
    pub locale: String,
    pub os: OS,
    pub screen: Screen,
    pub page: Page,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct App {
    pub build: String,
    pub name: String,
    pub namespace: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Traits {
    pub name: String,
    pub email: String,
    pub plan: String,
    pub company: Company,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OS {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screen {
    pub density: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub path: String,
    pub referrer: String,
    pub search: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "order_ID")]
    pub order_id: String,
    pub category: String,
    pub product_name: String,
    pub price: u64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integrations {
    pub all: bool,
}
