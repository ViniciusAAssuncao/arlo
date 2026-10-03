use std::collections::BTreeMap;
use std::sync::Mutex;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

static QUERIES: Mutex<BTreeMap<String, (usize, f64)>> = Mutex::new(BTreeMap::new());

#[derive(Default)]
struct Query {
    summary: String,
    seconds: f64,
}

impl Visit for Query {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "summary" {
            self.summary = value.to_owned();
        }
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        if field.name() == "elapsed_secs" {
            self.seconds = value;
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "summary" {
            self.summary = format!("{value:?}");
        }
    }
}

struct QuerySubscriber;

impl Subscriber for QuerySubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.target() == "sqlx::query"
    }

    fn new_span(&self, _: &Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _: &Id, _: &Record<'_>) {}
    fn record_follows_from(&self, _: &Id, _: &Id) {}
    fn enter(&self, _: &Id) {}
    fn exit(&self, _: &Id) {}

    fn event(&self, event: &Event<'_>) {
        if !self.enabled(event.metadata()) {
            return;
        }
        let mut query = Query::default();
        event.record(&mut query);
        let mut queries = QUERIES.lock().unwrap();
        let entry = queries.entry(query.summary).or_default();
        entry.0 += 1;
        entry.1 += query.seconds * 1000.0;
    }
}

pub fn enable() -> crate::Result<()> {
    if std::env::var_os("ARLO_PERF_SQL_TRACE").is_some() {
        tracing::subscriber::set_global_default(QuerySubscriber)?;
    }
    Ok(())
}

pub fn take() -> serde_json::Value {
    let mut queries = QUERIES.lock().unwrap();
    let result = serde_json::json!({
        "queries": queries.values().map(|entry| entry.0).sum::<usize>(),
        "query_elapsed_ms": queries.values().map(|entry| entry.1).sum::<f64>(),
        "groups": &*queries,
    });
    queries.clear();
    result
}
