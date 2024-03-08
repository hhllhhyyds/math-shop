import os

cmds = [
    'cargo clippy --all --all-features',
    'cargo clippy --package "refined-float" --no-default-features --features libm',

    'cargo test --all --all-features',
    'cargo test --package "refined-float" --no-default-features --features libm'
]


def run():
    for cmd in cmds:
        if os.system(cmd) != 0:
            break


if __name__ == "__main__":
    run()
