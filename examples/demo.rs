use dameng::Connection;
use dameng::Result;

fn main() -> Result<()> {
    let query = "SELECT '' || A FROM TEST_LG1;";

    let conn = Connection::connect("SYSDBA", "DMDBA_hust4400", "localhost")?;
    println!("Connected to the database.");

    println!("Running query: {}", query);
    match conn.query(query, &[]) {
        Ok(rows) => {
            for row_result in rows {
                match row_result {
                    Ok(row) => {
                        let result: std::result::Result<String, _> = row.get(0);
                        match result {
                            Ok(value) => println!("Result: {}", value),
                            Err(e) => println!("Error retrieving result: {}", e),
                        }
                    }
                    Err(e) => println!("Row error: {}", e),
                }
            }
        }
        Err(e) => println!("Query error: {}", e),
    }

    Ok(())
}
