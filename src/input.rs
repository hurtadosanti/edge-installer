/// The `input` module is responsible for interacting with the user via the terminal.
/// It provides functions for displaying prompts and receiving user input.
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use tokio::time::{self, Duration};

/// Displays a confirmation prompt with a customizable timeout.
///
/// If the user does not respond within the timeout, the default value (`true`) is returned.
///
/// # Arguments
/// * `prompt_text` - The message to display in the prompt.
/// * `timeout_secs` - The number of seconds to wait before proceeding with the default choice.
///
/// # Returns
/// * `bool` - Returns `true` if "Yes" is selected or timeout occurs, otherwise `false`.
pub async fn prompt_with_timeout(prompt_text: &str, timeout_secs: u64) -> bool {
    let prompt_text = prompt_text.to_string();

    let (tx, rx) = tokio::sync::oneshot::channel();
    let prompt_task = tokio::spawn(async move {
        let result = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt_text)
            .default(true)
            .interact()
            .unwrap_or(true);

        let _ = tx.send(result);
    });

    match time::timeout(Duration::from_secs(timeout_secs), rx).await {
        Ok(Ok(result)) => result,
        _ => {
            prompt_task.abort();
            println!("\nNo response received in {} seconds. Proceeding with default.", timeout_secs);
            true
        }
    }
}

/// Displays a selection menu for disks and returns the index of the selected disk.
///
/// # Arguments
/// * `disks` - A list of disk options to display.
///
/// # Returns
/// * `usize` - The index of the selected disk.
pub fn select_disk_prompt(disks: &[&str]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a disk for installation")
        .items(disks)
        .default(0)
        .interact()
        .unwrap()
}
/// Displays a selection menu for images and returns the index of the selected image.
///
/// # Arguments
/// * `images` - A list of image options to display.
///
/// # Returns
/// * `usize` - The index of the selected image.
pub fn select_image_prompt(images: &[&str]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an image to install")
        .items(images)
        .default(0)
        .interact()
        .unwrap()
}
