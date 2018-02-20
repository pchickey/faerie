extern crate faerie;
extern crate tempdir;

use tempdir::TempDir;
use std::fs::File;
use std::process::Command;
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

#[test]
fn same_obj_name_as_def() {
    let mut obj = Artifact::new(Target::X86_64, "a".into());

    obj.declare("a", Decl::Function { global: true }).unwrap();

    let code = vec![1, 2, 3, 4];
    obj.define("a", code).unwrap();

    let tempdir = TempDir::new("test").expect("create tempdir");
    let objpath = tempdir.path().join("a.o");
    let file = File::create(objpath.clone()).expect("open temporary file");
    obj.write::<Elf>(file).unwrap();

    let mut cmd_ld = Command::new("ld");
    cmd_ld.arg(objpath);
    cmd_ld.arg("-shared");
    cmd_ld.arg("-o");
    cmd_ld.arg(tempdir.path().join("a.so"));

    let run_ld = cmd_ld.output().expect("linking");

    if run_ld.stderr.len() > 0 {
        eprintln!("{}", String::from_utf8_lossy(&run_ld.stderr));
    }

    assert!(run_ld.status.success());
}
