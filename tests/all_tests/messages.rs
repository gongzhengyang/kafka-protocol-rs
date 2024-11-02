use bytes::BytesMut;
use kafka_protocol::messages::{
    fetch_response::FetchableTopicResponse, produce_response::TopicProduceResponse, FetchResponse,
    ProduceResponse, ResponseKind, TopicName,
};
use kafka_protocol::protocol::StrBytes;

fn expect_encode(response_kind: ResponseKind, version: i16, expect: Vec<u8>) {
    let mut buf = BytesMut::new();
    response_kind.encode(&mut buf, version).unwrap();
    assert_eq!(expect, buf.to_vec())
}

#[test]
fn produce_response_encode() {
    let produce = ProduceResponse::default().with_responses(vec![TopicProduceResponse::default()
        .with_name(TopicName::from(StrBytes::from_static_str("test")))]);

    let kind = ResponseKind::from(produce.clone());
    assert_eq!(ResponseKind::Produce(produce), kind);
    expect_encode(kind, 1, b"\0\0\0\x01\0\x04test\0\0\0\0\0\0\0\0".to_vec());
}

#[test]
fn fetch_response_encode() {
    let fetch = FetchResponse::default()
        .with_responses(vec![FetchableTopicResponse::default()
            .with_topic(TopicName::from(StrBytes::from_static_str("test")))])
        .with_session_id(1)
        .with_throttle_time_ms(2);

    let kind = ResponseKind::from(fetch.clone());
    assert_eq!(ResponseKind::Fetch(fetch), kind);
    expect_encode(
        kind,
        12,
        b"\0\0\0\x02\0\0\0\0\0\x01\x02\x05test\x01\0\0".to_vec(),
    );
}
