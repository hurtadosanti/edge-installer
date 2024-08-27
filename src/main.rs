use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn prompt_with_timeout(prompt_text: &str, timeout_secs: u64) -> bool {
    let (tx, rx) = mpsc::channel();
    let prompt_text = prompt_text.to_string();

    thread::spawn(move || {
        let response = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt_text)
            .default(true) // Default to 'y' (yes)
            .interact()
            .unwrap_or(true); // If interaction fails, assume default
        tx.send(response).unwrap();
    });

    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(result) => result,
        Err(_) => {
            println!("\nNo response received in {} seconds. Proceeding with default.", timeout_secs);
            true // Assume 'y' (yes) if timeout
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the installation wizard.");

    let disks = vec!["Disk 1: /dev/sda", "Disk 2: /dev/sdb", "Disk 3: /dev/sdc"];
    let images = vec!["Image 1: Ubuntu 20.04", "Image 2: Fedora 34", "Image 3: Arch Linux"];

    let customize = prompt_with_timeout("Do you want to start the installation with default values? (y/n)", 5);

    if customize {
        println!("Using default installation configuration.");
        println!("Selected Disk: {}", disks[0]);
        println!("Selected Image: {}", images[0]);
    } else {
        println!("Customize the installation");

        // Prompt for disk selection
        let disk_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a disk for installation")
            .items(&disks)
            .default(0) // Default to the first disk
            .interact()?;
        let selected_disk = disk_selection;

        // Prompt for image selection
        let image_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an image to install")
            .items(&images)
            .default(0) // Default to the first image
            .interact()?;
        let selected_image = image_selection;

        // Write selections to the output file
        let mut output_file = File::create("installation_output.txt")?;
        writeln!(output_file, "Selected Disk: {}", disks[selected_disk])?;
        writeln!(output_file, "Selected Image: {}", images[selected_image])?;
        println!("Installation configuration saved. Proceeding with installation...");

        // Show selected configuration
        println!("Selected Disk: {}", disks[selected_disk]);
        println!("Selected Image: {}", images[selected_image]);
    }

    // Proceed with the default installation if not customized
    println!("Installation started...");

    thread::sleep(Duration::from_secs(3)); // Simulate installation duration
    println!("Installation completed.");

    let shutdown = prompt_with_timeout("Do you want to abort the shutdown? (y/n)", 5);
    if shutdown {
        println!("Shutdown initiated...");
    } else {
        println!("Shutdown aborted.");
    }

    Ok(())
}
