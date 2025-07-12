use crate::cli::OptionsArgs;
use crate::config::{load_config, save_config, SortOrder};
use anyhow::Result;
use colored::*;
use inquire::{Confirm, Select, Text};

pub fn execute(_args: OptionsArgs) -> Result<()> {
    let mut config = load_config()?;

    let options_map = vec![
        ("showIds", "Controls if IDs are shown in lists"),
        ("confirm_command", "Asks for y/N on destructive actions"),
        ("orderBy", "Controls the sort order for 'snap list'"),
        // --- START: NEW OPTION IN MAP ---
        ("editUpdatesTimestamp", "Controls if editing a snapshot updates its timestamp"),
        ("listLimit", "Sets how many snapshots to show with 'snap list' (e.g., 5, 10, all)"),
        // --- END: NEW OPTION IN MAP ---
    ];

    let display_options: Vec<String> = options_map
        .iter()
        .map(|(key, desc)| {
            let current_value_str = match *key {
                "showIds" => config.options.show_ids.to_string(),
                "confirm_command" => config.options.confirm_command.to_string(),
                "orderBy" => format!("{:?}", config.options.order_by),
                // --- START: DISPLAY LOGIC FOR NEW OPTION ---
                "editUpdatesTimestamp" => config.options.edit_updates_timestamp.to_string(),
                "listLimit" => config.options.list_limit.clone(),
                // --- END: DISPLAY LOGIC FOR NEW OPTION ---
                _ => "Unknown".to_string(),
            };
            format!("{:<24} - {} (current: {})", key, desc, current_value_str.cyan())
        })
        .collect();

    let choice = Select::new("Select option to change:", display_options).prompt()?;

    let key_to_change = options_map
        .iter()
        .find(|(key, _)| choice.starts_with(key))
        .map(|(key, _)| *key)
        .unwrap();

    let mut changed = false;

    match key_to_change {
        "showIds" => {
            let current = config.options.show_ids;
            let new = Confirm::new("Show snapshot IDs in lists?").with_default(current).prompt()?;
            if current != new {
                config.options.show_ids = new;
                changed = true;
            }
        },
        "confirm_command" => {
            let current = config.options.confirm_command;
            let new = Confirm::new("Confirm destructive commands (delete, update)?").with_default(current).prompt()?;
            if current != new {
                config.options.confirm_command = new;
                changed = true;
            }
        },
        "orderBy" => {
            let current = config.options.order_by;
            let choices = vec!["Timestamp (default)", "Label"];
            let prompt = Select::new("Choose the sort order for 'snap list':", choices)
                .with_starting_cursor(if current == SortOrder::Label { 1 } else { 0 })
                .prompt()?;

            let new = if prompt == "Label" { SortOrder::Label } else { SortOrder::Timestamp };

            if current != new {
                config.options.order_by = new;
                changed = true;
            }
        },
        // --- START: UI LOGIC FOR NEW OPTION ---
        "editUpdatesTimestamp" => {
            let current = config.options.edit_updates_timestamp;
            let new = Confirm::new("Update a snapshot's timestamp when editing it?")
                .with_help_message("Default is No, which preserves the original creation date.")
                .with_default(current)
                .prompt()?;

            if current != new {
                config.options.edit_updates_timestamp = new;
                changed = true;
            }
        },
         // --- END: UI LOGIC FOR NEW OPTION ---
        "listLimit" => {
            let current = &config.options.list_limit;
            let validator = |input: &str| {
                if input.eq_ignore_ascii_case("all") {
                    return Ok(inquire::validator::Validation::Valid);
                }
                match input.parse::<usize>() {
                    Ok(n) if n > 0 => Ok(inquire::validator::Validation::Valid),
                    _ => Ok(inquire::validator::Validation::Invalid("Must be a positive number or 'all'".into())),
                }
            };
            let new = Text::new("Set the default number of snapshots to list:")
                .with_default(current)
                .with_validator(validator)
                .prompt()?;
            if current != &new {
                config.options.list_limit = new;
                changed = true;
            }
        },
        // --- END: UI LOGIC FOR NEW OPTION ---
        _ => unreachable!(),
    };

    if !changed {
        println!("\n{}", "[snap] No change was made.".yellow());
        return Ok(());
    }

    save_config(&config)?;
    println!(
        "\n{}",
        format!("[snap] Configuration saved successfully.").green()
    );

    Ok(())
}