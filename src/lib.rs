use rusqlite::{Connection, Result, params};
use csv::Reader;
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TeamStat {
    pub year: String,
    pub rank: String,
    pub team: String,
    pub conference: String,
    pub wins: String,
    pub losses: String,
    pub seed: String,
    pub pyth: String,
    pub adjust_o: String,
    pub adjust_o_rank: String,
    pub adjust_d: String,
    pub adjust_d_rank: String,
    pub adjust_t: String,
    pub adjust_t_rank: String,
    pub luck: String,
    pub luck_rank: String,
    pub sos_pyth: String,
    pub sos_pyth_rank: String,
    pub sos_opp_o: String,
    pub sos_opp_o_rank: String,
    pub sos_opp_d: String,
    pub sos_opp_d_rank: String,
    pub nc_sos_pyth: String,
    pub nc_sos_pyth_rank: String,
}

pub fn create_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Drop kenpom_stats table if it exists
    conn.execute("DROP TABLE IF EXISTS kenpom_stats", [])?;

    // Read csv file
    let mut reader: Reader<File> = Reader::from_path("kenpom.csv")?;

    // Retrieve column names from csv file
    let (col_names, _) = extract_column_info(&mut reader)?;

    // Create kenpom_stats table with column names
    let query = format!("CREATE TABLE IF NOT EXISTS kenpom_stats ({})", col_names);
    conn.execute(&query, [])?;

    // Result if successful
    Ok(())
}

pub fn insert_data(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Save iterator as reader
    let mut reader: Reader<File> = Reader::from_path("kenpom.csv")?;

    // Retrieve column info
    let (col_names, num_cols) = extract_column_info(&mut reader)?;

    // Create a list of "?" placeholders with the specified number of columns
    let placeholders: String = vec!["?".to_string(); num_cols].join(", ");

    // SQL query with the placeholders
    let query = format!("INSERT INTO kenpom_stats ({}) VALUES ({});", col_names, placeholders);

    // Begin unchecked transaction
    let tx = conn.unchecked_transaction()?;

    // Loop through reader.records()
    for result in reader.records() {
        match result {
            Ok(record) => {
                // Use TeamStat struct to represent new row of data
                let data = TeamStat {
                    year: record[0].to_string(),
                    rank: record[1].to_string(),
                    team: record[2].to_string(),
                    conference: record[3].to_string(),
                    wins: record[4].to_string(),
                    losses: record[5].to_string(),
                    seed: record[6].to_string(),
                    pyth: record[7].to_string(),
                    adjust_o: record[8].to_string(),
                    adjust_o_rank: record[9].to_string(),
                    adjust_d: record[10].to_string(),
                    adjust_d_rank: record[11].to_string(),
                    adjust_t: record[12].to_string(),
                    adjust_t_rank: record[13].to_string(),
                    luck: record[14].to_string(),
                    luck_rank: record[15].to_string(),
                    sos_pyth: record[16].to_string(),
                    sos_pyth_rank: record[17].to_string(),
                    sos_opp_o: record[18].to_string(),
                    sos_opp_o_rank: record[19].to_string(),
                    sos_opp_d: record[20].to_string(),
                    sos_opp_d_rank: record[21].to_string(),
                    nc_sos_pyth: record[22].to_string(),
                    nc_sos_pyth_rank: record[23].to_string()
                };
                
                // Execute query - unchecked transaction allows commit to happen outside of for loop
                tx.execute(&query, params![&data.year, &data.rank, &data.team, &data.conference, 
                    &data.wins, &data.losses, &data.seed, &data.pyth, &data.adjust_o, &data.adjust_o_rank, 
                    &data.adjust_d, &data.adjust_d_rank, &data.adjust_t, &data.adjust_t_rank, &data.luck, 
                    &data.luck_rank, &data.sos_pyth, &data.sos_pyth_rank, &data.sos_opp_o, &data.sos_opp_o_rank, 
                    &data.sos_opp_d, &data.sos_opp_d_rank, &data.nc_sos_pyth, &data.nc_sos_pyth_rank])?;
            },
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        };
    };

    // Commit all rows to kenpom.db at once
    tx.commit()?;

    //Result if successful
    Ok(())

}

pub fn extract_column_info(reader: &mut Reader<File>) -> Result <(String, usize), csv::Error> {
    // Read the header row to get column names
    let header_record = reader.headers()?.clone();

    // Extract column names and number of columns
    let column_names: Vec<String> = header_record.iter().map(|name| name.to_string()).collect();
    let num_columns = column_names.len();

    // Format column names into a single string - sql compatible
    let column_names = column_names.join(", ");

    // Result if successful
    Ok((column_names, num_columns))
}

pub fn sql_query(conn: &Connection, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare query from user input
    let mut stmt = conn.prepare(query)?;

    // Execute query
    let mut rows = stmt.query([])?;

    // Print first 20 rows from query
    let mut count = 0;
    println!("\n");
    while let Some(row) = rows.next()? {
        println!("{:?}", row);
        count += 1;
        
        if count >= 20 {
            println!("Response limited to 20 lines");
            break;
        }
    }
    println!("\n");

    //Result if successful
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Result;

    #[test]
    fn test_create_table() -> Result<(), Box<dyn std::error::Error>>  {
        let conn = Connection::open_in_memory()?;
        create_table(&conn)?;
        let table_names: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>>>()?;
        assert_eq!(table_names, vec!["kenpom_stats"]);
        Ok(())
    }

    #[test]
    fn test_insert_data() -> Result<(), Box<dyn std::error::Error>>  {
        let conn = Connection::open_in_memory()?;
        create_table(&conn)?;
        insert_data(&conn)?;
        let table_length: i64 = conn
        .query_row("SELECT COUNT(*) FROM kenpom_stats", [], |row| row.get(0))?;
        assert_eq!(table_length, 5453i64);
        Ok(())
    }
}