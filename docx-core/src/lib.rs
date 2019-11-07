mod documents;
mod types;
mod xml_builder;
mod zipper;

pub use documents::*;
pub use types::*;
pub use zipper::*;

pub fn simple() {
    let xml = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new("Hello")))
        .build();
    let path = std::path::Path::new("./test.docx");
    let file = std::fs::File::create(&path).unwrap();
    zip(file, xml);
}