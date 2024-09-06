mod input;
mod backend;

use log::{debug, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("Welcome to the Edge Installer");
    let disks = backend::load_disks()?;
    let images = backend::load_images();

    let customize = input::prompt_with_timeout("Do you want to start the installation with default values? (y/n)", 2).await;

    let selected_disk;
    let selected_image;

    if customize {
        debug!("Using default installation configuration.");
        selected_disk = &disks[0];
        selected_image = &images[0];
    } else {
        debug!("Customize the installation");
        selected_disk = &disks[input::select_disk_prompt(&disks)];
        selected_image = &images[input::select_image_prompt(&images)];
    }

    backend::setup_installation(selected_disk, selected_image).await?;

    debug!("Installation started...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    debug!("Installation completed.");

    let shutdown = input::prompt_with_timeout("Do you want to shutdown the system? (y/n)", 2).await;
    backend::handle_shutdown(shutdown).await;

    info!("Exiting...");
    Ok(())
}
