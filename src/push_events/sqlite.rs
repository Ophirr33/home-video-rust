use async_trait::async_trait;

use super::*;

struct MotionEventsViaSqlite {}

pub fn new_motion_events_via_sqlite(
    database: SqliteDatabaseLocation,
) -> Box<dyn MotionEvents + Sync> {
    Box::new(MotionEventsViaSqlite {})
}

#[async_trait]
impl MotionEvents for MotionEventsViaSqlite {
    async fn load_latest_events(
        &self,
        timestamp_order: Order,
    ) -> Result<Vec<MotionEvent>, MotionEventError> {
        Ok(Vec::new())
    }
    async fn mark_events_sent(&self, events: Vec<MotionEvent>) -> Result<(), MotionEventError> {
        Ok(())
    }
}
