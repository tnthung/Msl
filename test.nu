#!nu


def main [name: string] {
    cd Mslc
    cargo test --package ($name) -- --nocapture
}
