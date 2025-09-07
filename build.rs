
fn main() {
    tonic_build::configure()
        .compile(&["proto/infer.proto"], &["proto"])
        .unwrap();
}