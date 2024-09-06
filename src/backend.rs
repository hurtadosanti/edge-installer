/// The `backend` module handles the core installation logic and shutdown processes.
/// It processes the user input collected from the TUI and performs the necessary operations.
use std::error::Error;
/// Performs the installation setup based on the selected disk and image.
///
/// This function outputs the selected disk and image and simulates the installation process.
///
/// # Arguments
/// * `selected_disk` - The disk chosen for installation.
/// * `selected_image` - The image chosen for installation.
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if successful, or an error if something goes wrong.
pub async fn setup_installation(selected_disk: &str, selected_image: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting the installation process...");
    println!("Selected Disk: {}", selected_disk);
    println!("Selected Image: {}", selected_image);
    // Perform the installation logic with the given disk and image
    Ok(())
}
/// Handles the shutdown process after installation based on user input.
///
/// If the user confirms, the system will shut down (simulated). Otherwise, it will print a message indicating the shutdown was aborted.
///
/// # Arguments
/// * `shutdown` - A boolean indicating whether the user chose to shut down the system.
///
/// # Returns
/// * No return value. This function will terminate the process if shutdown is confirmed.
pub async fn handle_shutdown(shutdown: bool) {
    if shutdown {
        println!("Shutdown initiated...");
    } else {
        println!("Shutdown aborted.");
    }
    println!("\x1Bc"); // Optionally reset the terminal
    println!("Installation was finished successfully.");
    std::process::exit(0);
}

#[cfg(target_os = "linux")]
/// Loads the list of available disks dynamically on Linux using the `lsblk` command.
///
/// # Returns
/// * `Result<Vec<String>, Box<dyn Error>>` - A vector of dynamically fetched disk names or an error if the command fails.
pub fn load_disks() -> Result<Vec<String>, Box<dyn Error>> {
    use std::process::Command;

    // Use `lsblk` to list available disks on Linux
    let output = Command::new("lsblk")
        .arg("-dn") // -d: Do not print children, -n: No headings
        .arg("-o")
        .arg("NAME")
        .output()?;

    // Check if the command was successful
    if output.status.success() {
        // Parse the output into a vector of disk names
        let disks = String::from_utf8(output.stdout)?
            .lines()
            .map(String::from) // Convert each line to a String
            .collect();

        Ok(disks)
    } else {
        // If the command failed, return an error
        Err(Box::from("Failed to list disks"))
    }
}

#[cfg(not(target_os = "linux"))]
/// Returns a sample list of disks on non-Linux systems.
///
/// This is a fallback for systems where disk listing is not supported (e.g., macOS, Windows).
///
/// # Returns
/// * `Result<Vec<String>, Box<dyn Error>>` - A vector of sample disk names.
pub fn load_disks() -> Result<Vec<String>, Box<dyn Error>> {
    // Return a sample list of disks for non-Linux systems
    Ok(vec![
        String::from("/dev/sda"),
        String::from("/dev/sdb"),
        String::from("/dev/sdc"),
    ])
}

/// Loads the list of available images for installation.
///
/// # Returns
/// * `Vec<String>` - A vector of strings representing available images.
pub fn load_images() -> Vec<String> {
    // In this case, we'll return a hardcoded list, but this can be replaced with dynamic data
    vec![
        String::from("Ubuntu 20.04"),
        String::from("Fedora 34"),
        String::from("Arch Linux"),
    ]
}