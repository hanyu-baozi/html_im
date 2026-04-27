use actix_web::{dev::{Service, ServiceRequest, ServiceResponse}, Error};
use futures_util::future::{ready, Ready};
use log::info;
use std::future::Future;
use std::pin::Pin;

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S> LoggingMiddleware<S> {
    pub fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("Request: {} {}", req.method(), req.path());
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            info!("Response: {}", res.status());
            Ok(res)
        })
    }
}
