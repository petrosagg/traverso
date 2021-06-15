use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum Message {
    Hello(Hello),
    GetPeers,
    Peers(Peers),
    GetObject(GetObject),
    IHaveObject(IHaveObject),
    Object { object: Object },
    GetMempool,
    Mempool(Mempool),
    GetChainTip,
    ChainTip(ChainTip),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hello {
    pub version: String,
    pub agent: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Peers {
    // TODO: use IP types
    pub peers: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetObject {
    #[serde(rename = "objectid")]
    pub object_id: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IHaveObject {
    #[serde(rename = "objectid")]
    pub object_id: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mempool {
    #[serde(rename = "txids")]
    pub tx_ids: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChainTip {
    #[serde(rename = "blockid")]
    pub block_id: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "txids")]
    tx_ids: Vec<String>,
    nonce: String,
    #[serde(rename = "previd")]
    prev_id: String,
    // TODO parse this into a DateTime
    created: String,
    #[serde(rename = "T")]
    t: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello() {
        let data = r#"{
            "type": "hello",
            "version": "0.2.0",
            "agent": "Marabu-Core Client 0.7"
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();
        let expected = Message::Hello(Hello {
            version: "0.2.0".to_string(),
            agent: "Marabu-Core Client 0.7".to_string(),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_peers() {
        let data = r#"{
            "type": "getpeers"
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();

        assert_eq!(actual, Message::GetPeers);
    }

    #[test]
    fn test_peers() {
        let data = r#"{
            "type": "peers",
            "peers": [
              "dionyziz.com:18018",
              "138.197.191.170:18018",
              "[fe80::f03c:91ff:fe2c:5a79]:18018"
             ]
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();
        let expected = Message::Peers(Peers {
            peers: vec![
                "dionyziz.com:18018".to_string(),
                "138.197.191.170:18018".to_string(),
                "[fe80::f03c:91ff:fe2c:5a79]:18018".to_string(),
            ],
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_object() {
        let data = r#"{
            "type": "getobject",
            "objectid": "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8"
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();
        let expected = Message::GetObject(GetObject {
            object_id: "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8"
                .to_string(),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_i_have_object() {
        let data = r#"{
            "type": "ihaveobject",
            "objectid": "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8"
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();
        let expected = Message::IHaveObject(IHaveObject {
            object_id: "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8"
                .to_string(),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_object() {
        let data = r#"{
            "type": "object",
            "object": {
                "type": "block",
                "txids": [
                    "740bcfb434c89abe57bb2bc80290cd5495e87ebf8cd0dadb076bc50453590104"
                ],
                "nonce": "a26d92800cf58e88a5ecf37156c031a4147c2128beeaf1cca2785c93242a4c8b",
                "previd": "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8",
                "created": "1622825642",
                "T": "003a000000000000000000000000000000000000000000000000000000000000"
            }
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();
        let expected = Message::Object {
            object: Object {
                ty: "block".to_string(),
                tx_ids: vec![
                    "740bcfb434c89abe57bb2bc80290cd5495e87ebf8cd0dadb076bc50453590104".to_string(),
                ],
                nonce: "a26d92800cf58e88a5ecf37156c031a4147c2128beeaf1cca2785c93242a4c8b"
                    .to_string(),
                prev_id: "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8"
                    .to_string(),
                created: "1622825642".to_string(),
                t: "003a000000000000000000000000000000000000000000000000000000000000".to_string(),
            },
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_mempool() {
        let data = r#"{
            "type": "getmempool"
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();

        assert_eq!(actual, Message::GetMempool);
    }

    #[test]
    fn test_mempool() {
        let data = r#"{
            "type": "mempool",
            "txids": [
                "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8"
            ]
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();
        let expected = Message::Mempool(Mempool {
            tx_ids: vec![
                "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8".to_string(),
            ],
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_chaintip() {
        let data = r#"{
            "type": "getchaintip"
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();

        assert_eq!(actual, Message::GetChainTip);
    }

    #[test]
    fn test_chaintip() {
        let data = r#"{
            "type": "chaintip",
            "blockid": "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8"
        }"#;

        let actual: Message = serde_json::from_str(data).unwrap();
        let expected = Message::ChainTip(ChainTip {
            block_id: "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8"
                .to_string(),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_reject_garbage() {
        let data = r#"{
            "type": "chaintip",
            "blockid": "0024839ec9632d382486ba7aac7e0bda3b4bda1d4bd79be9ae78e7e1e813ddd8",
            "garbage": 123
        }"#;

        let result: Result<Message, _> = serde_json::from_str(data);

        assert!(result.is_err());
    }
}
