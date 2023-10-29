extern crate kenpoms_analysis;
use kenpoms_analysis::create_table;

fn main(){
    let conn = rusqlite::Connection::open("kenpom.db").unwrap();
    create_table(&conn).unwrap();
}