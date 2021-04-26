use wkhtmltopdf::*;

fn main() -> std::io::Result<()> {
    let html = r##"
        <html>
            <head></head>
            <body>
                <h1>Rust can haz PDFs</h1>
                <a href="#page3">Link Text</a>
                <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png">
                <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png">
                <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png">
                <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png">
                <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png">
                <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png">
                <div name="page3">
                    <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png">
                </div>
            </body>
        </html>
    "##;
    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .orientation(Orientation::Portrait)
        .page_size(PageSize::A4)
        .margin(Size::Millimeters(5))
        .title("2021 Year Planner")
        .build_from_html(&html)
        .expect("failed to build pdf");
    pdfout.save("2021.pdf").expect("failed to save foo.pdf");
    Ok(())
}
