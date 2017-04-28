# ACRA Collector

A small, simple ACRA backend written in Rust.

It does the following:

- Append report to `crashes.txt` file
- Send an e-mail with the stack trace

## Configuration

Create a `config.json` with the following fields:

- `host`: String, the host to listen on
- `port`: Integer, the port to listen on
- `email_from`: String
- `email_to`: String
- `smtp_host`: String
- `smtp_port`: Integer
- `smtp_user`: String
- `smtp_pass`: String

The SMTP server must support STARTTLS.
