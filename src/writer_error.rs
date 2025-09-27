// use crate::channel_data::ChannelData;
// use std::sync::mpsc::SendError as SyncSendError;
// use thiserror::Error;
//
// #[derive(Debug, Error)]
// pub enum WriterError {
// 	#[error("SendError:{0}")]
// 	SendError(#[source] SyncSendError<ChannelData>),
// 	#[error("AlreadyClosedError")]
// 	AlreadyClosedError,
// }
//
// impl From<SyncSendError<ChannelData>> for WriterError {
// 	fn from(value: SyncSendError<ChannelData>) -> Self {
// 		WriterError::SendError(value)
// 	}
// }
//
// impl WriterError {
// 	pub fn is_already_closed(&self) -> bool {
// 		match self {
// 			Self::AlreadyClosedError => true,
// 			_ => false,
// 		}
// 	}
// }
