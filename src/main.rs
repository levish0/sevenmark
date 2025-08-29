use sevenmark::parse_document;
use std::fs;
use std::time::Instant;

fn main() {
    let input_content = fs::read_to_string("ToParse.txt").expect("ToParse.txt file not found");
    let document_len = input_content.len();

    println!("Input ({} bytes):\n{}\n", document_len, "=".repeat(50));

    let start_time = Instant::now();
    let result = parse_document(&input_content);
    let duration = start_time.elapsed();

    println!("Parsed {} elements in {:?}", result.len(), duration);

    let json_output = serde_json::to_string_pretty(&result).unwrap();
    // println!("JSON Output:\n{}", json_output);

    fs::write("ParseResult.json", &json_output).ok();
    println!("\nResult saved to ParseResult.json");
    println!(
        "Performance: {:.2} KB/s",
        document_len as f64 / 1024.0 / duration.as_secs_f64()
    );
}
