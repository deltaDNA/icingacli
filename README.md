# icingacli

[![Build Status](https://travis-ci.org/sfackler/rust-postgres.svg?branch=master)](https://travis-ci.org/william20111/icingacli) [![Latest Version](https://img.shields.io/crates/v/icingacli.svg)](https://crates.io/crates/icingacli)

CLI client for interacting with the icinga [API](https://docs.icinga.com/icinga2/latest/doc/module/icinga2/toc#!/icinga2/latest/doc/module/icinga2/chapter/icinga2-api)

![](header.png)

## Installation

OS X & Linux:
```sh
cargo install icingacli
```

Currently working on rpm/deb builds.

## Usage

By default it looks for $HOME/.icingacli but you can pass -c with a config path.

```toml

server = "localhost"
port = 5665
user = "root"
password = "toor"

```

Run icingacli --help for full list of commands and options.

```
william@server-1 $ ./target/debug/icingacli --help
icingacli 0.1.1
William Fleming <wfleming.fleming@deltadna.com>
icingai cli tool

USAGE:
    icingacli [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -c, --config <FILE>    Sets a custom config file

SUBCOMMANDS:
    acknowledge-problem    ack knowledge problem
    add-comment            Adds a comment from an author to services or hosts
    generate-ticket        generate a ticket for CSR signing
    group                  list hosts in group
    help                   Prints this message or the help of the given
                           subcommand(s)
    host                   list host info
    problems               print information on icinga problems
    remove-comment         removes a comment using the name attribute
    reschedule-check       reschedule a check for hosts and services
    restart                restart the icinga process
    shutdown               shutdown icinga process
    status                 prints status api page
    stream                 stream events from API
```

## Examples

Listing all current problems
```bash

william@server-1 $ icingacli problems | jq

{
  "results": [
    {
      "attrs": {
        "name": "disks",
        "state": 2
      },
      "joins": {},
      "meta": {},
      "name": "server-3!disks",
      "type": "Service"
    }
  ]
}

```

Acknowledge problem with serivce

```bash
william@server-1 $ icingacli acknowledge-problem -a "william" -c "its all good" -h "server-3" -s "disks"
{
  "results": [
    {
      "code": 200.0,
      "status": "Successfully acknowledged problem for object 'server-3!disks'."
    }
  ]
}
```

Remotely restart icinga

```bash
william@server-1 $ icingacli restart | jq

{
  "results": [
    {
      "code": 200,
      "status": "Restarting Icinga 2."
    }
  ]
}

```

Generate PKI ticket for new agent
```bash
william@server-1 $ icingacli generate-ticket -c "server-3" | jq
{
  "results": [
    {
      "code": 200,
      "status": "Generated PKI ticket '6ef860c52aeafb282c2939098d503241b30438ba' for common name 'server-3'.",
      "ticket": "6ef860c52aeafb282c2939098d503241b30438ba"
    }
  ]
}
```

## Development setup

```bash
william@server-1 $ git clone https://github.com/william20111/icingacli.git
william@server-1 $ cd icingacli
william@server-1 $ cargo build
```

## Release History

* 0.1.2
    * Added schedule downtime & remove downtime
* 0.1.1
    * Added comment remove & add comments commands
* 0.1.0
    * First release WIP.

## Meta


## Contributing

1. Fork it (<https://github.com/deltaDNA/icingacli>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
