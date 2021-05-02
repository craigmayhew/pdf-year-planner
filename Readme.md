# PDF Year Planner

This is a rust project that generates pdf year planners full of linked pages for devices such as the Remarkable.

Year                     |  Days                   |  Tasks                  |  Notes
-------------------------|-------------------------|-------------------------|-------------------------
<img src="https://github.com/craigmayhew/pdf-year-planner/blob/main/imgs/example_year.svg" width="200" />  |  <img src="https://github.com/craigmayhew/pdf-year-planner/blob/main/imgs/example_day.svg" width="200" />  |  <img src="https://github.com/craigmayhew/pdf-year-planner/blob/main/imgs/example_tasks.svg" width="200" />  |  <img src="https://github.com/craigmayhew/pdf-year-planner/blob/main/imgs/example_notes.svg" width="200" />

## Where Can I download the latest PDF year planner?
https://github.com/craigmayhew/pdf-year-planner/releases/latest/download/2021.pdf

## Developers
Generate your own, specifying the year you require:
```
rustup update
git clone git@github.com:craigmayhew/pdf-year-planner.git
cd pdf-year-planner
cargo build --release && ./target/release/pdf_year_planner 2021
```

## Developers FAQ
Q: Any system dependencies?
A: Yes. The wkhtmltopdf OS package.

Q: I'm on ubuntu and I am unable to install wkhtmltopdf
A: Download the package from this git repo and install from there:
```sh
wget https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6-1/wkhtmltox_0.12.6-1.focal_amd64.deb
sudo apt install ./wkhtmltox_0.12.6-1.focal_amd64.deb
rm wkhtmltox_0.12.6-1.focal_amd64.deb
```

Please check https://github.com/craigmayhew/pdf-year-planner/issues for feature requests and known issues.
