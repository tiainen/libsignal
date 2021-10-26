//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

#![allow(non_snake_case)]
extern crate zkgroup;

use curve25519_dalek::ristretto::RistrettoPoint;
use sha2::Sha256;

#[test]
fn test_lizard() {
    let p = RistrettoPoint::lizard_encode::<Sha256>(&zkgroup::common::constants::TEST_ARRAY_16);
    let data_out = p.lizard_decode::<Sha256>();
    assert!(data_out.unwrap() == zkgroup::common::constants::TEST_ARRAY_16);
}
pub const AUTH_CREDENTIAL_PRESENTATION_RESULT: [u8; zkgroup::AUTH_CREDENTIAL_PRESENTATION_LEN] = [
    0x00, 0x0c, 0xde, 0x97, 0x97, 0x37, 0xed, 0x30, 0xbb, 0xeb, 0x16, 0x36, 0x2e, 0x4e, 0x07, 0x69,
    0x45, 0xce, 0x02, 0x06, 0x9f, 0x72, 0x7b, 0x0e, 0xd4, 0xc3, 0xc3, 0x3c, 0x01, 0x1e, 0x82, 0x54,
    0x6e, 0x1c, 0xdf, 0x08, 0x1f, 0xbd, 0xf3, 0x7c, 0x03, 0xa8, 0x51, 0xad, 0x06, 0x0b, 0xdc, 0xbf,
    0x63, 0x78, 0xcb, 0x4c, 0xb1, 0x6d, 0xc3, 0x15, 0x4d, 0x08, 0xde, 0x54, 0x39, 0xb5, 0x32, 0x32,
    0x03, 0x72, 0x9d, 0x18, 0x41, 0xb5, 0x17, 0x03, 0x3a, 0xf2, 0xfd, 0x17, 0x7d, 0x30, 0x49, 0x1c,
    0x13, 0x8a, 0xe7, 0x23, 0x65, 0x57, 0x34, 0xf6, 0xe5, 0xcc, 0x01, 0xc0, 0x06, 0x96, 0xf4, 0xe9,
    0x20, 0x96, 0xd8, 0xc3, 0x3d, 0xf2, 0x6b, 0xa2, 0xa8, 0x20, 0xd4, 0x2e, 0x97, 0x35, 0xd3, 0x0f,
    0x8e, 0xee, 0xf9, 0x6d, 0x39, 0x90, 0x79, 0x07, 0x3c, 0x09, 0x9f, 0x70, 0x35, 0x52, 0x3b, 0xfe,
    0x71, 0x66, 0x38, 0x65, 0x93, 0x19, 0xd3, 0xc3, 0x6a, 0xd3, 0x4c, 0x00, 0xef, 0x88, 0x50, 0xf6,
    0x63, 0xc4, 0xd9, 0x30, 0x30, 0x23, 0x50, 0x74, 0x31, 0x2a, 0x88, 0x78, 0xb6, 0xa5, 0xc5, 0xdf,
    0x4f, 0xbc, 0x7d, 0x32, 0x93, 0x52, 0x78, 0xbf, 0xa5, 0x99, 0x6b, 0x44, 0xab, 0x75, 0xd6, 0xf0,
    0x6f, 0x4c, 0x30, 0xb9, 0x86, 0x40, 0xad, 0x5d, 0xe7, 0x47, 0x42, 0x65, 0x6c, 0x89, 0x77, 0x56,
    0x7d, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0xe6, 0x9f, 0x82, 0xad, 0x2d, 0xcb,
    0x49, 0x09, 0x65, 0x0a, 0xc6, 0xb2, 0x57, 0x38, 0x41, 0xaf, 0x56, 0x8f, 0xef, 0x82, 0x2b, 0x32,
    0xb4, 0x5f, 0x62, 0x5a, 0x76, 0x46, 0x91, 0xa7, 0x04, 0xd1, 0x1b, 0x6f, 0x38, 0x52, 0x61, 0x46,
    0x81, 0x17, 0xea, 0xd5, 0x7f, 0xa6, 0x23, 0x33, 0x8e, 0x21, 0xc6, 0x6e, 0xd8, 0x46, 0xab, 0x65,
    0x80, 0x9f, 0xca, 0xc1, 0x58, 0x06, 0x6d, 0x8e, 0x0e, 0x44, 0x40, 0x77, 0xb9, 0x95, 0x40, 0xd8,
    0x86, 0xe7, 0xdc, 0x09, 0x55, 0x5d, 0xd6, 0xfa, 0xea, 0x2c, 0xd3, 0x69, 0x7f, 0x1e, 0x08, 0x9f,
    0x82, 0xd5, 0x4e, 0x5d, 0x0f, 0xe4, 0xa1, 0x85, 0x00, 0x8b, 0x5c, 0xbc, 0x39, 0x79, 0x39, 0x1a,
    0xd7, 0x16, 0x86, 0xbc, 0x03, 0xbe, 0x7b, 0x00, 0xea, 0x7e, 0x42, 0xc0, 0x8d, 0x9f, 0x1d, 0x75,
    0xc3, 0xa5, 0x6c, 0x27, 0xae, 0x24, 0x67, 0xb8, 0x06, 0x36, 0xc0, 0xb5, 0x34, 0x3e, 0xda, 0x7c,
    0xd5, 0x78, 0xba, 0x88, 0xdd, 0xb7, 0xa0, 0x76, 0x65, 0x68, 0x47, 0x7f, 0xed, 0x63, 0xcf, 0x53,
    0x18, 0x62, 0x12, 0x2c, 0x6c, 0x15, 0xb4, 0xa7, 0x07, 0x97, 0x3d, 0x41, 0x78, 0x2c, 0xfc, 0x0e,
    0xf4, 0xfe, 0x6c, 0x31, 0x15, 0x98, 0x8a, 0x2e, 0x33, 0x90, 0x15, 0x93, 0x8d, 0x2d, 0xf0, 0xa5,
    0xd3, 0x02, 0x37, 0xa2, 0x59, 0x2c, 0xc1, 0x0c, 0x05, 0xa9, 0xe4, 0xef, 0x6b, 0x69, 0x5b, 0xca,
    0x99, 0x73, 0x6b, 0x1a, 0x49, 0xea, 0x39, 0x60, 0x6a, 0x38, 0x1e, 0xcf, 0xb0, 0x5e, 0xfe, 0x60,
    0xd2, 0x8b, 0x54, 0x82, 0x3e, 0xc5, 0xa3, 0x68, 0x0c, 0x76, 0x5d, 0xe9, 0xdf, 0x4c, 0xfa, 0x54,
    0x87, 0xf3, 0x60, 0xe2, 0x9e, 0x99, 0x34, 0x3e, 0x91, 0x81, 0x1b, 0xae, 0xc3, 0x31, 0xc4, 0x68,
    0x09, 0x85, 0xe6, 0x08, 0xca, 0x5d, 0x40, 0x8e, 0x21, 0x72, 0x5c, 0x6a, 0xa1, 0xb6, 0x1d, 0x5a,
    0x8b, 0x48, 0xd7, 0x5f, 0x4a, 0xaa, 0x9a, 0x3c, 0xbe, 0x88, 0xd3, 0xe0, 0xf1, 0xa5, 0x43, 0x19,
    0x08, 0x1f, 0x77, 0xc7, 0x2c, 0x8f, 0x52, 0x54, 0x74, 0x40, 0xe2, 0x01, 0x00,
];

pub const PROFILE_KEY_CREDENTIAL_PRESENTATION_RESULT: [u8;
    zkgroup::PROFILE_KEY_CREDENTIAL_PRESENTATION_LEN] = [
    0x00, 0xc4, 0xd1, 0x9b, 0xca, 0x1a, 0xe8, 0x44, 0x58, 0x51, 0x68, 0x86, 0x9d, 0xa4, 0x13, 0x3e,
    0x0e, 0x0b, 0xb5, 0x9f, 0x2c, 0xe1, 0x7b, 0x7a, 0xc6, 0x5b, 0xff, 0x5d, 0xa9, 0x61, 0x0e, 0xca,
    0x10, 0x34, 0x29, 0xd8, 0x02, 0x2a, 0x94, 0xba, 0xe2, 0xb5, 0xb1, 0x05, 0x7b, 0x55, 0x95, 0xb8,
    0xad, 0x70, 0xbf, 0xc2, 0xd0, 0xe1, 0xad, 0x66, 0x2c, 0xb7, 0x5e, 0x6b, 0xae, 0x07, 0x82, 0xbe,
    0x6f, 0x00, 0xe3, 0xdb, 0x79, 0x3b, 0xc2, 0x85, 0x61, 0xf0, 0x19, 0x6c, 0x2e, 0x74, 0xda, 0x6f,
    0x30, 0x3f, 0xa8, 0xbc, 0xb7, 0x0c, 0x94, 0x09, 0x66, 0x71, 0xb7, 0x3f, 0x7b, 0x3a, 0x95, 0xfb,
    0x00, 0x22, 0x00, 0xd5, 0xb9, 0x18, 0x0f, 0xa0, 0xef, 0x7d, 0x30, 0x14, 0xd0, 0x13, 0x44, 0x14,
    0x5b, 0x4d, 0x38, 0x48, 0x0d, 0x72, 0xff, 0x25, 0xc2, 0x42, 0x94, 0xe3, 0x05, 0xe5, 0x70, 0x50,
    0x72, 0xe0, 0xd3, 0x2c, 0xc4, 0xe8, 0x4f, 0x5c, 0xaf, 0x31, 0x48, 0x60, 0x89, 0xa4, 0xb9, 0x34,
    0xc8, 0x0c, 0x92, 0xeb, 0xa4, 0x34, 0x72, 0xff, 0x23, 0xa5, 0xaf, 0x93, 0xc3, 0x97, 0x53, 0x5d,
    0x33, 0x80, 0x1f, 0x0e, 0x6f, 0xc6, 0xeb, 0x2e, 0xe0, 0xd1, 0x17, 0xf0, 0x3b, 0xb4, 0xfd, 0x38,
    0xa8, 0xb9, 0xc8, 0x8d, 0x94, 0x70, 0x81, 0x31, 0xf3, 0x87, 0x42, 0xca, 0x80, 0x4a, 0x3c, 0xfc,
    0x4f, 0x94, 0x76, 0xbc, 0x2d, 0x03, 0xf5, 0x3d, 0x17, 0x00, 0x1c, 0x36, 0x47, 0x8a, 0xfb, 0xe9,
    0xcc, 0x53, 0x5a, 0x22, 0x4b, 0x2d, 0xf6, 0xb2, 0xb0, 0x8b, 0xef, 0x06, 0xcb, 0xc7, 0xd4, 0xdc,
    0x42, 0xcc, 0xfc, 0x34, 0x59, 0xf7, 0xac, 0x5c, 0x44, 0x19, 0xae, 0x9f, 0x3c, 0x8a, 0x16, 0x1d,
    0x55, 0x4d, 0x04, 0x77, 0x78, 0x94, 0x32, 0x16, 0x24, 0x08, 0x58, 0xda, 0x3b, 0x11, 0x01, 0x98,
    0x4c, 0x40, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7a, 0x01, 0xee, 0xa6, 0xb2, 0xad, 0xad,
    0x14, 0xd7, 0x1a, 0xb8, 0xb8, 0xe4, 0x11, 0xbe, 0xf3, 0xc5, 0x96, 0xe9, 0x54, 0xb7, 0x0e, 0x40,
    0x31, 0x57, 0x0c, 0xb1, 0xab, 0xd7, 0xe9, 0x32, 0x08, 0x32, 0x41, 0xf1, 0xca, 0xca, 0x31, 0x16,
    0x70, 0x8f, 0xa4, 0x31, 0x9f, 0xbb, 0xdf, 0xe3, 0x51, 0x37, 0x6c, 0x23, 0x64, 0x4a, 0xe0, 0x9a,
    0x42, 0xf0, 0x15, 0x5d, 0xb4, 0x99, 0x6c, 0x9d, 0x0c, 0x7f, 0xfc, 0x85, 0x21, 0xc1, 0x91, 0x4c,
    0x0e, 0x1a, 0x20, 0xae, 0x51, 0xe6, 0x5d, 0xf6, 0x4d, 0xd5, 0xe6, 0xe5, 0x98, 0x5b, 0x3d, 0x9d,
    0x31, 0x73, 0x20, 0x46, 0xd2, 0xd7, 0x7f, 0x9c, 0x08, 0xaa, 0xcc, 0xf0, 0x56, 0xb8, 0x40, 0x26,
    0x07, 0x39, 0x76, 0xee, 0xc6, 0x16, 0x4c, 0xbd, 0xae, 0xe5, 0xd9, 0xe7, 0x6e, 0x49, 0x7f, 0x0c,
    0x29, 0x0a, 0xf6, 0x81, 0xca, 0xbd, 0x5c, 0x51, 0x01, 0x28, 0x2a, 0xbb, 0x26, 0xc3, 0x68, 0x0d,
    0x60, 0x87, 0xce, 0x05, 0x33, 0x10, 0xfe, 0x8a, 0x94, 0xf5, 0x9d, 0x8a, 0xe2, 0x3c, 0xaa, 0xc5,
    0xfc, 0x0e, 0xd0, 0xc3, 0x79, 0x88, 0x8a, 0xbf, 0x02, 0x8a, 0x6f, 0x29, 0xf8, 0x9d, 0x4f, 0xe2,
    0xac, 0xc1, 0x70, 0x63, 0x41, 0xb2, 0x24, 0x5b, 0xa1, 0x88, 0x5b, 0xca, 0x57, 0xe1, 0xe2, 0x7c,
    0xcf, 0x7e, 0xd7, 0x93, 0x71, 0x50, 0x09, 0x65, 0x00, 0x9f, 0x96, 0x0c, 0x2b, 0xa0, 0x0f, 0xad,
    0x3e, 0x93, 0x38, 0x3b, 0x87, 0xce, 0x11, 0x9c, 0xac, 0x0b, 0x33, 0x60, 0xeb, 0x99, 0x28, 0x4c,
    0xe7, 0x8e, 0x2c, 0xbe, 0xd6, 0x80, 0xf7, 0x96, 0x03, 0x73, 0xe0, 0xab, 0x75, 0xc1, 0x90, 0x25,
    0x41, 0x60, 0xc2, 0x35, 0x36, 0x14, 0x10, 0x94, 0x89, 0xe6, 0x53, 0xc9, 0xb2, 0xe1, 0xc9, 0x3f,
    0x92, 0xc7, 0xc5, 0xad, 0x58, 0x3d, 0x98, 0x7a, 0x04, 0xbd, 0x35, 0x41, 0xb2, 0x44, 0x85, 0xc3,
    0x3e, 0xa4, 0x9b, 0xac, 0x43, 0xc8, 0x7c, 0x4a, 0xb3, 0xef, 0xde, 0x2e, 0x2d, 0x7e, 0xc1, 0x0a,
    0x40, 0xbe, 0x54, 0x41, 0x99, 0xf9, 0x25, 0xb2, 0x0b, 0x2c, 0x55, 0x54, 0x2b, 0xc5, 0x64, 0x10,
    0x57, 0x1e, 0x41, 0xcd, 0x8e, 0x02, 0x86, 0xf6, 0x09, 0xa6, 0x67, 0x68, 0xb5, 0x06, 0x1c, 0xcb,
    0x47, 0x77, 0xaf, 0x32, 0x30, 0x99, 0x28, 0xdd, 0x09, 0x76, 0x5d, 0xe9, 0xdf, 0x4c, 0xfa, 0x54,
    0x87, 0xf3, 0x60, 0xe2, 0x9e, 0x99, 0x34, 0x3e, 0x91, 0x81, 0x1b, 0xae, 0xc3, 0x31, 0xc4, 0x68,
    0x09, 0x85, 0xe6, 0x08, 0xca, 0x5d, 0x40, 0x8e, 0x21, 0x72, 0x5c, 0x6a, 0xa1, 0xb6, 0x1d, 0x5a,
    0x8b, 0x48, 0xd7, 0x5f, 0x4a, 0xaa, 0x9a, 0x3c, 0xbe, 0x88, 0xd3, 0xe0, 0xf1, 0xa5, 0x43, 0x19,
    0x08, 0x1f, 0x77, 0xc7, 0x2c, 0x8f, 0x52, 0x54, 0x74, 0x48, 0xc0, 0x3a, 0xb4, 0xaf, 0xbf, 0x6b,
    0x8f, 0xb0, 0xe1, 0x26, 0xc0, 0x37, 0xa0, 0xad, 0x40, 0x94, 0x60, 0x0d, 0xd0, 0xe0, 0x63, 0x4d,
    0x76, 0xf8, 0x8c, 0x21, 0x08, 0x7f, 0x3c, 0xfb, 0x48, 0x5a, 0x89, 0xbc, 0x1e, 0x3a, 0xbc, 0x4c,
    0x95, 0x04, 0x1d, 0x1d, 0x17, 0x0e, 0xcc, 0xf0, 0x29, 0x33, 0xec, 0x53, 0x93, 0xd4, 0xbe, 0x1d,
    0xc5, 0x73, 0xf8, 0x3c, 0x33, 0xd3, 0xb9, 0xa7, 0x46,
];

#[test]
fn test_integration_auth() {
    let server_secret_params = zkgroup::ServerSecretParams::generate(zkgroup::TEST_ARRAY_32);
    let server_public_params = server_secret_params.get_public_params();

    let master_key = zkgroup::groups::GroupMasterKey::new(zkgroup::TEST_ARRAY_32_1);
    let group_secret_params =
        zkgroup::groups::GroupSecretParams::derive_from_master_key(master_key);
    let group_public_params = group_secret_params.get_public_params();

    // Random UID and issueTime
    let uid = zkgroup::TEST_ARRAY_16;
    let redemption_time = 123456u32;

    // SERVER
    // Issue credential
    let randomness = zkgroup::TEST_ARRAY_32_2;
    let auth_credential_response =
        server_secret_params.issue_auth_credential(randomness, uid, redemption_time);

    // CLIENT
    let auth_credential = server_public_params
        .receive_auth_credential(uid, redemption_time, &auth_credential_response)
        .unwrap();

    // Create and decrypt user entry
    let uuid_ciphertext = group_secret_params.encrypt_uuid(uid);
    let plaintext = group_secret_params.decrypt_uuid(uuid_ciphertext).unwrap();
    assert!(plaintext == uid);

    // Create and receive presentation
    let randomness = zkgroup::TEST_ARRAY_32_5;

    let presentation = server_public_params.create_auth_credential_presentation(
        randomness,
        group_secret_params,
        auth_credential,
    );

    let presentation_bytes = &bincode::serialize(&presentation).unwrap();

    //for b in presentation_bytes.iter() {
    //    print!("0x{:02x}, ", b);
    //}
    assert!(AUTH_CREDENTIAL_PRESENTATION_RESULT[..] == presentation_bytes[..]);

    server_secret_params
        .verify_auth_credential_presentation(group_public_params, &presentation)
        .unwrap();

    // test encoding
    // these tests will also discover if the serialized sizes change,
    //   necessitating an update to the LEN constants
    //let mut ccm_bytes = [0u8; zkgroup::common::constants::CLIENT_CREDENTIAL_MANAGER_LEN];
    let mut group_secret_params_bytes = [0u8; zkgroup::common::constants::GROUP_SECRET_PARAMS_LEN];
    let mut server_secret_params_bytes =
        [0u8; zkgroup::common::constants::SERVER_SECRET_PARAMS_LEN];
    let mut group_public_params_bytes = [0u8; zkgroup::common::constants::GROUP_PUBLIC_PARAMS_LEN];
    let mut server_public_params_bytes =
        [0u8; zkgroup::common::constants::SERVER_PUBLIC_PARAMS_LEN];
    let mut auth_credential_response_bytes =
        [0u8; zkgroup::common::constants::AUTH_CREDENTIAL_RESPONSE_LEN];
    let mut auth_credential_bytes = [0u8; zkgroup::common::constants::AUTH_CREDENTIAL_LEN];
    let mut auth_credential_presentation_bytes =
        [0u8; zkgroup::common::constants::AUTH_CREDENTIAL_PRESENTATION_LEN];
    let mut uuid_ciphertext_bytes = [0u8; zkgroup::common::constants::UUID_CIPHERTEXT_LEN];
    let mut uid_bytes = [0u8; zkgroup::common::constants::UUID_LEN];
    let mut randomness_bytes = [0u8; zkgroup::common::constants::RANDOMNESS_LEN];

    //ccm_bytes.copy_from_slice(&bincode::serialize(&client_credential_manager).unwrap());
    group_secret_params_bytes.copy_from_slice(&bincode::serialize(&group_secret_params).unwrap());
    server_secret_params_bytes.copy_from_slice(&bincode::serialize(&server_secret_params).unwrap());
    group_public_params_bytes.copy_from_slice(&bincode::serialize(&group_public_params).unwrap());
    server_public_params_bytes.copy_from_slice(&bincode::serialize(&server_public_params).unwrap());
    auth_credential_response_bytes
        .copy_from_slice(&bincode::serialize(&auth_credential_response).unwrap());
    auth_credential_bytes.copy_from_slice(&bincode::serialize(&auth_credential).unwrap());
    auth_credential_presentation_bytes.copy_from_slice(&bincode::serialize(&presentation).unwrap());
    uuid_ciphertext_bytes.copy_from_slice(&bincode::serialize(&uuid_ciphertext).unwrap());
    uid_bytes.copy_from_slice(&bincode::serialize(&uid).unwrap());
    randomness_bytes.copy_from_slice(&bincode::serialize(&randomness).unwrap());
}

#[test]
fn test_integration_profile() {
    // Random UID and issueTime
    let _uid = zkgroup::TEST_ARRAY_16;

    // SERVER
    let server_secret_params = zkgroup::ServerSecretParams::generate(zkgroup::TEST_ARRAY_32);
    let server_public_params = server_secret_params.get_public_params();

    // CLIENT
    let master_key = zkgroup::groups::GroupMasterKey::new(zkgroup::TEST_ARRAY_32_1);
    let group_secret_params =
        zkgroup::groups::GroupSecretParams::derive_from_master_key(master_key);
    let group_public_params = group_secret_params.get_public_params();

    let uid = zkgroup::TEST_ARRAY_16;
    let profile_key =
        zkgroup::profiles::ProfileKey::create(zkgroup::common::constants::TEST_ARRAY_32_1);
    let profile_key_commitment = profile_key.get_commitment(uid);

    // Create context and request
    let randomness = zkgroup::TEST_ARRAY_32_3;

    let context = server_public_params.create_profile_key_credential_request_context(
        randomness,
        uid,
        profile_key,
    );
    let request = context.get_request();

    // SERVER

    let randomness = zkgroup::TEST_ARRAY_32_4;
    let response = server_secret_params
        .issue_profile_key_credential(randomness, &request, uid, profile_key_commitment)
        .unwrap();

    // CLIENT
    // Gets stored profile credential
    let profile_key_credential = server_public_params
        .receive_profile_key_credential(&context, &response)
        .unwrap();

    // Create encrypted UID and profile key
    let uuid_ciphertext = group_secret_params.encrypt_uuid(uid);
    let plaintext = group_secret_params.decrypt_uuid(uuid_ciphertext).unwrap();
    assert!(plaintext == uid);

    let profile_key_ciphertext = group_secret_params.encrypt_profile_key(profile_key, uid);
    let decrypted_profile_key = group_secret_params
        .decrypt_profile_key(profile_key_ciphertext, uid)
        .unwrap();

    assert!(decrypted_profile_key.get_bytes() == profile_key.get_bytes());

    // Create presentation
    let randomness = zkgroup::TEST_ARRAY_32_5;

    let presentation = server_public_params.create_profile_key_credential_presentation(
        randomness,
        group_secret_params,
        profile_key_credential,
    );

    let presentation_bytes = &bincode::serialize(&presentation).unwrap();
    //for b in presentation_bytes.iter() {
    //    print!("0x{:02x}, ", b);
    //}
    assert!(PROFILE_KEY_CREDENTIAL_PRESENTATION_RESULT[..] == presentation_bytes[..]);

    // SERVER
    server_secret_params
        .verify_profile_key_credential_presentation(group_public_params, &presentation)
        .unwrap();

    // test encoding
    // these tests will also discover if the serialized sizes change,
    //   necessitating an update to the LEN constants

    let mut profile_key_commitment_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_COMMITMENT_LEN];
    let mut profile_key_credential_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_LEN];
    let mut profile_key_credential_presentation_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_PRESENTATION_LEN];
    let mut profile_key_credential_request_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_REQUEST_LEN];
    let mut profile_key_credential_request_context_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_REQUEST_CONTEXT_LEN];
    let mut profile_key_credential_response_bytes =
        [0u8; zkgroup::common::constants::PROFILE_KEY_CREDENTIAL_RESPONSE_LEN];

    profile_key_commitment_bytes
        .copy_from_slice(&bincode::serialize(&profile_key_commitment).unwrap());
    profile_key_credential_bytes
        .copy_from_slice(&bincode::serialize(&profile_key_credential).unwrap());
    profile_key_credential_presentation_bytes
        .copy_from_slice(&bincode::serialize(&presentation).unwrap());
    profile_key_credential_request_bytes.copy_from_slice(&bincode::serialize(&request).unwrap());
    profile_key_credential_request_context_bytes
        .copy_from_slice(&bincode::serialize(&context).unwrap());
    profile_key_credential_response_bytes.copy_from_slice(&bincode::serialize(&response).unwrap());
}

#[test]
fn test_server_sigs() {
    let server_secret_params =
        zkgroup::api::server_params::ServerSecretParams::generate(zkgroup::TEST_ARRAY_32);
    let server_public_params = server_secret_params.get_public_params();
    let randomness = zkgroup::TEST_ARRAY_32_2;
    let message = zkgroup::TEST_ARRAY_32_1;
    let signature = server_secret_params.sign(randomness, &message).unwrap();
    //println!("signature = {:#x?}", &signature[..]);
    for b in signature.iter() {
        print!("0x{:02x}, ", b);
    }
    assert!(
        signature[..]
            == [
                0x87, 0xd3, 0x54, 0x56, 0x4d, 0x35, 0xef, 0x91, 0xed, 0xba, 0x85, 0x1e, 0x08, 0x15,
                0x61, 0x2e, 0x86, 0x4c, 0x22, 0x7a, 0x04, 0x71, 0xd5, 0x0c, 0x27, 0x06, 0x98, 0x60,
                0x44, 0x06, 0xd0, 0x03, 0xa5, 0x54, 0x73, 0xf5, 0x76, 0xcf, 0x24, 0x1f, 0xc6, 0xb4,
                0x1c, 0x6b, 0x16, 0xe5, 0xe6, 0x3b, 0x33, 0x3c, 0x02, 0xfe, 0x4a, 0x33, 0x85, 0x80,
                0x22, 0xfd, 0xd7, 0xa4, 0xab, 0x36, 0x7b, 0x06,
            ][..]
    );
    server_public_params
        .verify_signature(&message, signature)
        .unwrap();
}

#[test]
fn test_blob_encryption() {
    let master_key = zkgroup::groups::GroupMasterKey::new(zkgroup::TEST_ARRAY_32_1);
    let group_secret_params =
        zkgroup::groups::GroupSecretParams::derive_from_master_key(master_key);
    let randomness = zkgroup::TEST_ARRAY_32_2;

    let plaintext_vec = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19,
    ];

    // WARNING: THIS VECTOR DOES *NOT* MATCH JAVA/SWIFT/NODE AS THEY IMPLEMENT PADDING
    let ciphertext_vec = vec![
        0xe9, 0x58, 0x07, 0xb1, 0x90, 0xd4, 0x78, 0xd7, 0xbe, 0x3a, 0x77, 0xb2, 0x29, 0x27, 0x13,
        0x2e, 0xeb, 0xa5, 0x1c, 0x73, 0x9c, 0xd5, 0x70, 0x73, 0x17, 0xf7, 0x3e, 0x59, 0x1a, 0x91,
        0x5f, 0xff, 0x1f, 0x20, 0xa3, 0x02, 0x69, 0x2a, 0xfd, 0xc7, 0x08, 0x7f, 0x10, 0x19, 0x60,
        0x00,
    ];

    let calc_ciphertext_vec = group_secret_params
        .encrypt_blob(randomness, &plaintext_vec)
        .unwrap();
    let calc_plaintext_vec = group_secret_params
        .decrypt_blob(&calc_ciphertext_vec)
        .unwrap();
    assert!(calc_plaintext_vec == plaintext_vec);
    for b in calc_ciphertext_vec.iter() {
        print!("0x{:02x}, ", b);
    }
    assert!(calc_ciphertext_vec == ciphertext_vec);
}
