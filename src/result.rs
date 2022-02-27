use std::sync::mpsc::{RecvError, SendError};

use crate::message::Message;

#[derive(Debug)]
pub enum ServiceError {
    ThreadPoolSizeMustBePositive,
    SendError(SendError<Message>),
    RecvError(RecvError),
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl From<SendError<Message>> for ServiceError {
    fn from(e: SendError<Message>) -> Self {
        ServiceError::SendError(e)
    }
}
