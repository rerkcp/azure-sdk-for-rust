use azure_core::headers::utc_date_from_rfc2822;
use azure_storage::core::headers::CommonStorageResponseHeaders;
use azure_storage::core::xml::read_xml;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct PeekMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub messages: Vec<PeekMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeekMessageInternal {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "InsertionTime")]
    pub insertion_time: String,
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: u64,
    #[serde(rename = "MessageText")]
    pub message_text: String,
}

#[derive(Debug, Clone)]
pub struct PeekMessage {
    pub message_id: String,
    pub insertion_time: DateTime<Utc>,
    pub expiration_time: DateTime<Utc>,
    pub dequeue_count: u64,
    pub message_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeekMessagesInternal {
    #[serde(rename = "QueueMessage")]
    pub messages: Option<Vec<PeekMessageInternal>>,
}

impl std::convert::TryFrom<&Response<Bytes>> for PeekMessagesResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = response.body();

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);
        let response: PeekMessagesInternal = read_xml(body)?;
        debug!("response == {:?}", response);

        let mut messages = Vec::new();
        for message in response.messages.unwrap_or_default().into_iter() {
            messages.push(PeekMessage {
                message_id: message.message_id,
                insertion_time: utc_date_from_rfc2822(&message.insertion_time)?,
                expiration_time: utc_date_from_rfc2822(&message.expiration_time)?,
                dequeue_count: message.dequeue_count,
                message_text: message.message_text,
            })
        }

        Ok(PeekMessagesResponse {
            common_storage_response_headers: headers.try_into()?,
            messages,
        })
    }
}
