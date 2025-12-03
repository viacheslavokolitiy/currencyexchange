use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage, HttpRequest};
use tracing::Span;
use tracing_actix_web::{DefaultRootSpanBuilder, Level, RootSpan, RootSpanBuilder, TracingLogger};

pub struct NetworkLogSpanBuilder {
    middleware: TracingLogger<NetworkLogSpanBuilder>,
}

impl NetworkLogSpanBuilder {
    pub fn new() -> Self {
        let logger = TracingLogger::<NetworkLogSpanBuilder>::new();
        Self {
            middleware: logger
        }
    }

    pub fn middleware(&self) -> &TracingLogger<NetworkLogSpanBuilder> {
        &self.middleware
    }
}

pub fn process_request_path(path: &str) -> Level {
    match path {
        _ => Level::INFO,
    }
}

impl RootSpanBuilder for NetworkLogSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let level = process_request_path(request.path());
        tracing_actix_web::root_span!(level = level, request)
    }

    fn on_request_end<B: MessageBody>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}