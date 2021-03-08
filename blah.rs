use tracing_subscriber::layer::SubscriberExt;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

#[tracing::instrument]
async fn trace_test() ->  Result<()> {
    tracing::warn!("start");
    tracing::warn!("end! span: {:?}", tracing::Span::current());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // k port-forward -n monitoring service/grafana-agent-traces 55680:55680
    let otlp_endpoint = std::env::var("OPENTELEMETRY_ENDPOINT_URL").unwrap_or("http://0.0.0.0:55680".to_string());
    let (tracer, _uninstall) = opentelemetry_otlp::new_pipeline()
        .with_endpoint(&otlp_endpoint)
        .install()?;

    // Register all subscribers
    let collector = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_subscriber::fmt::layer());

    // Uncomment this for a working example (and remove async from trace_test)
    //tracing::subscriber::with_default(collector, || {
    //    let r = trace_test();
    //    tracing::info!("trace test returned: {:?}", r);
    //});

    // Example that we want to get working:
    tracing::subscriber::set_global_default(collector)?;
    let r = trace_test().await?;
    tracing::info!("trace test returned: {:?}", r);

    Ok(())
}
