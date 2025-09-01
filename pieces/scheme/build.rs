fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["proto/server.proto"], &["proto"])
        .unwrap();
}
