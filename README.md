# 24Sessions communication plugin for ID Contact

This respository is a part of the ID Contact ecosystem. It contains a communication plugin used to hook up the
[24sessions](https://www.24sessions.com/) video conferencing software to the ID Contact infrastructure.

This implementation is intended as a reference for other video conferencing plugins. It aims to keep the
video conferencing software as agnostic as possible to ID Contact, by implementing only the minimally required interaction.

The plugin consists of 2 parts:

- The plugin backend, which is responsible for talking to the rest of the ID Contact ecosystem as well as redirecting guest users to the location where they can initiate the authentication process.
- The attribute display, which is responsible for displaying attributes in an iframe embedded in the interface for the service employee.

The plugin backend is written in [Rust](https://www.rust-lang.org/), the web interface is created using Typescript and React.

## Getting started

To build and run this plugin backend run:
```
ROCKET_CONFIG=config.sample.toml cargo run
```

The attribute UI can be built by:
```
cd attribute-ui
yarn
yarn run build
```

Setup a Postgres database and execute `schema.sql`.

Configure the applicable domains and credentials in:

- `attribute-ui/config.js`
- `config.sample.toml`

You will need a webserver (like NGINX) to serve static files and perform the necessary routing. 

## Further reading

Complete documentation for this plugin can be found in [the general ID Contact documentation](https://docs.idcontact.nl)
