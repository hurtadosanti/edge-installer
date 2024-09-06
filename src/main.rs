mod input;
mod backend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the Edge Installer");

    let disks = vec!["Disk 1: /dev/sda", "Disk 2: /dev/sdb", "Disk 3: /dev/sdc"];
    let images = vec!["Image 1: Ubuntu 20.04", "Image 2: Fedora 34", "Image 3: Arch Linux"];

    let customize = input::prompt_with_timeout("Do you want to start the installation with default values? (y/n)", 2).await;

    let selected_disk;
    let selected_image;

    if customize {
        println!("Using default installation configuration.");
        selected_disk = disks[0];
        selected_image = images[0];
    } else {
        println!("Customize the installation");
        selected_disk = disks[input::select_disk_prompt(&disks)];
        selected_image = images[input::select_image_prompt(&images)];
    }

    backend::setup_installation(selected_disk, selected_image).await?;

    println!("Installation started...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Installation completed.");

    // TUI collects shutdown input
    let shutdown = input::prompt_with_timeout("Do you want to shutdown the system? (y/n)", 2).await;
    backend::handle_shutdown(shutdown).await;

    Ok(())
}
