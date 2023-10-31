extern crate kenpoms_analysis;
use kenpoms_analysis::{create_table, insert_data, sql_query};
use rusqlite::{Connection, Error};
use std::io;

fn establish_connection() -> Result<Connection, Error> {
    // Establish database connection
    let conn = Connection::open("kenpom.db")?;

    //Result if successful
    Ok(conn)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Establish connection
    let conn: Connection = establish_connection()?;

    // Create table
    create_table(&conn)?;

    // Initial data dump into rusqlite db
    insert_data(&conn)?;

    // Establish count for loop
    let mut count = 0;

    // Loop 10 times for 10 sql queries
    while count < 10 {

        count += 1;

        // Print prompt
        let intro = format!("({} of 10) Enter a SQL Query or type 'Exit': ", count);
        println!("{}", intro);

        //read user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        //Option: Exit
        if user_input.trim() == "Exit" {
            break;
        };

        // Execute SQL query and handle exception
        match sql_query(&conn, &user_input) {
            Ok(_) => {
                println!("\nQuery executed successfully.\n");
                continue
            },
            Err(err) => {
                eprintln!("Error {} \n", err);
                println!("An error occurred with the previous query. Please try again \n");
                continue;
            }
        };
    }

    // Exit Statement
    println!("\nExiting College Basketball Stats CLI\n");

    //Result if successful
    Ok(())

}