use wkhtmltopdf::*;

fn generate_html_style() -> String {
    r##"
    <style>
        html,body {margin: 0px; padding: 0px;}
        a,div,td,th {color: #000; font-size: 9px; text-decoration: none;}

        table {page-break-after: always;}
        table.year tr td {padding: 1mm; background: #999; height: 5mm; width: 5mm;}
        table.day tr td {padding: 3mm; background: #eee; height: 297mm; width: 210mm;}

        div.header div.year {font-size: 20mm; padding: 5mm 60mm 2mm 20mm;}
        div.tabs_top div.tab {background: #ccc; border: 1mm #fff solid; display: inline-block; padding: 2mm; text-align: center; width: 30mm;}

        div.tabs_side {padding: 25mm 2mm 0mm 0mm;}
        div.tabs_side div.tab {background: #ccc; margin: 2mm; text-align: center; height: 20mm; width: 10mm;}
        div.tabs_side div.tab div {font-size: 8mm; padding: 8mm 0mm 6mm 0mm; -webkit-transform: rotate(270deg); -webkit-transform-origin: center bottom auto;}

        div.tabs_top div.tab {background: #ccc; border: 1mm #fff solid; display: inline-block; padding: 2mm; text-align: center; width: 30mm;}
        div.header, div.header div.year, div.tabs_top, div.tabs_side {float: left;}
    </style>"##.to_owned()
}

fn generate_html_page_header() -> String {
    r##"
    <div class="header">
        <div class="year">2021</div>
    "##.to_owned()
    + &generate_html_tabs_top()
    + "</div>"
}

fn generate_html_tabs_top() -> String {
    r##"
    <div class="tabs_top">
        <div class="tab calendar">Calendar</div>
        <div class="tab tasks">Tasks</div>
        <div class="tab notes">Notes</div>
    </div>
    "##.to_owned()
}

fn generate_html_tabs_side() -> String {
    r##"
    <div class="tabs_side">
        <div class="tab jan"><div>JAN</div></div>
        <div class="tab feb"><div>FEB</div></div>
        <div class="tab mar"><div>MAR</div></div>
        <div class="tab apr"><div>APR</div></div>
        <div class="tab may"><div>MAY</div></div>
        <div class="tab jun"><div>JUN</div></div>
        <div class="tab jul"><div>JUL</div></div>
        <div class="tab aug"><div>AUG</div></div>
        <div class="tab sep"><div>SEP</div></div>
        <div class="tab oct"><div>OCT</div></div>
        <div class="tab nov"><div>NOV</div></div>
        <div class="tab dec"><div>DEC</div></div>
    </div>
    "##.to_owned()
}

fn generate_html_year() -> String {
    let mut html: String = r##"<table id="page_year" class="year" name="year">"##.to_owned();
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
    html += "</table>";
    
    generate_html_tabs_side() + &generate_html_page_header() + &html
}

fn generate_html_days() -> String {
    let mut html: String = "".to_owned();
    for month in 1..13 {
        html += "<tr>";
        for day in 1..32 {
            let table = generate_html_tabs_side() + &generate_html_page_header() + r##"<table class="day" name="day_"## + &month.to_string() + "_" + &day.to_string() + r##""><tr><td></td></tr></table>"##;
            html += &table;
        }
    }
    html
}

fn main() -> std::io::Result<()> {
    let html = "<html><head>".to_owned() + &generate_html_style() + "</head><body>" + &generate_html_year() + &generate_html_days() + "</body></html>";
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
