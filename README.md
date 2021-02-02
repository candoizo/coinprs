# Coinp.rs 💰

A config-driven customizable cryptocurrency command line tool written in Rust :crab: , powering https://coinprs.com & https://coinp.rs

## Features

-   A locally installable
-   Track hundreds of token prices with CoinGecko API (thanks! 🐍).
-   **YAML** configuration files, plus support for TOML/JSON/HJSON/INI.
-   Outrageously fast, customizable, and cross-platform!
-   Well suited for cli / stdio / cronjob interaction
-   Red-green colourblind mode (plus fully custom colours!) ❤️

#### Screenshots

@TODO

#### Try me!

I'll host a copy to try for convience, without installing!
```bash
curl https://api.coinp.rs/bitcoin
```

## Installation

Docker: (1337Mb) **Recommended**
```bash
docker pull https://registry.gitlab.com/candoizo/coinprs:latest
# save your configuration file, and it's ready to go!
docker run -v ./coinprs.yml:/coinprs.yml candoizo/coinprs report
```

Rust Cargo: `cargo install coinprs`

Arch Linux:

```bash
# from the aur
paru -S coinprs
# from source
git clone https://registry.gitlab.com/candoizo/coinprs
cd coinprs && makepkg -si
```

## Basic Configuration

By default we check for a file named `coinprs.[yml/yaml/toml/json]` in the current directory `.`, the user's home `$HOME`, and finally `$HOME/.config/`. YAML Is recommended for non-programmatic use.

**Sample Yaml Configuration:**
```yml
# tldr: configure your local currency, add some assets, type `coinprs report`
table:
  title: we like the coin 🔷✋
  localize:
    headers:
      num:
        text: "#"
        tint: green
        align: center
    data:
      num:
        tint: blue

money:
  decimals: 2
  currency:
    - usd
    - btc

assets:
  - bitcoin:
      desc: binance account
      amount: 0.3

  - bitcoin:
      desc: cold storage
      amount: 0.00432
      tint: "#icyblue"
      decimals: 4 # override global money decimals

  - ethereum:
      desc: browsers
      amount: 0.05

  - ethereum:
      desc: weth
      amount: 0.02

  - dogecoin:
      amount: 1337

```

See [this](./yaml) same example written in [toml](./toml) or [json](./json) or [ini](./ini)!

## Basic Usage

**Examples:**

List supported coins in config.
```sh
coinprs list
...snip...
coinprs list | grep btc
bitcoin btc
```

Print out and optionally (-s)aving output to a file.

```sh
coinprs report -s # -q can hide the table

Saved to ./coinprs.2020-01-31.14.36.55.txt
```

Display only bitcoin assets.

```sh
coinprs report bitcoin
```

Check the price of Bitcoin.

```sh
coinprs price bitcoin
```

## Full Documentation

### Configuration

### Usage

```yml
# File location is checked for one of the following:
# ./coinprs.yml
# OR $HOME/coinprs.yml
# OR $HOME/.config/coinprs.yml

currencies: # array of ISO currency identifies
  - btc # default
  - usd # default

assets: # array of tracked coins / assets

  # decimals can be set for all currencies here, or individually
  decimals: 3 # default

  # options: < alpha(default), price, value >
  sort: alpha # default
  reverse_sort: false # default


  - name: bitcoin # token to track
    amount: 0.23 # amount of token owned
    decimals: 3 # individual decimal

```

## Build

This project can be built from source using a docker container using `docker-compose`.

```bash
git clone https://gitlab.com/candoizo/coinprs
cd coinprs/docker && docker-compose build
```

## Issues

Please include your operating system, rust versions, configuration, where you installed the package / how you are using it. Thanks in advance!

## @TODO

-   More modular
-   Web interfacing
-   Streaming updates
-   Optional interactive interfaces!
-   Optional database collections!

## License

Following conditions must be met:
- Improvements / new features must be passed back to this project
- Bug fixes / additional enhancements must be documented

To be determined...
