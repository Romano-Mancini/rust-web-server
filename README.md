# Rust web server

Multithreaded web server built from scratch in Rust, for learning purposes. Every line of code is hand-written, meaning I did not paste it from an LLM.

## Features

Fixed size thread pool of 5 workers. It serves static HTML files and supports graceful shutdown.

| Route     | Status        | Response         | Behavior                                                       |
| --------- | ------------- | ---------------- | -------------------------------------------------------------- |
| `/`       | 200 OK        | `index.html`     | Home page.                                                     |
| `/sleep`  | 200 OK        | `sleep.html`     | Waits 5 seconds before responding, to simulate a slow request. |
| `/kill`   | 200 OK        | `kill.html`      | Responds, then shuts the server down gracefully.               |
| any other | 404 Not Found | `not_found.html` | Fallback for unknown paths.                                    |

## Limitations to address in the future

- Several calls to `unwrap()`, which might cause the program to panic;
- no keep-alive;
- only supports GET

## How to run

Clone the repository, then simply `cargo run` inside the package folder. Then, open `localhost:8080` on a web browser.

## Inspiration

Inspired by the last chapter of [The Rust Programming Language](https://doc.rust-lang.org/book/) book.
