mod processes;
mod schedulers;
mod services;

use schedulers::main_scheduler::create_main_scheduler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_scheduler = create_main_scheduler();

    main_scheduler.start().await?;
    Ok(())
}
