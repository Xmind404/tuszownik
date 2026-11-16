mod pdf;
mod detection;

fn main() {
    println!("Hello, world!");
    let readed_text = pdf::reader::read_pdf("sample.pdf");
    let founded_values = detection::detector::detect_sensitive_data(&readed_text, &detection::patterns::PATTERNS);

    for (category, value) in founded_values.iter() {
        println!("{} -> {}", category.to_string(), value.to_string());
    }
}
