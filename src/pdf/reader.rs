use pdfium_render::prelude::*;


pub fn read_pdf(path: &str) -> String{
  let bindings = Pdfium::bind_to_library("./libpdfium.so").unwrap();
  let pdfium = Pdfium::new(bindings);


  let document = pdfium.load_pdf_from_file(path, None).unwrap();
  let mut whole_text = String::new();

  for page in document.pages().iter() {
    let page_text = page.text().unwrap().all();
    whole_text.push_str(&page_text);
  }
  whole_text
}