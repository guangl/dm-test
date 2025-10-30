use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("tests/datatype/bigint/01_bigint_basic.test");
    println!("Reading: {}\n", path.display());

    let content = fs::read_to_string(path).expect("read file");

    // Show original content (first 200 chars)
    println!(
        "--- File head ---\n{}\n--- End head ---\n",
        &content.chars().take(400).collect::<String>()
    );

    // Our runner splits by ';' — show the chunks and their prefixes
    println!("Splitting by ';' into chunks (trimmed) and showing first 120 chars of each):\n");
    for (i, chunk) in content.split(';').enumerate() {
        let trimmed = chunk.trim();
        let preview: String = trimmed.chars().take(120).collect();
        println!("Chunk {} (len={}):\n{}\n", i + 1, trimmed.len(), preview);
    }

    println!("\nNow show the specific chunk that contains the first SQL DROP statement:\n");
    // find the chunk containing the word DROP TABLE
    for chunk in content.split(';') {
        if chunk.contains("DROP TABLE") {
            println!("--- DROP chunk ---\n{}\n--- End ---\n", chunk);
            break;
        }
    }

    println!(
        "\nConclusion:\nThe test file uses sqllogictest directives (lines like 'statement ok' and 'query II') and # comments.\nOur simple runner just splits by ';' and sends each chunk to the database. That means chunks begin with non-SQL directives or comments (e.g., starting with '#', 'statement ok'), which are not valid SQL and will be passed to the DB verbatim, causing syntax or execution errors.\nTo run these tests correctly you must parse the .test format (directives + expected output) or use the sqllogictest harness to interpret them before executing SQL.\n"
    );
}
