//
// Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use libsignal_bridge_macros::*;
use libsignal_bridge_types::grpc::GrpcHeaders;
use signal_grpc::{GrpcClient, GrpcReply, GrpcReplyListener, Result};

use crate::support::*;
use crate::*;

bridge_handle_fns!(GrpcClient, clone = false);

#[bridge_fn(ffi = false, node = false)]
pub fn GrpcClient_New(target: String) -> Result<GrpcClient> {
    GrpcClient::new(target)
}

#[bridge_fn(ffi = false, node = false)]
pub fn GrpcClient_SendDirectMessage(
    grpc_client: &mut GrpcClient,
    method: String,
    url_fragment: String,
    body: &[u8],
    headers: GrpcHeaders,
) -> Result<GrpcReply> {
    grpc_client.send_direct_message(method, url_fragment, body, headers.0)
}

#[bridge_fn(ffi = false, node = false)]
pub fn GrpcClient_OpenStream(
    grpc_client: &mut GrpcClient,
    uri: String,
    headers: GrpcHeaders,
    listener: &mut dyn GrpcReplyListener,
) -> Result<()> {
    grpc_client.open_stream(uri, headers.0, listener)
}

#[bridge_fn(ffi = false, node = false)]
pub fn GrpcClient_SendMessageOnStream(
    grpc_client: &mut GrpcClient,
    method: String,
    url_fragment: String,
    body: &[u8],
    headers: GrpcHeaders,
) -> Result<()> {
    grpc_client.send_message_on_stream(method, url_fragment, body, headers.0)
}
