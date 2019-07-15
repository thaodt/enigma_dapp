# enigma_dapp
## Requirements:
- Docker
- [docker-compose](https://docs.docker.com/compose/install/) version 1.23.2 or higher
- Simulation mode (or Software mode: is used for development purpose)
    - run in production (run SGX in Hardware mode): need to enable Intel Software Guard Extensions (SGX) & install Linux SGX driver)
- [Rust](https://www.rust-lang.org/tools/install): `rustup`, `rustc` and `cargo`

## Setup a developer environment / init Enigma Discovery Docker Network on your local machine
1. Clone enigma-contract, enigma-core & discovery-docker-network 
```
git clone https://github.com/enigmampc/enigma-contract.git
git clone https://github.com/enigmampc/enigma-core.git
git clone https://github.com/enigmampc/discovery-docker-network.git
```
Within discovery-docker-network dir:

2. Create `.env` file from `.env.template` and adjust `SGX_MODE` to equal `SW` (software/simulation) or `HW` (hardware-compatible device) accordingly
3. Mount volumes for developing by editing `docker-compose.yml` file to volume map your local repositories on the relevant containers (`client` and `core`).
for examples:
```
client:
  build:
    context: enigma-contract
    args:
      - GIT_BRANCH_CONTRACT=${GIT_BRANCH_CONTRACT}
  stdin_open: true
  tty: true
  networks:
    - net
  hostname: client
  volumes:
    - "built_contracts:/root/enigma-contract/build/contracts"
    - "/path_to/enigma-contract:/root/enigma-contract"  # add this mapping
```

mapping the local folder to where you've cloned the repo to the corresponding folder inside that container 
(`local_folder:folder_in_container`).


More specifically:

For the `client` container, add the following line to the volumes section:
- "/local/path/enigma-contract:/root/enigma-contract"
where you must replace the `/local/path/enigma-contract` fragment with the absolute path to 
where you have cloned the enigma-contract repository in step 1 above in your local machine.

For the `core` container, add a volumes section and the core repository mapping:

volumes:
  - "/local/path/enigma-core:/root/enigma-core"

4. Still the same dir, run `docker-compose build` to build all the Docker images needed for the network.
5. After done, run `./launch.bash` to launch the Docker network.
    Note: can add option `-l` to write logs for tracking & debug purposes.

6. Within the core repo folder, create a new lib Rust project under `examples/eng_wasm_contracts` on the command line with:

cd examples/eng_wasm_contracts
cargo new <project_name> --lib

and edit the `Cargo.toml` inside the newly created folder, to add the following sections
 (so that it resembles one of the example ones):
```
[dependencies]
eng-wasm = {path = "../../../eng-wasm"}
eng-wasm-derive = {path = "../../../eng-wasm/derive"}

[lib]
crate-type = ["cdylib"]

[profile.release]
panic = "abort"
lto = true
opt-level = "z"
```
7. Write your secret contract in `src/lib.rs` for the actual secret contract code. 
8. After your local testnet launched successfully, open new terminal, 
Within the `discovery-docker-network` dir, enter the `core` container by running the following command:
```
docker-compose exec core /bin/bash
```

- the first time, you need to build the core binaries inside the container, 
which varies depending on whether you are running in Hardware mode or in Simulation mode:
Simulation Mode
```
root@core:~# cd enigma-core/enigma-core
root@core:~/enigma-core/enigma-core# make full-clean
root@core:~/enigma-core/enigma-core# SGX_MODE=SW make DEBUG=1
```
- then, you can build/compile your secret contract with the following commands:
```
root@core:~/enigma-core/enigma-core# cd ~/enigma-core/examples/eng_wasm_contracts/<project_name>
root@core:~/enigma-core/examples/eng_wasm_contracts/<project_name># cargo build --release --target wasm32-unknown-unknown
```

9. The `cargo build` command above will compile your secret contract to the following path inside the container:
```
~/enigma-core/examples/eng_wasm_contracts/<project_name>/target/wasm32-unknown-unknown/release/contract.wasm
```
- Because this volume is mounted on your local filesystem, it will also be available at:
```
/local/path/enigma-core/examples/eng_wasm_contracts/<project_name>/target/wasm32-unknown-unknown/release/contract.wasm
```