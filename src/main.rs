use std::error::Error;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use tokio::time::{self, Duration};

async fn prompt_with_timeout(prompt_text: &str, timeout_secs: u64) -> bool {
    let prompt_text = prompt_text.to_string();

    let (tx, rx) = tokio::sync::oneshot::channel();
    let prompt_task = tokio::spawn(async move {
        let result = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt_text)
            .default(true)
            .interact()
            .unwrap_or(true);

        // Send the result through the channel
        let _ = tx.send(result);
    });

    match time::timeout(Duration::from_secs(timeout_secs), rx).await {
        Ok(Ok(result)) => result, // Got the result from the prompt
        _ => {
            // Timeout occurred or some error, so cancel the prompt task
            prompt_task.abort();
            println!("\nNo response received in {} seconds. Proceeding with default.", timeout_secs);
            true
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to the installation wizard.");

    let disks = vec!["Disk 1: /dev/sda", "Disk 2: /dev/sdb", "Disk 3: /dev/sdc"];
    let images = vec!["Image 1: Ubuntu 20.04", "Image 2: Fedora 34", "Image 3: Arch Linux"];

    setup_installation(&disks, &images).await?;

    println!("Installation started...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Installation completed.");

    handle_shutdown().await;

    Ok(())
}

async fn handle_shutdown() {
    let shutdown = prompt_with_timeout("Do you want to shutdown the system? (y/n)", 2).await;
    if shutdown {
        println!("Shutdown initiated...");
    } else {
        println!("Shutdown aborted.");
    }
    std::process::exit(0);
}

async fn setup_installation(disks: &[&str], images: &[&str]) -> Result<(), Box<dyn Error>> {
    let customize = prompt_with_timeout("Do you want to start the installation with default values? (y/n)", 2).await;

    if customize {
        println!("Using default installation configuration.");
        println!("Selected Disk: {}", disks[0]);
        println!("Selected Image: {}", images[0]);
    } else {
        println!("Customize the installation");

        let selected_disk = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a disk for installation")
            .items(disks)
            .default(0)
            .interact()?;

        let selected_image = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an image to install")
            .items(images)
            .default(0)
            .interact()?;

        println!("Selected Disk: {}", disks[selected_disk]);
        println!("Selected Image: {}", images[selected_image]);
    }
    Ok(())
}
