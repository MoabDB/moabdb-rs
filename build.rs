// Jackson Coxson

fn main() {
    prost_build::compile_protos(&["src/moabdb.proto"], &["src/"]).unwrap();
}
