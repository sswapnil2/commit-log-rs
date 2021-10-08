# commit-log-rs
Created a basic in memory logger in rust. Used Actix to server it using rest api. 

# Getting Started

## Install Rust
Follow the instructions in this link to install [rust](https://www.rust-lang.org/tools/install)
After installing cargo tool chain, you can run the application

# How to Run the application
In your terminal run `cargo run`

# Endpoints to test the application

1) `/health`
Health endpoint to test the application

2) `/append`
Append endpoint add the log to inmemory logger and return an offset

3) `/get`
Returns the log given the offset

# Further Scope of Improvement
Store the log data in bytes to a file using buffer reader.


