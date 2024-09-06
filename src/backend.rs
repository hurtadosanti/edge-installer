/// The `backend` module handles the core installation logic and shutdown processes.
/// It processes the user input collected from the TUI and performs the necessary operations.

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
