use tracing::{subscriber, Subscriber};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{self, MakeWriter},
    layer::SubscriberExt,
    EnvFilter, Registry,
};

use crate::get_root_dir;

pub fn init() -> WorkerGuard {
    let root_dir = get_root_dir().expect("Failed to get root dir");
    let log_dir = root_dir.join("logs");

    // 创建按天轮转的文件日志记录器
    let file_appender = RollingFileAppender::new(Rotation::DAILY, &log_dir, "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 从环境变量获取日志级别，默认为info
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = create_subscriber("app", env_filter, non_blocking);
    init_subscriber(subscriber).expect("Failed to initialize subscriber");

    guard
}

fn create_subscriber<W>(
    name: &str,
    env_filter: EnvFilter,
    writer: W,
) -> impl Subscriber + Sync + Send
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // 控制台输出层
    let fmt_layer = fmt::Layer::default()
        .with_level(true)
        .with_timer(fmt::time::ChronoLocal::rfc_3339())
        .with_span_events(fmt::format::FmtSpan::CLOSE)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_line_number(true)
        .with_ansi(true)
        .compact();

    // 文件输出层(JSON格式)
    let file_layer = BunyanFormattingLayer::new(name.into(), writer);

    //控制台输出层(JSON格式)
    // let console_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        // .with(console_layer)
        .with(file_layer)
        .with(fmt_layer)
}

pub fn init_subscriber<S>(subscriber: S) -> anyhow::Result<()>
where
    S: Subscriber + Send + Sync + 'static,
{
    LogTracer::init()?;
    subscriber::set_global_default(subscriber)?;
    Ok(())
}
