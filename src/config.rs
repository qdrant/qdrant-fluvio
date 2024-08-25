use fluvio_connector_common::connector;
use fluvio_connector_common::secret::SecretString;

#[connector(config, name = "qdrant")]
#[derive(Debug)]
pub(crate) struct QdrantConfig {
    pub(crate) url: String,
    pub(crate) api_key: Option<SecretString>,
}
