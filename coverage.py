import os

cmds = [
    'rustup component add llvm-tools-preview',
    'cargo test --all --all-features',
    'grcov . --binary-path target/debug/deps/ -s . -t html --branch --ignore-not-existing -o target/coverage/html',
    'grcov . --binary-path target/debug/deps/ -s . -t lcov --branch --ignore-not-existing -o target/coverage/lcov.info',
]

def set_env():
    cwd = os.getcwd()
    os.environ['CARGO_INCREMENTAL'] = "0"
    os.environ['RUSTFLAGS'] = "-Cinstrument-coverage"
    os.environ['LLVM_PROFILE_FILE'] = os.path.join(cwd, "target/coverage/cargo-test-%p-%m.profraw")


def run():
    for cmd in cmds:
        if os.system(cmd) != 0:
            break

def clean_tmp():
    cwd = os.getcwd()
    dir = os.path.join(cwd, "target/coverage")
   
    for item in os.listdir(dir):
        if item.endswith(".profraw"):
            file = os.path.join(dir,item)
            os.remove(file)


if __name__ == "__main__":
    set_env()
    run()
    clean_tmp()