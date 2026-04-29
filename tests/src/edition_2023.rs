use alloc::vec;

use prost::Message;

include!(concat!(env!("OUT_DIR"), "/edition_2023.rs"));

#[test]
fn test_repeated_edition_2023_fields_always_use_packed_encoding() {
    // This test also implicitly tests that the deprecated annotation propagates
    // correctly, and that the new features.repeated_field_encoding syntax does
    // not affect the packed field option that Prost sees. This means this test
    // actually asserts that this fork of Prost INCORRECTLY uses packed encoding
    // when encoding is specified as EXPANDED in edition 2023 syntax.

    #[expect(deprecated)]
    let message = Test {
        current: None,
        outdated: None,
        current_packed: vec![1, 2, 3],
        deprecated_packed: vec![2, 3, 4],
        current_feature_packed: vec![3, 4, 5],
        current_feature_expanded: vec![4, 5, 6],
        deprecated_feature_packed: vec![5, 6, 7],
        deprecated_feature_expanded: vec![6, 7, 8],
    };

    const LENGTH_DELIMITED_WIRE_TYPE: u8 = 2;
    const FIELD_3_TAG: u8 = 3 << 3 | LENGTH_DELIMITED_WIRE_TYPE;
    const FIELD_4_TAG: u8 = 4 << 3 | LENGTH_DELIMITED_WIRE_TYPE;
    const FIELD_5_TAG: u8 = 5 << 3 | LENGTH_DELIMITED_WIRE_TYPE;
    const FIELD_6_TAG: u8 = 6 << 3 | LENGTH_DELIMITED_WIRE_TYPE;
    const FIELD_7_TAG: u8 = 7 << 3 | LENGTH_DELIMITED_WIRE_TYPE;
    const FIELD_8_TAG: u8 = 8 << 3 | LENGTH_DELIMITED_WIRE_TYPE;

    let expected_encoded = [
        FIELD_3_TAG, 3, 1, 2, 3, // current_packed
        FIELD_4_TAG, 3, 2, 3, 4, // deprecated_packed
        FIELD_5_TAG, 3, 3, 4, 5, // current_feature_packed
        FIELD_6_TAG, 3, 4, 5, 6, // current_feature_expanded
        FIELD_7_TAG, 3, 5, 6, 7, // deprecated_feature_packed
        FIELD_8_TAG, 3, 6, 7, 8, // deprecated_feature_expanded
    ];

    assert_eq!(message.encode_to_vec(), expected_encoded);
}
