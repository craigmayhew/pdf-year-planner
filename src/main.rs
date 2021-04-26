use wkhtmltopdf::*;

fn generate_html_style() -> String {
    r##"
    <style>
        table {page-break-after: always;}
        table.year tr td {padding: 3px; background: #999;}
        table.day tr td {padding: 3px; background: #999; height: 20cm; width: 15cm;}
    </style>"##.to_owned()
}

fn generate_html_table_year() -> String {
    let mut html: String = "".to_owned();
    for month in 1..13 {
        html += "<tr>";
        for day in 1..32 {
            let link = "<a href=\"#day_".to_owned() + &month.to_string() + "_" + &day.to_string() + "\">" + &day.to_string() + "</a>";
            html += "<td>";
            html += &link;
            html += "</td>";
        }
        html += "</tr>";
    }
    
    r##"<table class="year" name="year">"##.to_owned() + &html + "</table>"
}

fn generate_html_table_days() -> String {
    let mut html: String = "".to_owned();
    for month in 1..13 {
        html += "<tr>";
        for day in 1..32 {
            let table = r##"<table class="day" name="day_"##.to_owned() + &month.to_string() + "_" + &day.to_string() + r##""><tr><td></td></tr></table>"##;
            html += &table;
        }
    }
    html
}

fn main() -> std::io::Result<()> {
    let html = "<html><head>".to_owned() + &generate_html_style() + "</head><body>" + &generate_html_table_year() + &generate_html_table_days() + "</body></html>";
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
