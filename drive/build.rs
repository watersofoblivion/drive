// use drive_macros::smithy;

fn main() {
    // smithy! {};

    let files: Vec<&str> = vec![];
    for file in files {
        let msg = format!("Slint build failed on file {:#?}", file);
        slint_build::compile(file).expect(&msg);
    }
}