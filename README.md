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

## Builds

You can download binary builds for Debian 8 on the
[releases](https://github.com/dbrgn/acra-collector/releases) page.

All releases are signed with [my PGP key](https://keybase.io/dbrgn).

## Building

You can build the program yourself with a recent version of Rust:

    cargo build --release

The binary is now at `target/release/acra-collector`.

If you want a binary compiled for Debian 8 (with OpenSSL linked in
dynamically), you can use the `build-debian.sh` script.

I don't provide static binaries since I want the binary to link against the
system OpenSSL for update security reasons. If you want, you could build your
own static binary with musl libc though.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
