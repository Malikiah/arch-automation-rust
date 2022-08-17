use std::process::Command;
use std::fs;
use std::str;

// use std::env;
// fn type_of(_: T) -> &'static str {
//     type_name::()
// }

fn main() { 

    // install_packages takes an arguement of packages and ownership of the packages variable to be
    // dereferenced when the function is complete.
    install_packages();
}

fn install_packages() {
    //    let args: Vec<String> = env::args().collect();
    // let packages = vec!["sddm", "curl", "taco"];
    let packages = fs::read("/home/default/Documents/programming/rust/arch-automation/pkg_list.txt")
        .expect("Should have been able to read the file");
    // This converts the list of from the file to a string.
    let packages = str::from_utf8(&packages).unwrap();
    // This the string into a vector of Strings based on whitespace.
    let packages: Vec<String> = packages
        .split(char::is_whitespace)
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    
    for package in packages.iter() {
        let mut install_package = Command::new("sudo")
            .arg("pacman")
            .arg("-Sy")
            .arg("--noconfirm")
            .arg(package)
            // spawn is for running a command and not waiting for it to finish.
            // .spawn()
            .status()
            .expect("pacman command failed to start");
        
        // println!("{:?}", install_package);


    }
//    let ls = Command::new("sudo")
//         .arg("pacman")
//         .arg("-Sy")
//         .arg("--noconfirm")
//         .arg(packages)
//         .spawn()
//         .expect("pacman command failed to start");
//    println!("{:?}", ls)
    
}



