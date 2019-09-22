# enigma_dapp
## Requirements:
- [Docker](https://docs.docker.com/install/)
- [docker-compose](https://docs.docker.com/compose/install/) version 1.23.2 or higher
- NodeJS (version 10 or higher): node & npm (Enigma team advise nvm to switch node versions). Currently it worked on node 10 & 11.
- [Rust](https://www.rust-lang.org/tools/install): `rustup`, `rustc` and `cargo`.

## Install to prepare for a developer environment
```
npm install -g @enigmampc/discovery-cli
```

## Discovery command usage
```
$ discovery <command>

Commands:
  discovery init     Initialize Enigma Discovery development environment
  discovery compile  Compile Secret Contracts and Smart Contracts
  discovery migrate  Migrate Secret Contracts and Smart Contracts
  discovery pull     Pull the latest images for the containers in the network
  discovery start    Launch the Discovery Docker network
  discovery stop     Stop the network by stopping and removing all containers
  discovery test     Test Secret Contracts and Smart Contracts

Options:
  --help     Show help                                                 [boolean]
  --version  Show version number                                       [boolean]
```

## Initialize Enigma Discovery dev env
After installing Discovery CLI package above, the next thing you need to create the Enigma Discovery DEV env. Lets say I create a new directory called `enigma-experience`:
```
$ mkdir enigma-experience
$ cd enigma-experience
```
Then you need to run the command `discovery init`. This command pulls the necessary pre-built docker images to **_run the Enigma testnet locally_**. 

- When asked whether to set up the environment in _the current folder_, choose `Y` for yes.
- When asked whether to use `hardware mode` (**`HW`**) or `simulation mode` (**`SW`**), we will select `sw`. 
- Finally, choose `Y` to allow the docker images to be pulled to your machine — if this is your first time creating an Enigma project and/or pulling these docker images, this may take ~15–20 minutes.

After finishing the initialization, you can see it will generate some new directories in your current box.
- You can temporarily ignore about those folders, I will explain later.
- But the source code in this repository, you may clone into the `secret_contracts` dir.
- After finishing clone the source code, the next thing is compiling it. You can run `discovery compile`.

## Development Progress
- Basic features just finished and some's still in development.
- Compilation should be okay now.
