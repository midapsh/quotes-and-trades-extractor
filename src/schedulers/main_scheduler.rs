use crate::processes::debank_process::debank_process;
use tokio_cron_scheduler::{Job, JobScheduler};

pub fn create_main_scheduler() -> JobScheduler {
    let mut sched = JobScheduler::new();

    let _ = sched
        .add(Job::new_async("1/10 * * * * *", |_uuid, _l| Box::pin(debank_process())).unwrap());

    sched
}
