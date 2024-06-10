use std::future::IntoFuture;
use httpclient::multipart::Part;
use httpclient::{InMemoryBody, RequestBuilder};
use crate::GmailClient;

pub struct Batch<'a> {
    pub(crate) client: &'a GmailClient,
//     pub(crate) requests: Vec::new(),
}
//
// impl<'a> IntoFuture for Batch<'_> {
//     type Output = Vec<httpclient::InMemoryResult<()>>;
//     type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
//     fn into_future(self) -> Self::IntoFuture {
//         Box::pin(async move {
//             let req = self.client.client.post("/batch/gmail/v1");
//             let form = httpclient::multipart::Form::new();
//             let mut part = Part::new(InMemoryBody::Empty);
//             part.headers.insert("Content-Type", "application/http".parse().unwrap());
//             let form = form.part(part);
//             req.multipart(form);
//
//             // let mut results = Vec::new();
//             // for request in self.requests {
//             //     results.push(request.into_future().await);
//             // }
//         })
//     }
// }