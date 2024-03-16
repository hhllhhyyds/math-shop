import os
import argparse


def print_green(skk):
    print("\033[92m {}\033[00m" .format(skk), end="")


def run_cmd(cmd):
    print_green("Running ")
    print(cmd)
    if os.system(cmd) != 0:
        quit()


cmds_basic = [
    'cargo clippy --all --all-features',
    'cargo test --all --all-features',
]

cmds_all_features = [
    'cargo clippy --package "refined-float" --no-default-features --features std',
    'cargo clippy --package "refined-float" --no-default-features --features libm',
    'cargo clippy --package "refined-float"',
    'cargo clippy --package "refined-float" --all-features',

    'cargo test --package "refined-float" --no-default-features --features std',
    'cargo test --package "refined-float" --no-default-features --features libm',
    'cargo test --package "refined-float"',
    'cargo test --package "refined-float" --all-features',

    'cargo clippy --package "fft-shop" --no-default-features --features std-float',
    'cargo clippy --package "fft-shop" --no-default-features --features libm-float',
    'cargo clippy --package "fft-shop"',
    'cargo clippy --package "fft-shop" --all-features',

    'cargo test --package "fft-shop" --no-default-features --features std-float',
    'cargo test --package "fft-shop" --no-default-features --features libm-float',
    'cargo test --package "fft-shop"',
    'cargo test --package "fft-shop" --all-features',
]


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--all-features", default=False,
                        action="store_true", help="test different features combinations for all crates")
    args = parser.parse_args()

    [run_cmd(cmd) for cmd in cmds_basic]

    if args.all_features:
        [run_cmd(cmd) for cmd in cmds_all_features]
