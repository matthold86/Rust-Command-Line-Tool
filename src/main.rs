extern crate kenpoms_analysis;
use kenpoms_analysis::{create_table, sql_query};
use rusqlite::{Connection, Error};
use std::io;
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Matthew Holden <matthew.holden@duke.edu>",
    about = "Create a CLI for college basketball statistics database."
)]

struct CLIQuery {
    #[clap(short, long)]
    query: String,
}

fn establish_connection() -> Result<Connection, Error> {
    let conn = Connection::open("kenpom.db")?;
    Ok(conn)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {


    let conn: Connection = match establish_connection() {
        Ok(conn) => {
            // The connection is established successfully
            println!("Connected to the KenPoms database.");
            conn
        },
        Err(err) => {
            // An error occurred when establishing the connection
            eprintln!("Error opening the database: {:?}", err);
            return Ok(())
        }
    };


    create_table(&conn)?;

    //insert_data(&conn)?;

    let mut count = 0;

    while count < 10 {

        count += 1;

        let intro = format!("({} of 10) Enter a SQL Query or type 'Exit': ", count);
        println!("{}", intro);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        if user_input == String::from("Exit") {
            break; // Exit the loop if the user likes the salad
        }

        let query: String = user_input;


        //let query = String::from("SELECT year, team, adjust_o FROM kenpom_stats WHERE team = 'Duke';");
        match sql_query(&conn, &query) {
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Error {} \n", err);
                println!("An error occurred with the previous query. Please try again \n");
                continue;
            }
        };
    }

    println!("Exiting College Basketball Stats CLI")
    
    Ok(())
}