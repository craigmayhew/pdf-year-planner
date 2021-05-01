use wkhtmltopdf::*;

fn generate_html_style() -> String {
    r##"
    <style>
        html,body,table,tr,td {margin: 0px; padding: 0px;}
        a,div,td,th {color: #000; font-size: 9px; text-decoration: none;}

        table, div.page {page-break-after: always;}
        table.year tr td {padding: 1mm; background: #999; height: 5mm; width: 5mm;}
        table.day tr td {padding: 3mm; background: #eee; height: 297mm; width: 210mm;}
 
        div.page {padding: 3mm; height: 297mm; width: 220mm;}

        div.header div.year {padding: 5mm 60mm 2mm 20mm;}
        div.header div.year a {font-size: 20mm;}
        div.tabs_top div.tab a {font-size: 5mm;}

        div.tabs_side {padding: 25mm 2mm 0mm 0mm;}
        div.tabs_side div.tab {background: #ccc; margin: 2mm; text-align: center; height: 20mm; width: 10mm; border-radius: 2mm 0mm 0mm 2mm;}
        div.tabs_side div.tab div {font-size: 8mm; padding: 8mm 0mm 6mm 0mm; -webkit-transform: rotate(270deg); -webkit-transform-origin: center bottom auto;}

        div.tabs_top div.tab {background: #ccc; display: inline-block; padding: 2mm; text-align: center; width: 30mm; border-radius: 2mm 2mm 0mm 0mm;}
        div.header, div.header div.year, div.tabs_top, div.tabs_side, div.page {float: left;}

        div.lines {
            padding-top: 10px;
            background-color: black;
            background-image: -webkit-repeating-linear-gradient(90deg, transparent, transparent 1px, rgba(255, 255, 255, 1) 1px, rgba(255, 255, 255, 1) 25px);
            width: 600px;
            height: 600px;
        }
        
    </style>"##.to_owned()
}

fn generate_html_page_header() -> String {
    r##"
    <div class="header">
        <div class="year"><a href="#page_year">2021</a></div>
    "##.to_owned()
    + &generate_html_tabs_top()
    + "</div>"
}

fn generate_html_tabs_top() -> String {
    r##"
    <div class="tabs_top">
        <div class="tab calendar"><a href="#page_year">Calendar</a></div>
        <div class="tab tasks"><a href="#page_tasks">Tasks</a></div>
        <div class="tab notes"><a href="#page_notes">Notes</a></div>
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
        for day in 1..32 {
            let table = generate_html_tabs_side() + &generate_html_page_header() + r##"<table class="day" name="day_"## + &month.to_string() + "_" + &day.to_string() + r##""><tr><td></td></tr></table>"##;
            html += &table;
        }
    }
    html
}

fn generate_html_tasks() -> String {
    generate_html_tabs_side() + &generate_html_page_header() + 
    r##"<div class="tasks page" name="page_tasks"><ul><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li></ul></div>"##
}

fn generate_html_notes() -> String {
    generate_html_tabs_side() + &generate_html_page_header() + r##"<div class="notes page lines" name="page_notes"></div>"##
}

fn main() -> std::io::Result<()> {
    let html = "<html><head>".to_owned() + &generate_html_style() + "</head><body>" +
                &generate_html_year() +
                &generate_html_days() +
                &generate_html_tasks() +
                &generate_html_notes() +
               "</body></html>";
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
