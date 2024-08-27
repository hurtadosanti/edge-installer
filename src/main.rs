use std::error::Error;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn prompt_with_timeout(prompt_text: &str, timeout_secs: u64) -> bool {
    let result = Arc::new((Mutex::new(None), Condvar::new()));
    let result_clone = Arc::clone(&result);
    let prompt_text = prompt_text.to_string();

    thread::spawn(move || {
        let response = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt_text)
            .default(true) // Default to 'y' (yes)
            .interact()
            .unwrap_or(true); // If interaction fails, assume default

        let (lock, cvar) = &*result_clone;
        let mut response_lock = lock.lock().unwrap();
        *response_lock = Some(response);
        cvar.notify_one(); // Notify that the input is received
    });

    let (lock, cvar) = &*result;
    let response_lock = lock.lock().unwrap();

    let timeout = Duration::from_secs(timeout_secs);
    let (response_lock, timeout_result) = cvar.wait_timeout(response_lock, timeout).unwrap();

    match *response_lock {
        Some(response) => response,
        None => {
            if timeout_result.timed_out() {
                println!("\nNo response received in {} seconds. Proceeding with default.", timeout_secs);
                true // Assume 'y' (yes) if timeout
            } else {
                false // This should not happen but added as a fallback
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the installation wizard.");

    let disks = vec!["Disk 1: /dev/sda", "Disk 2: /dev/sdb", "Disk 3: /dev/sdc"];
    let images = vec!["Image 1: Ubuntu 20.04", "Image 2: Fedora 34", "Image 3: Arch Linux"];

    setup_installation(&disks, &images)?;

    // Proceed with the default installation if not customized
    println!("Installation started...");

    thread::sleep(Duration::from_secs(3)); // Simulate installation duration
    println!("Installation completed.");

    handle_shutdown();

    Ok(())
}

fn handle_shutdown() {
    let shutdown = prompt_with_timeout("Do you want to abort the shutdown? (y/n)", 5);
    if shutdown {
        println!("Shutdown initiated...");
    } else {
        println!("Shutdown aborted.");
    }
}

fn setup_installation(disks: &Vec<&str>, images: &Vec<&str>) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
