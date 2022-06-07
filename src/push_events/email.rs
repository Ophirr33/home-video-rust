use async_trait::async_trait;

use super::*;

struct PushEventEmailSink {
    recipients: Vec<String>,
}

pub fn new_push_events_email_sink(recipients: Vec<String>) -> Box<dyn PushEventSink + Sync> {
    Box::new(PushEventEmailSink { recipients })
}

#[async_trait]
impl PushEventSink for PushEventEmailSink {
    async fn send_push_event(&self, push_event: PushEvent) -> Result<(), PushEventsSinkError> {
        Ok(())
    }
}
