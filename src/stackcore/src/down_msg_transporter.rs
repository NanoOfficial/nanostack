/**
 * @filename: down_msg_transporter.rs
 * @author: Krisna Pranav
 * @copyright: COPYRIGHT (2023) MIT LICENSE Krisna Pranav
*/
use crate::*;

#[derive(Serialize)]
pub struct DownMsgTransporterForSer<'a, DMsg: Serialize> {
    pub down_msg: &'a DMsg,
    pub cor_id: CorId,
}

#[cfg(feature = "serde-lite")]
#[derive(Deserialize)]
pub struct DownMsgTransporterForDe<DMsg: Deserialize> {
    pub down_msg: DMsg,
    pub cor_id: CorId,
}

#[cfg(feature = "serde")]
#[derive(Deserialize)]
pub struct DownMsgTransporterForDe<DMsg> {
    pub down_msg: DMsg,
    pub cor_id: CorId,
}
