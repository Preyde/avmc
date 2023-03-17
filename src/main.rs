use abap_value_mapping_creator::{await_user_enter, remove_ending_cr, remove_last_if_empty};
use clipboard_win::{formats, get_clipboard, set_clipboard};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of the source variable in program
    source_var: String,
    /// The name of the destination variable in program
    dest_var: String,
    /// The type you want to map to (leave empty to use dest_var type)
    #[clap(short = 't', long, default_value = "#")]
    abap_type: String,
    /// Print values that are read from clipboard
    #[clap(short = 'v', long)]
    verbose: bool,
    /// Use the old abap syntax for systems below version 7.40
    #[clap(short = 'o', long)]
    old_syntax: bool,
    #[clap(long)]
    /// Flag to define that the source value type is number and no '' are needed
    source_type_number: bool,
    /// Flag to define that the destination value type is number and no '' are needed
    #[clap(long)]
    dest_type_number: bool,
}

fn main() {
    let args = Args::parse();

    println!("Make sure the source values are copied to clipboard");
    println!("Press Enter when done");

    await_user_enter();

    let source_val_str: String = get_clipboard(formats::Unicode).expect("To set clipboard");
    let mut source_values: Vec<&str> = source_val_str.split("\n").collect();

    let formatted_source = remove_ending_cr(&remove_last_if_empty(&mut source_values));

    if args.verbose {
        formatted_source.iter().for_each(|v| println!("{}", v));
    }

    println!("Found {} values", formatted_source.clone().iter().count());

    println!("Now copy the values you want the old ones to map with into the clipboard");
    println!("Make sure it is in the right order");
    println!("Press Enter when done");

    await_user_enter();

    let mapping_val_str: String = get_clipboard(formats::Unicode).expect("To set clipboard");
    let mut mapping_values: Vec<&str> = mapping_val_str.split("\n").collect();

    let formatted_mapping = remove_ending_cr(&remove_last_if_empty(&mut mapping_values));

    if args.verbose {
        formatted_mapping.iter().for_each(|v| println!("{}", v));
    }

    if formatted_source.len() != formatted_mapping.len() {
        println!("There is an inconsistent number of source values and mapping values!!!")
    }

    let mapping = if !args.old_syntax {
        let mut mapping = String::from(format!(
            "{} = SWITCH {}( {}",
            args.dest_var, args.abap_type, args.source_var
        ));

        let mut i = 0;

        loop {
            mapping.push_str(&format!(
                "\n    WHEN '{}' THEN '{}'",
                formatted_source[i], formatted_mapping[i]
            ));

            i = i + 1;
            if i == formatted_source.len() {
                break;
            }
        }
        mapping.push_str(" ).");
        mapping
    } else {
        let mut mapping = String::from(format!("CASE {}.", args.source_var));

        let mut i = 0;

        loop {
            mapping.push_str(&format!("\n  WHEN '{}'.", formatted_source[i]));
            mapping.push_str(&format!(
                "\n    {} = '{}'.",
                args.dest_var, formatted_mapping[i]
            ));

            i = i + 1;
            if i == formatted_source.len() {
                break;
            }
        }

        mapping
    };

    set_clipboard(formats::Unicode, &mapping).unwrap();
    println!("{}", mapping);
    println!("Copied above content to clipboard");
}
