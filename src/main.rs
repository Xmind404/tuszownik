mod pdf;
mod detection;

fn main() {
    println!("Hello, world!");
    println!("{}", pdf::reader::read_pdf("sample.pdf"));
}
