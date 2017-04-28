# ACRA Collector

A small, simple ACRA backend written in Rust.

It does the following:

- Append report to `crashes.txt` file
- Send an e-mail with some crash information and the stack trace

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

## Building

You can build the program yourself with a recent version of Rust:

    cargo build --release

The binary is now at `target/release/acra-collector`.

If you want a binary compiled for Debian 8 (with OpenSSL linked in
dynamically), you can use the `build-debian.sh` script.

I don't provide static binaries since I want the binary to link against the
system OpenSSL for update security reasons. If you want, you could build your
own static binary with musl libc though.
