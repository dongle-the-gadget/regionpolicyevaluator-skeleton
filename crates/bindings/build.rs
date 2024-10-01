use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=winmd/W.I.S.P.RegionPolicyEvaluator.idl");
    println!("cargo:rerun-if-changed=riddle.rsp");

    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    let mut command = Command::new("midlrt.exe");
    command.arg("@midlrt.rsp");

    if !command.status().unwrap().success() {
        panic!("MIDLRT failed.");
    }

    if let Err(error) = windows_bindgen::bindgen([
        "--in",
        "winmd/",
        &metadata_dir,
        "--out",
        "src/bindings.rs",
        "--filter",
        "Windows.Internal.System.Profile",
        "--config",
        "implement",
    ]) {
        panic!("{error}");
    }
}
