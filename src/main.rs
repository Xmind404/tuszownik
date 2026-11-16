mod pdf;
mod detection;

fn main() {
    println!("Hello, world!");
    let readed_text = pdf::reader::read_pdf("sample.pdf");
    let founded_values: Vec<detection::detector::Detection> = detection::detector::detect_sensitive_data(&readed_text, &detection::patterns::PATTERNS);

    for element in founded_values.iter() {
        println!("Type: {}, value: {}, start: {}, end: {}", element.category, element.value, element.start, element.end);
    }
}
