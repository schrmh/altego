# LCPAE Discord bot

Blazingly fast Discord bot written in Rust using Serenity Discord API library, features:
* stuff

## How to use:

### Prerequisites

Latest stable Rust which you can install from https://rustup.rs or up to date Docker https://www.docker.com/.

### Setup

#### Rust from repositories or rustup.rs:

``` bash
rustup override set stable
cargo build --release
DISCORD_TOKEN=yourtoken ./target/release/lcpae 
```

#### Docker:

``` bash
sudo docker build -t lcpae .
sudo docker volume create lcpae-data
sudo docker run -it -d --name lcpae -v lcpae-data:/lcpae-data -e DISCORD_TOKEN=yourtoken lcpae
```

Learn more about Docker [here](https://docs.docker.com/get-started/)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

