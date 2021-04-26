# PDF Year Calendar

This is a rust project that generates pdf year planners full of linked pages for devices such as the Remarkable.

PDF downloads available from here: 


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
