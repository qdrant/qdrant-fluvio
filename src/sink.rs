use anyhow::Result;
use async_trait::async_trait;
use qdrant_client::{
    qdrant::{NamedVectors, PointId, PointStruct, UpsertPointsBuilder, Vector},
    Payload, Qdrant, QdrantBuilder,
};

use fluvio::{consumer::Record, Offset};
use fluvio_connector_common::{tracing, LocalBoxSink, Sink};
use serde_json::Value;

use crate::config::QdrantConfig;

pub(crate) struct QdrantSink {
    client: Qdrant,
}

impl QdrantSink {
    #[allow(unused_variables)]
    pub(crate) async fn new(config: QdrantConfig) -> Result<Self> {
        let mut client_config = QdrantBuilder::from_url(&config.url);

        if let Some(api_key) = config.api_key {
            client_config.set_api_key(&api_key.resolve().unwrap());
        }

        Ok(Self {
            client: client_config.build()?,
        })
    }
}

#[async_trait]
impl Sink<Record> for QdrantSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<Record>> {
        let client = self.client;
        let unfold = futures::sink::unfold(client, |client, record: Record| async move {
            let value = match serde_json::from_slice::<Value>(record.value()) {
                Ok(value) => value,
                Err(e) => {
                    tracing::error!("Failed to parse record value: {}", e);
                    return Err::<Qdrant, anyhow::Error>(e.into());
                }
            };

            let (collection_name, point) = value_to_point(&value)?;
            let response = client
                .upsert_points(UpsertPointsBuilder::new(collection_name, vec![point]).wait(true))
                .await?;

            if let Some(result) = response.result {
                tracing::info!("Upsert status: {:?}", result);
            } else {
                return Err::<Qdrant, anyhow::Error>(anyhow::anyhow!("Failed to upsert point"));
            }

            Ok::<Qdrant, anyhow::Error>(client)
        });

        Ok(Box::pin(unfold))
    }
}

fn value_to_point(json_value: &Value) -> Result<(String, PointStruct), anyhow::Error> {
    let collection_name = if json_value["collection_name"].is_string() {
        json_value["collection_name"].as_str().unwrap().to_string()
    } else {
        return Err(anyhow::anyhow!("Invalid collection_name type"));
    };

    // Qdrant only allows UUID or a positive integer as point ID
    // https://qdrant.tech/documentation/concepts/points/#point-ids
    let id = if json_value["id"].is_string() {
        PointId::from(json_value["id"].as_str().unwrap())
    } else if json_value["id"].is_u64() {
        PointId::from(json_value["id"].as_u64().unwrap())
    } else {
        return Err(anyhow::anyhow!("Invalid point ID type"));
    };

    let mut named_vectors = NamedVectors::default();

    // For collection created with multiple named vectors
    // https://qdrant.tech/documentation/concepts/collections/#collection-with-multiple-vectors
    if let Some(vectors) = json_value["vectors"].as_object() {
        for (key, value) in vectors {
            let vector = if let Some(arr) = value.as_array() {
                if arr.iter().all(|v| v.is_f64()) {
                    Vector::new_dense(arr.iter().map(|v| v.as_f64().unwrap() as f32).collect())
                } else if arr.iter().all(|v| v.is_array()) {
                    let multi = arr
                        .iter()
                        .map(|v| {
                            v.as_array()
                                .unwrap()
                                .iter()
                                .map(|v| v.as_f64().unwrap() as f32)
                                .collect()
                        })
                        .collect();
                    Vector::new_multi(multi)
                } else {
                    return Err(anyhow::anyhow!(
                        "Invalid dense/multi vector type for key {}",
                        key
                    ));
                }
            } else if let Some(map) = value.as_object() {
                if map.contains_key("indices") && map.contains_key("values") {
                    let indices: Vec<u32> = map["indices"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_u64().unwrap() as u32)
                        .collect();
                    let values: Vec<f32> = map["values"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_f64().unwrap() as f32)
                        .collect();
                    Vector::new_sparse(indices, values)
                } else {
                    return Err(anyhow::anyhow!(
                        "Invalid sparse vector type for key {}",
                        key
                    ));
                }
            } else {
                return Err(anyhow::anyhow!("Invalid vector type for key {}", key));
            };
            named_vectors = named_vectors.add_vector(key, vector);
        }
    }
    // For a collection created with a single default vector
    // https://qdrant.tech/documentation/concepts/collections/#create-a-collection
    else if let Some(vectors) = json_value["vectors"].as_array() {
        named_vectors = named_vectors.add_vector(
            "",
            Vector::new_dense(vectors.iter().map(|v| v.as_f64().unwrap() as f32).collect()),
        );
    } else {
        return Err(anyhow::anyhow!("Invalid vectors type"));
    }

    let payload = Payload::try_from(json_value["payload"].clone()).unwrap();

    Ok((
        collection_name,
        PointStruct::new(id, named_vectors, payload),
    ))
}
