use std::fs;
use std::io::{Read, Write};
use std::io;
use std::path::Path;

#[cfg(windows)]
const NL: &'static str = "\r\n";
#[cfg(not(windows))]
const NL: &'static str = "\r\n";

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let day = 5;

    let day_name = format!("j{}", day);

    let source_folder = Path::new("./day_template");
    let day_format = format!("../src/{}", day_name);
    let target_folder = Path::new(day_format.as_str());

    fs::create_dir_all(target_folder).unwrap();
    copy_dir_all(source_folder, target_folder).expect("Failed To copy folder");
    fs::File::create(target_folder.join(format!("{}.txt", day_name))).unwrap();
    fs::File::create(target_folder.join(format!("{}_test.txt", day_name))).unwrap();

    fs::rename(target_folder.join("mod.txt"), target_folder.join("mod.rs")).unwrap();
    let mut s = String::new();
    let mut f = fs::File::open(target_folder.join("mod.rs")).unwrap();
    f.read_to_string(&mut s).unwrap();
    let new_file = s.replace("jX", day_name.as_str());
    fs::File::create(target_folder.join("mod.rs")).unwrap().write_all(new_file.as_bytes()).unwrap();

    let mut f = fs::File::open("../src/main.rs").unwrap();
    s.clear();
    f.read_to_string(&mut s).unwrap();
    let mut lines: Vec<&str> = s.lines().collect();
    let last_line = lines.pop().unwrap();
    let str = format!("mod {};", day_name);
    lines.insert(0, str.as_str());
    let str = format!("    println!(\"{} -----------------------------------------------------\");", day_name.to_uppercase());
    lines.push(str.as_str());
    let str = "    println!(\"p1\");";
    lines.push(str);
    let str = format!("    println!(\"{{}}\", {}::p1());", day_name);
    lines.push(str.as_str());
    let str = "    println!(\"p2\");";
    lines.push(str);
    let str = format!("    println!(\"{{}}\", {}::p2());", day_name);
    lines.push(str.as_str());
    lines.push(last_line);
    fs::File::create("../src/main.rs").unwrap().write_all(lines.join(NL).as_bytes()).unwrap();

    let mut f = fs::File::open("../cargo.toml").unwrap();
    s.clear();
    f.read_to_string(&mut s).unwrap();
    let mut lines: Vec<&str> = s.lines().collect();
    lines.push(NL);
    let str = "[[bench]]";
    lines.push(str);
    let str = format!("name = \"{}\"", day_name);
    lines.push(str.as_str());
    let str = format!("path = \"src/bench_{}.rs\"", day_name);
    lines.push(str.as_str());
    let str = "harness = false";
    lines.push(str);
    fs::File::create("../cargo.toml").unwrap().write_all(lines.join(NL).as_bytes()).unwrap();

    let target = format!("../src/bench_{}.rs", day_name);
    fs::copy("bench.txt", target.as_str()).unwrap();
    let mut s = String::new();
    let mut f = fs::File::open(target.as_str()).unwrap();
    f.read_to_string(&mut s).unwrap();
    let new_file = s.replace("jX", day_name.as_str());
    fs::File::create(target.as_str()).unwrap().write_all(new_file.as_bytes()).unwrap();
}
