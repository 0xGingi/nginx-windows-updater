# nginx-windows-updater
Simple Application written in Rust that automatically updates nginx on windows

## Info

* Assumes Nginx is installed at C:\nginx\nginx.exe

* Assumes you have a Windows Service named "Nginx"

* Checks Latest Version of Nginx

* Checks Locally Install Version of Nginx

* Will Stop Nginx Service -> Download latest nginx.zip -> Replaces nginx.exe -> Starts Nginx Service

* Checks for updates on launch then every hour


![Screenshot 2024-06-02 130055](https://github.com/0xGingi/nginx-windows-updater/assets/104647854/d9181740-1a9d-4bd6-a05f-6b06e06b85b8)
