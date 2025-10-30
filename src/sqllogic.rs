use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::DmDatabase;

pub struct SqlLogicTestRunner;

impl SqlLogicTestRunner {
    /// Scan a directory recursively and return a list of `.test` files.
    pub fn find_test_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut files = Vec::new();
        if !dir.exists() {
            return Ok(files);
        }

        fn visit(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit(&path, out)?;
                } else if let Some(ext) = path.extension() {
                    if ext == "test" {
                        out.push(path);
                    }
                }
            }
            Ok(())
        }

        visit(dir, &mut files)?;
        Ok(files)
    }

    pub fn run_dir(dir: &Path) -> Result<(), Box<dyn Error>> {
        let files = Self::find_test_files(dir)?;

        let mut tester = sqllogictest::Runner::new(|| async {
            let mut db = DmDatabase::new();
            db.connect("SYSDBA", "DMDBA_hust4400", "localhost:5236")
                .unwrap();
            Ok(db)
        });

        for file in files {
            // Print which test file we're about to run so we can trace runtime failures
            println!("Running test file: {}", file.display());
            let _res = tester.run_file(file).unwrap();
        }

        Ok(())
    }
}
