use clap::Parser;
use cli_salad::create_fruit_salad;
use std::io;


#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Matthew Holden <matthew.holden@duke.edu>",
    about = "Create a fruit salad. Rerun the fruit salad generator until the user is happy."
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    loop {
        let opts: Opts = Opts::parse();

        // Get the number of fruits the user requested
        let num_fruits = opts.number;

        // Create the fruit salad
        let fruit_salad = create_fruit_salad(num_fruits);

        // Print the fruit salad in human readable format with a count of fruits used
        println!(
            "Created Fruit salad with {} fruits: {:?}",
            num_fruits,
            fruit_salad
        );

        // Ask the user if they like the salad
        println!("Do you like the salad? (Yes/No):");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        // Trim and convert the input to lowercase to handle variations in user input
        let user_input = user_input.trim().to_lowercase();

        // Check if the user likes the salad, and either end the loop or continue
        if user_input == "yes" {
            break; // Exit the loop if the user likes the salad
        } else if user_input != "no" {
            println!("Invalid response. Please enter 'Yes' or 'No'.");
        }
    }
}

