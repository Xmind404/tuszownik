mod pdf;

fn main() {
    println!("Hello, world!");
    println!("{}", pdf::reader::read_pdf("sample.pdf"));
}
