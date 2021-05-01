use chrono::{Date, Datelike, DateTime, NaiveDate, NaiveDateTime, Utc};
use wkhtmltopdf::*;

fn generate_html_style() -> String {
    r##"
    <style>
        html,body,table,tr,td {margin: 0px; padding: 0px;}
        a,div,td,th {color: #000; font-size: 9px; text-decoration: none;}

        div.page {page-break-after: always; padding: 3mm; height: 297mm; width: 220mm;}

        div.year_month {text-align: center; width: 50mm; height: 50mm; display: inline-block;}
        div.year_month div.title {text-align: center; width: 50mm; height: 5mm; display: inline-block;}
        div.year_month a div {background: #eee; height: 5mm; padding: 1mm; text-align: center; vertical-align: middle; width: 5mm; display: inline-block;}
        div.year_month a div.weekend {background: #aaa;}

        table.day tr td {padding: 3mm; background: #eee; height: 297mm; width: 210mm;}

        div.header div.year {padding: 5mm 60mm 2mm 20mm;}
        div.header div.year a {font-size: 20mm;}
        div.tabs_top div.tab a {font-size: 5mm;}

        div.tabs_side {padding: 25mm 2mm 0mm 0mm;}
        div.tabs_side div.tab {background: #ccc; margin: 2mm; text-align: center; height: 20mm; width: 10mm; border-radius: 2mm 0mm 0mm 2mm;}
        div.tabs_side div.tab div {font-size: 8mm; padding: 8mm 0mm 6mm 0mm; -webkit-transform: rotate(270deg); -webkit-transform-origin: center bottom auto;}

        div.tabs_top div.tab {background: #ccc; display: inline-block; padding: 2mm; text-align: center; width: 30mm; border-radius: 2mm 2mm 0mm 0mm;}
        div.header, div.header div.year, div.tabs_top, div.tabs_side, div.page {float: left;}

        div.lines {
            padding-top: 25px;
            background-color: #000;
            background-image: -webkit-repeating-linear-gradient(#fff 1px, #fff 20px, #000 20px, #000 21px);
            width: 220mm;
            height: 290mm;
        }

        ul {list-style-type: circle;}
        ul li {font-size: 12mm;}
        
    </style>"##.to_owned()
}

fn generate_html_page_header(year: &str) -> String {
    "<div class=\"header\">
        <div class=\"year\"><a href=\"#page_year\">".to_owned() + &year + "</a></div>
    "
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

fn generate_html_year(year: &str) -> String {
    let mut naive_date: NaiveDateTime = NaiveDate::from_ymd(year.parse::<i32>().unwrap(), 1, 1).and_hms(1, 1, 1);
    let mut html: String = r##"<div id="page_year" class="year page" name="year">"##.to_owned();
    for month in 1..13 {
        let mut date = DateTime::<Utc>::from_utc(naive_date.with_month(month).unwrap(), Utc);

        html += "<div class=\"year_month\">";
            html += "<div class=\"title\">";
            html += &date.format("%B").to_string();
            html += "</div>";
            for day in 1..28 {
                date = DateTime::<Utc>::from_utc(naive_date.with_month(month).unwrap().with_day(day).unwrap(), Utc);
                let formatted_date = date.weekday();
                let weekend = if formatted_date == chrono::Weekday::Sat || formatted_date == chrono::Weekday::Sun {"weekend"} else {""};
                
                let link = "<a href=\"#day_".to_owned() + &month.to_string() + "_" + &day.to_string() + "\"><div class=\""+weekend+"\">" + &day.to_string() + "</div></a>";
                html += &link;
            }
        html += "</div>";
    }
    html += "</div>";
    
    generate_html_tabs_side() + &generate_html_page_header(year) + &html
}

fn generate_html_days(year: &str) -> String {
    let mut html: String = "".to_owned();
    for month in 1..13 {
        for day in 1..32 {
            let table = generate_html_tabs_side() + &generate_html_page_header(year) + r##"<table class="day" name="day_"## + &month.to_string() + "_" + &day.to_string() + r##""><tr><td></td></tr></table>"##;
            html += &table;
        }
    }
    html
}

fn generate_html_tasks(year: &str) -> String {
    generate_html_tabs_side() + &generate_html_page_header(year) + 
    r##"<div class="tasks page" name="page_tasks"><ul><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li><li></li></ul></div>"##
}

fn generate_html_notes(year: &str) -> String {
    generate_html_tabs_side() + &generate_html_page_header(year) + r##"<div class="notes page lines" name="page_notes"></div>"##
}

fn main() -> std::io::Result<()> {
    let mut chosen_year = "2021";
    let title = chosen_year.to_string() + " Year Planner";
    let filename = chosen_year.to_string() + ".pdf";
    let error_pdf_save = "failed to save ".to_owned() + &filename;

    // generate html
    let html = "<html><head>".to_owned() + &generate_html_style() + "</head><body>" +
                &generate_html_year(chosen_year) +
                &generate_html_days(chosen_year) +
                &generate_html_tasks(chosen_year) +
                &generate_html_notes(chosen_year) +
               "</body></html>";

    // turn html into a pdf
    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .orientation(Orientation::Portrait)
        .page_size(PageSize::A4)
        .margin(Size::Millimeters(5))
        .title(&title)
        .build_from_html(&html)
        .expect("failed to build pdf");
    
    //save the pdf
    pdfout.save(&filename).expect(&error_pdf_save);
    Ok(())
}
