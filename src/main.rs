use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use std::fs::File;
use std::io::{self, Write, stdout};
use std::time::{Duration, Instant};
use console::{Key, Term};
use std::thread::sleep;

fn main() -> io::Result<()> {
    let disks = vec!["Disk 1: /dev/sda", "Disk 2: /dev/sdb", "Disk 3: /dev/sdc"];
    let images = vec!["Image 1: Ubuntu 20.04", "Image 2: Fedora 34", "Image 3: Arch Linux"];
    let mut output_file = File::create("installation_output.txt")?;
    let mut selected_disk = 0;  // Default to the first disk
    let mut selected_image = 0; // Default to the first image

    loop {
        writeln!(output_file, "Using default configuration: {}, {}", disks[selected_disk], images[selected_image])?;
        println!("Proceeding with default configuration: {}, {}", disks[selected_disk], images[selected_image]);

        if wait_for_configuration_change(10, &mut output_file)? {
            if let Ok((disk, image)) = modify_configuration(&disks, &images, &mut output_file) {
                selected_disk = disk;
                selected_image = image;
            }
        }

        writeln!(output_file, "Writing {} to {}...", images[selected_image], disks[selected_disk])?;
        println!("\nInstallation complete.");

        if !handle_post_installation_action()? {
            break;
        }
    }

    Ok(())
}

fn wait_for_configuration_change(seconds: u64, output_file: &mut File) -> io::Result<bool> {
    let term = Term::stdout();
    let start_time = Instant::now();

    for i in (1..=seconds).rev() {
        print!("\rProceeding in {} seconds... Press 'm' to modify configuration.", i);
        stdout().flush()?;
        sleep(Duration::from_secs(1));

        if let Ok(key) = term.read_key() {
            if key == Key::Char('m') {
                println!("\nModifying installation configuration...");
                writeln!(output_file, "User chose to modify the configuration.")?;
                return Ok(true);
            }
        }

        if start_time.elapsed().as_secs() >= seconds {
            break;
        }
    }

    Ok(false)
}

fn post_installation_action() -> io::Result<String> {
    let actions = vec!["Shutdown", "Restart", "Exit"];
    println!("Choose an action to take:");

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action to take:")
        .items(&actions)
        .default(0)
        .interact_on(&Term::stderr())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(actions[selection].to_string())
}

fn handle_post_installation_action() -> io::Result<bool> {
    let action = post_installation_action()?;
    match action.as_str() {
        "Shutdown" => {
            println!("Shutting down...");
            Ok(false)
        }
        "Restart" => {
            println!("Restarting...");
            Ok(false)
        }
        "Exit" => {
            println!("Exiting...");
            Ok(false)
        }
        _ => unreachable!(),
    }
}

fn modify_configuration(disks: &[&str], images: &[&str], output_file: &mut File) -> io::Result<(usize, usize)> {
    let selected_disk = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a target disk")
        .items(disks)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let selected_image = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an image to install")
        .items(images)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to write to disk? (y/n)")
        .default(true)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    {
        writeln!(output_file, "Confirmed: Writing {} to {}...", images[selected_image], disks[selected_disk])?;
        Ok((selected_disk, selected_image))
    } else {
        writeln!(output_file, "Operation canceled by user.")?;
        println!("Operation canceled.");
        Err(io::Error::new(io::ErrorKind::Other, "Operation canceled by user"))
    }
}
