mod data;
extern crate num_cpus;

use crate::data::model::{FlushEventPayload, IngestionEventPayload};
use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
use fluvio::metadata::topic::TopicSpec;
use fluvio::{Fluvio, RecordKey};
use object_store::gcp::GoogleCloudStorageBuilder;
// use object_store::path::Path;
use tracing::log::error;
use tracing::{debug, info};

const TOPIC_NAME: &str = "rudderstack-events";
// use partition number for consumer
// const PARTITION_NUM: u32 = 0;
const PARTITIONS: u32 = 1;
const REPLICAS: u32 = 1;

// Each request spawns a new thread
// No shared state across threads/requests.
// TODO: Add Header guards to filter out requests from other sources
#[post("/ingestion/webhook")]
async fn ingest_events(payload: web::Json<IngestionEventPayload>) -> Result<HttpResponse, Error> {
    info!("processed and validated incoming JSON payload against schema");
    let payload_json = serde_json::to_string(&payload)?;
    debug!("{:#?}", payload_json);

    let fluvio = Fluvio::connect().await.unwrap();
    let admin = fluvio.admin().await;
    let topic_spec = TopicSpec::new_computed(PARTITIONS, REPLICAS, None);
    let _topic_create = admin
        .create(TOPIC_NAME.to_string(), false, topic_spec)
        .await;

    let producer = fluvio::producer(TOPIC_NAME).await.unwrap();
    producer.send(RecordKey::NULL, payload_json).await.unwrap();
    producer.flush().await.unwrap();

    Ok(HttpResponse::Ok().json(r#"{"topic_create_status": "SUCCESS"}"#))
}

#[post("/write/gcs")]
async fn flush_events(payload: web::Json<FlushEventPayload>) -> Result<HttpResponse, Error> {
    let gcs_bucket_name = payload.bucket_name.clone();
    // let topic_name = payload.topic_name.clone();
    // let prefix: Path = "events".try_into().unwrap();
    info!("Bucket name: {:#?}", gcs_bucket_name);
    match GoogleCloudStorageBuilder::from_env()
        .with_bucket_name(gcs_bucket_name)
        .build()
    {
        Ok(gcs) => {
            debug!("GCS Client: {}", gcs.to_string())
        }
        Err(e) => {
            error!("Erroring creating GCS client: {}", e)
        }
    }
    Ok(HttpResponse::Ok().json(r#"{"topic_flush_status: "SUCCESSS""#))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let num_cpus = num_cpus::get();
    HttpServer::new(|| App::new().service(ingest_events).service(flush_events))
        .bind(("127.0.0.1", 8080))?
        .workers(num_cpus * 2)
        .run()
        .await
}
