use std::path::PathBuf;

#[tokio::main]
async fn main() {
    println!("Running single sqllogictest file: tests/datatype/bigint/01_bigint_basic.test");
    let mut tester = sqllogictest::Runner::new(|| async {
        let mut db = dm_test::DmDatabase::new();
        // using local default credentials from README
        match db.connect("SYSDBA", "DMDBA_hust4400", "localhost:5236") {
            Ok(()) => println!("Connected to DM"),
            Err(e) => eprintln!("Connect error: {}", e),
        }
        Ok(db)
    });

    let file = PathBuf::from("tests/datatype/bigint/01_bigint_basic.test");
    match tester.run_file(file) {
        Ok(res) => println!("Run finished: {:?}", res),
        Err(e) => eprintln!("Runner error: {}", e),
    }
}
