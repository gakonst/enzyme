use futures::future::{Future, FutureExt};
use futures::TryStreamExt;
use http::request::Builder;
use hyper::body::Body;
use hyper::{client::HttpConnector, Client};
use serde::{Deserialize, Serialize};
use std::{error::Error, pin::Pin};

pub type RemoteAsyncResponse<Res> = Pin<Box<dyn Future<Output = Result<Res, ()>> + Send>>;

pub struct RemoteEndpoint;

impl RemoteEndpoint {
    pub fn new<Req, Res>(
        f: fn() -> &'static mut Builder,
    ) -> impl Fn(Req) -> RemoteAsyncResponse<Res>
    where
        Req: Serialize + Send + 'static,
        Res: for<'de> Deserialize<'de>,
    {
        move |req: Req| {
            let fut = async move {
                let bytes = match serde_json::to_vec(&req) {
                    Ok(bytes) => bytes,
                    Err(_) => return Err(()),
                };

                let builder = f();

                let request = match builder.body(Body::from(bytes)) {
                    Ok(request) => request,
                    Err(_) => return Err(()),
                };

                let response = match Client::default().request(request).await.map_err(|_| ()) {
                    Ok(response) => response,
                    Err(_) => return Err(()),
                };

                let bytes = match Body::wrap_stream(response.into_body()).try_concat().await {
                    Ok(chunk) => chunk.into_bytes(),
                    Err(e) => return Err(()),
                };

                serde_json::from_slice(&bytes).map_err(|_| ())
            };

            fut.boxed()
        }
    }
}
