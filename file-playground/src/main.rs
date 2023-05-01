#![deny(clippy::all)]
use std::path::Path;

fn main() {
    // let bangla_path = Path::new("font/noto_serif_bengali/static/NotoSerifBengali/");
    // let bangla_name = "NotoSerifBengali";
    let english_path = Path::new("font/static/");
    let english_name = "RedHatMono";

    //? Load a font from the file system
    let font_family = genpdf::fonts::from_files(&english_path, &english_name, None)
        .expect("Failed to load font family");

    //? Create a document and set the default font family
    let mut doc = genpdf::Document::new(font_family);

    //? Change the default settings
    doc.set_title("Demo Document");

    //? Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    //? Add one or more elements
    doc.push(genpdf::elements::Paragraph::new("This is a Paragraph"));

    //? Render the document and write it to a file
    doc.render_to_file("files/output.pdf")
        .expect("Failed to generate pdf");
}
