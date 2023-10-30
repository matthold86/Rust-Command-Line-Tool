use rusqlite::{Connection, Result, params};
use csv::Reader;
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TeamStat {
    pub year: i32,
    pub rank: i32,
    pub team: String,
    pub conference: String,
    pub wins: i32,
    pub losses: i32,
    pub seed: i32,
    pub pyth: f64,
    pub adjust_o: f64,
    pub adjust_o_rank: i32,
    pub adjust_d: f64,
    pub adjust_d_rank: i32,
    pub adjust_t: f64,
    pub adjust_t_rank: i32,
    pub luck: f64,
    pub luck_rank: i32,
    pub sos_pyth: f64,
    pub sos_pyth_rank: i32,
    pub sos_opp_o: f64,
    pub sos_opp_o_rank: i32,
    pub sos_opp_d: f64,
    pub sos_opp_d_rank: i32,
    pub nc_sos_pyth: f64,
    pub nc_sos_pyth_rank: i32,
}

pub fn create_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader: Reader<File> = Reader::from_path("kenpom.csv")?;
    let (col_names, _) = extract_column_info(&mut reader)?;
    let query = format!("CREATE TABLE IF NOT EXISTS kenpom_stats ({})", col_names);
    conn.execute(&query, [])?;
    Ok(())
}

pub fn insert_data(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Save iterator as reader
    let mut reader: Reader<File> = Reader::from_path("kenpom.csv")?;
    /*let mut reader = match Reader::from_path("kenpom.csv") {
        Ok(reader) => reader,
        Err(err) => {
            println!("Error reading file: {:?}", err);
            return Ok(());
        }
    };*/

    // Retrieve column info
    let (col_names, num_cols) = extract_column_info(&mut reader)?;

    /*let (col_names, num_cols) = match extract_column_info(&mut reader){
        Ok((column_names, num_columns)) => (column_names, num_columns),
        Err(err) => {
            eprintln!("Error extracting column names: {:?}", err);
            return Ok(());
        }
    };*/

    // Create a list of "?" placeholders with the specified number of columns
    let placeholders: String = vec!["?".to_string(); num_cols].join(", ");

    // SQL query with the placeholders
    let query = format!("INSERT INTO kenpom_stats ({}) VALUES ({});", col_names, placeholders);
    //let mut stmt = conn.prepare(&query);
    for result in reader.records() {
        match result {
            Ok(record) => {
                let year = &record[0];
                let rank = &record[1];
                let team = &record[2];
                let conference = &record[3];
                let wins = &record[4];
                let losses = &record[5];
                let seed = &record[6];
                let pyth = &record[7];
                let adjust_o = &record[8];
                let adjust_o_rank = &record[9];
                let adjust_d = &record[10];
                let adjust_d_rank = &record[11];
                let adjust_t = &record[12];
                let adjust_t_rank = &record[13];
                let luck = &record[14];
                let luck_rank = &record[15];
                let sos_pyth = &record[16];
                let sos_pyth_rank = &record[17];
                let sos_opp_o = &record[18];
                let sos_opp_o_rank = &record[19];
                let sos_opp_d = &record[20];
                let sos_opp_d_rank = &record[21];
                let nc_sos_pyth = &record[22];
                let nc_sos_pyth_rank = &record[23];
                //let new_params = [year, rank, team, conference, wins, losses, seed, pyth, adjust_o, adjust_o_rank, adjust_d, adjust_d_rank, adjust_t, adjust_t_rank, luck, luck_rank, sos_pyth, sos_pyth_rank, sos_opp_o, sos_opp_o_rank, sos_opp_d, sos_opp_d_rank, nc_sos_pyth, nc_sos_pyth_rank];
                conn.execute(&query, params![year, rank, team, conference, wins, losses, seed, pyth, adjust_o, adjust_o_rank, adjust_d, adjust_d_rank, adjust_t, adjust_t_rank, luck, luck_rank, sos_pyth, sos_pyth_rank, sos_opp_o, sos_opp_o_rank, sos_opp_d, sos_opp_d_rank, nc_sos_pyth, nc_sos_pyth_rank])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }
    Ok(())
}

pub fn extract_column_info(reader: &mut Reader<File>) -> Result <(String, usize), csv::Error> {
    // Read the header row to get column names
    let header_record = reader.headers()?.clone();

    // Extract column names and number of columns
    let column_names: Vec<String> = header_record.iter().map(|name| name.to_string()).collect();
    let num_columns = column_names.len();

    // Print the column names and number of columns
    //println!("Column Names: {:?}", column_names);
    //println!("Number of Columns: {}", num_columns);
    let column_names = column_names.join(", ");
    Ok((column_names, num_columns))
}

pub fn sql_query(conn: &Connection, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query([])?;
    let mut count = 0;
    while let Some(row) = rows.next()? {
        println!("{:?}", row);
        count += 1;
        
        if count >= 20 {
            println!("Response limited to 20 lines");
            break; // Exit the loop after printing 5 rows
        }
    }
    Ok(())
}
