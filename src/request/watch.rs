use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::watch`].

On request success, this will return a [`WatchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchRequest {
    pub label_filter_action: Option<String>,
    pub label_ids: Option<Vec<String>>,
    pub topic_name: Option<String>,
    pub user_id: String,
}
impl WatchRequest {}
impl FluentRequest<'_, WatchRequest> {
    pub fn label_filter_action(mut self, label_filter_action: &str) -> Self {
        self.params.label_filter_action = Some(label_filter_action.to_owned());
        self
    }
    pub fn label_ids(
        mut self,
        label_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .label_ids = Some(
            label_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn topic_name(mut self, topic_name: &str) -> Self {
        self.params.topic_name = Some(topic_name.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WatchRequest> {
    type Output = httpclient::InMemoryResult<WatchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/watch", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.label_filter_action {
                r = r.json(json!({ "labelFilterAction" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.label_ids {
                r = r.json(json!({ "labelIds" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.topic_name {
                r = r.json(json!({ "topicName" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}