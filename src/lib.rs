use rusqlite::{Connection, Result};

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

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS kenpom_stats (
            year INTEGER,
            rank INTEGER,
            team TEXT,
            conference TEXT,
            wins INTEGER,
            losses INTEGER,
            seed INTEGER,
            pyth REAL,
            adjust_o REAL,
            adjust_o_rank INTEGER,
            adjust_d REAL,
            adjust_d_rank INTEGER,
            adjust_t REAL,
            adjust_t_rank INTEGER,
            luck REAL,
            luck_rank INTEGER,
            sos_pyth REAL,
            sos_pyth_rank INTEGER,
            sos_opp_o REAL,
            sos_opp_o_rank INTEGER,
            sos_opp_d REAL,
            sos_opp_d_rank INTEGER,
            nc_sos_pyth REAL,
            nc_sos_pyth_rank INTEGER
        )",
        [],
    )?;
    Ok(())
}
