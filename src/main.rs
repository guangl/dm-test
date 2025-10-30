use std::error::Error;
use std::path::PathBuf;

use tokio::main as tokio_main;

use dm_test::SqlLogicTestRunner;

#[tokio_main]
async fn main() -> Result<(), Box<dyn Error>> {
    let test_dir = PathBuf::from("tests");

    SqlLogicTestRunner::run_dir(&test_dir)?;

    Ok(())
}
