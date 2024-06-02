# nginx-windows-updater
Simple Application written in Rust that automatically updates nginx on windows

## Info

* Assumes Nginx in installed at C:\nginx\nginx.exe

* Assumes you have a Windows Service named "Nginx"

* Checks Latest Version of Nginx

* Checks Locally Install Version of Nginx

* Will Stop Nginx Service -> Download latest nginx.zip -> Replaces nginx.exe -> Starts Nginx Service

* Checks for updates on launch then every hour
