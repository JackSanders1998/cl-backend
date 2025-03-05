use axum::body::Body;
use axum::{extract::MatchedPath, http::Request};
use tower_http::trace::{HttpMakeClassifier, TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct CustomTraceLayer {}

impl CustomTraceLayer {
    pub fn init() {
        // https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/src/main.rs
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    // axum logs rejections from built-in extractors with the `axum::rejection`
                    // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                    format!(
                        "{}=debug,tower_http=debug,axum::rejection=trace",
                        env!("CARGO_CRATE_NAME")
                    )
                    .into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    pub fn setup() -> TraceLayer<HttpMakeClassifier, fn(&Request<Body>) -> Span> {
        // https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/src/main.rs
        TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        })
        // .on_request(|_request: &Request<Body>, _span: &Span| {
        //     // You can use `_span.record("some_other_field", value)` in one of these
        //     // closures to attach a value to the initially empty field in the info_span
        //     // created above.
        // })
        // .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
        //     // ...
        // })
        // .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
        //     // ...
        // })
        // .on_eos(
        //     |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
        //         // ...
        //     },
        // )
        // .on_failure(
        //     |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
        //         // ...
        //     },
        // );
    }
}
