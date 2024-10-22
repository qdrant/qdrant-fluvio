# Fluvio Connector for Qdrant

Connector to stream records from [Fluvio](https://www.fluvio.io/) topics to a Qdrant collection.

## Downloading the connector

Run the following commands after [setting up Fluvio](https://www.fluvio.io/docs/fluvio/quickstart).

```
cdk hub download qdrant/qdrant-sink@0.1.0
```

## Example Config

> _config.yml_

```yaml
apiVersion: 0.1.0
meta:
  version: 0.1.1
  name: my-qdrant-connector
  type: qdrant-sink
  topic: some-topic
  secrets:
    - name: QDRANT_API_KEY

qdrant:
  url: https://xyz-example.eu-central.aws.cloud.qdrant.io:6334
  api_key: "${{ secrets.QDRANT_API_KEY }}"
```

> _secrets.txt_

```text
QDRANT_API_KEY=<SOME_API_KEY>
```

## Running

```
cdk deploy start --ipkg qdrant-qdrant-sink-0.1.0.ipkg -c config.yml --secrets secrets.txt
```

## Produce Messages

You can now run the following to generate messages to be written into Qdrant.

```
fluvio produce some-topic
```

## Message Formats

This sink connector supports messages with dense/sparse/multi vectors.

_Click each to expand._

<details>
  <summary><b>Unnamed/Default vector</b></summary>

Reference: [Creating a collection with a default vector](https://qdrant.tech/documentation/concepts/collections/#create-a-collection).

```json
{
    "collection_name": "{collection_name}",
    "id": 1,
    "vectors": [
        0.1,
        0.2,
        0.3,
        0.4,
        0.5,
        0.6,
        0.7,
        0.8
    ],
    "payload": {
        "name": "fluvio",
        "description": "Solution for distributed stream processing",
        "url": "https://www.fluvio.io/"
    }
}
```

</details>

<details>
  <summary><b>Named multiple vectors</b></summary>

Reference: [Creating a collection with multiple vectors](https://qdrant.tech/documentation/concepts/collections/#collection-with-multiple-vectors).

```json
{
    "collection_name": "{collection_name}",
    "id": 1,
    "vectors": {
        "some-dense": [
            0.1,
            0.2,
            0.3,
            0.4,
            0.5,
            0.6,
            0.7,
            0.8
        ],
        "some-other-dense": [
            0.1,
            0.2,
            0.3,
            0.4,
            0.5,
            0.6,
            0.7,
            0.8
        ]
    },
    "payload": {
        "name": "fluvio",
        "description": "Solution for distributed stream processing",
        "url": "https://www.fluvio.io/"
    }
}
```

</details>

<details>
  <summary><b>Sparse vectors</b></summary>

Reference: [Creating a collection with sparse vectors](https://qdrant.tech/documentation/concepts/collections/#collection-with-sparse-vectors).

```json
{
    "collection_name": "{collection_name}",
    "id": 1,
    "vectors": {
        "some-sparse": {
            "indices": [
                0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9
            ],
            "values": [
                0.1,
                0.2,
                0.3,
                0.4,
                0.5,
                0.6,
                0.7,
                0.8,
                0.9,
                1.0
            ]
        }
    },
    "payload": {
        "name": "fluvio",
        "description": "Solution for distributed stream processing",
        "url": "https://www.fluvio.io/"
    }
}
```

</details>

<details>
  <summary><b>Multi-vector</b></summary>

```json
{
    "collection_name": "{collection_name}",
    "id": 1,
    "vectors": {
        "some-multi": [
            [
                0.1,
                0.2,
                0.3,
                0.4,
                0.5,
                0.6,
                0.7,
                0.8,
                0.9,
                1.0
            ],
            [
                1.0,
                0.9,
                0.8,
                0.5,
                0.4,
                0.8,
                0.6,
                0.4,
                0.2,
                0.1
            ]
        ]
    },
    "payload": {
        "name": "fluvio",
        "description": "Solution for distributed stream processing",
        "url": "https://www.fluvio.io/"
    }
}
```

</details>

<details>
  <summary><b>Combination of named dense and sparse vectors</b></summary>

Reference:

- [Creating a collection with multiple vectors](https://qdrant.tech/documentation/concepts/collections/#collection-with-multiple-vectors).

- [Creating a collection with sparse vectors](https://qdrant.tech/documentation/concepts/collections/#collection-with-sparse-vectors).

```json
{
    "collection_name": "{collection_name}",
    "id": "a10435b5-2a58-427a-a3a0-a5d845b147b7",
    "vectors": {
        "some-other-dense": [
            0.1,
            0.2,
            0.3,
            0.4,
            0.5,
            0.6,
            0.7,
            0.8
        ],
        "some-sparse": {
            "indices": [
                0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9
            ],
            "values": [
                0.1,
                0.2,
                0.3,
                0.4,
                0.5,
                0.6,
                0.7,
                0.8,
                0.9,
                1.0
            ]
        }
    },
    "payload": {
        "name": "fluvio",
        "description": "Solution for distributed stream processing",
        "url": "https://www.fluvio.io/"
    }
}
```

</details>

## LICENSE

Apache 2.0 © [2024](https://github.com/qdrant/qdrant-fluvio/blob/master/LICENSE)
