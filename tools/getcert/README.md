# getcert

## Reason for this project to exist

This project amis to automate the extraction of certificates from the Android Auto application.
The reason for this is perfectly explained by [Tomasz Grobelny](https://github.com/tomasz-grobelny):

```markdown
Android Auto Protocol uses client and server certificates for mutual "authentication". It is in quotes as the only real effect is making the protocol more difficult to implement, it does not provide any security (if you exclude SbO). Anyway, the certificates have to be signed by specific RootCA (owned by Google) and they have validity dates. For headunits it may be longed as their software is rarely updated, for clients (running on phones) it is shorter, the assumption being AA software can be updated anytime.

These certificates are mutually validated - that is the headunit may validate client's certificate and vice versa. IIRC Openauto does not check client's certificate RootCA and validity dates, but my Seat's headunit created by LG does - possibly a requirement by Google.

[...]
```

[[Source](https://github.com/tomasz-grobelny/AACS/issues/3#issuecomment-767168061)]

## Requirements

1. ADB
2. Rooted Android Emulator with Android (Version 10, 11 or 12)

## Building

```shell
$ cargo build --release
```

## Usage

```shell
getcert --dhu-path="path/to/dhu.exe"
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

Copyright (c) 2022 - Luca Lewin
