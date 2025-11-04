//! std udp interface impl
//!
//! std udp uses COBS for framing over a UDP stream.

use crate::interface_manager::{
    utils::{framed_stream, std::StdQueue},
    Interface,
};

/// An interface implementation for UDP using tokio
pub struct TokioUdpInterface {}

impl Interface for TokioUdpInterface {
    type Sink = framed_stream::Sink<StdQueue>;
}
