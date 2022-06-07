mod email;
mod sqlite;

use async_trait::async_trait;
use std::path::PathBuf;
use thiserror::Error;

pub type EpochSeconds = i64;
pub type DateTimePeriod = std::ops::Range<EpochSeconds>;

#[async_trait]
pub trait PushEvents {
    async fn send_push_events(&self) -> Result<SendPushEventsStats, SendPushEventsError>;
}

fn new_push_events(config: PushEventsConfig) -> Box<dyn PushEvents> {
    let motion_event_repo = match config.motion_event_repo_config {
        None => Box::new(NoOpMotionEvents),
        Some(MotionEventsConfig::Sqlite { database }) => sqlite::new_motion_events_via_sqlite(database),
    };
    let push_event_sink = match config.push_event_sink_config {
        None => Box::new(NoOpPushEventSink),
        Some(PushEventSinkConfig::Email { recipients }) => email::new_push_events_email_sink(recipients),
    };
    Box::new(PushEventsWorkflow {
        motion_event_repo,
        push_event_sink,
    })
}

struct PushEventsWorkflow {
    motion_event_repo: Box<dyn MotionEvents + Sync>,
    push_event_sink: Box<dyn PushEventSink + Sync>,
}

#[derive(Debug, Clone, Default)]
pub struct PushEventsConfig {
    push_event_sink_config: Option<PushEventSinkConfig>,
    motion_event_repo_config: Option<MotionEventsConfig>,
}

#[derive(Debug, Clone)]
pub enum MotionEventsConfig {
    Sqlite { database: SqliteDatabaseLocation },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqliteDatabaseLocation {
    InMemory,
    InFile(PathBuf),
}

#[derive(Debug, Clone)]
pub enum PushEventSinkConfig {
    Email { recipients: Vec<String> },
}

#[async_trait]
impl PushEvents for PushEventsWorkflow {
    async fn send_push_events(&self) -> Result<SendPushEventsStats, SendPushEventsError> {
        Ok(SendPushEventsStats::default())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SendPushEventsStats {
    pub num_events: usize,
    pub time_period: DateTimePeriod,
}

#[derive(Error, Debug)]
pub enum SendPushEventsError {}

#[async_trait]
trait MotionEvents {
    async fn load_latest_events(&self, timestamp_order: Order) -> Result<Vec<MotionEvent>, MotionEventError>;
    async fn mark_events_sent(&self, events: Vec<MotionEvent>) -> Result<(), MotionEventError>;
}

#[derive(Debug, Clone)]
struct NoOpMotionEvents;

#[async_trait]
impl MotionEvents for NoOpMotionEvents {
    async fn load_latest_events(&self, timestamp_order: Order) -> Result<Vec<MotionEvent>, MotionEventError> {
        Ok(Vec::new())
    }
    async fn mark_events_sent(&self, events: Vec<MotionEvent>) -> Result<(), MotionEventError> {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MotionEvent {
    camera: isize,
    file_name: PathBuf,
    file_type: isize,        // not sure what these ints mean tbh
    timestamp: EpochSeconds, // unix epoch seconds
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    Ascending,
    Descending,
}

#[derive(Error, Debug)]
enum MotionEventError {}

#[async_trait]
trait PushEventSink {
    async fn send_push_event(&self, push_event: PushEvent) -> Result<(), PushEventsSinkError>;
}

struct NoOpPushEventSink;

#[async_trait]
impl PushEventSink for NoOpPushEventSink {
    async fn send_push_event(&self, push_event: PushEvent) -> Result<(), PushEventsSinkError> {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct PushEvent {
    // TODO: image frame?
    num_motion_events: usize,
    date_time_period: DateTimePeriod,
    latest_event_file: PathBuf,
}

#[derive(Error, Debug)]
enum PushEventsSinkError {}
