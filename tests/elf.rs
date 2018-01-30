extern crate faerie;
extern crate tempdir;

use tempdir::TempDir;
use std::fs::File;
use faerie::*;

#[test]
fn link_symbol_pair_panic() {
    let mut obj = Artifact::new(Target::X86_64, "t.o".into());

    obj.declare("a", Decl::Function { global: true }).unwrap();
    obj.declare("b", Decl::Function { global: true }).unwrap();

    let code = vec![1, 2, 3, 4];
    obj.define("b", code).unwrap();
    obj.link(Link {
        to: "a",
        from: "b",
        at: 0,
    }).unwrap();

    let tempdir = TempDir::new("test").expect("create tempdir");
    let file = File::create(tempdir.path().join("t.o")).expect("open temporary file");
    obj.write::<Elf>(file).unwrap();
}
