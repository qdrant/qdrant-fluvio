# qdrant-connector

Connector to stream records from [Fluvio](https://www.fluvio.io/) topics to a Qdrant collection.

## Example Config

```yaml
apiVersion: 0.1.0
meta:
  version: 0.1.0
  name: my-qdrant-connector
  type: qdrant-sink
  topic: some-topic-name
  secrets:
    - name: QDRANT_API_KEY

qdrant:
  url: https://xyz-example.eu-central.aws.cloud.qdrant.io:6334
  api_key: "${{ secrets.QDRANT_API_KEY }}"
```

## Example records

```json
{
    "collection_name": "fluvio",
    "id": "f0ed0746-c140-4014-9dee-75f906cd0ad9",
    "vectors": {
        "sparse": {
            "indices": [432, 5235, 523532],
            "values": [0.5235, 0.2423, 0.3]
        },
        "multi": [[0.5235, 0.2, 0.3, 0.325], [0.65643, 0.643, 0.6364, 0.9979]],
        "dense": [0.5235, 0.2, 0.3, 0.325]
    },
    "payload": {
        "some_key": false
    }
}
```

```json
{
    "collection_name": "fluvio",
    "id": 32,
    "vectors": [0.5235, 0.2, 0.3, 0.325],
    "payload": {
        "some_other_date": 3223,
    }
}
```
