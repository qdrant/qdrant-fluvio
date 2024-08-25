mod config;
mod sink;
use config::QdrantConfig;
use sink::QdrantSink;

use futures::StreamExt;

use fluvio_connector_common::{connector, consumer::ConsumerStream, tracing, Result, Sink};
use futures::SinkExt;

const SIGNATURES: &str = concat!("Qdrant Sink Connector ", env!("CARGO_PKG_VERSION"));
#[connector(sink)]
async fn start(config: QdrantConfig, mut stream: impl ConsumerStream) -> Result<()> {
    tracing::debug!(?config);

    tracing::info!("Starting {SIGNATURES}");

    let sink = QdrantSink::new(config).await?;

    let mut sink = sink.connect(None).await?;

    while let Some(item) = stream.next().await {
        tracing::info!("Received record in consumer");
        sink.send(item?).await?;
    }
    tracing::info!("Consumer loop finished");

    Ok(())
}
