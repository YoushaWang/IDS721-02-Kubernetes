# IDS721-02-food-recommend
## set up
1. create virtual enviroment
    python3 -m venv env
    source env/bin/activate
2. install rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
3. create rust project
    cargo new path
    cargo build
4. write dockerfile and Makefile
    make format
    make lint
5.  run web microservice
