use std::process::{Command, Stdio};
use std::fs;
use std::str;
use regex::Regex;

use crate::Opt;

pub struct Packages {
    location: String,
    packages: Vec<String>,
}

impl Packages {

    pub fn location(opt: &Opt) -> String {
        if opt.file_name.is_none() && opt.installation_type.as_ref().unwrap() == "FULL" {
            let directory = Command::new("pwd")
                .stdout(Stdio::piped())
                .output()
                .expect("pwd command failed to start");
            // This regex is to remove the (\,",\n) characters from the pwd command
            let regex = Regex::new(r#"(\\n|\\|")"#).unwrap();
            // this converts the utf8 encoded vector into a string.
            let directory = String::from_utf8(directory.stdout).unwrap();
            
            // This combines the string with a default file name to create a full path.
            format!("{:}", regex.replace_all(&format!("{:?}/pkg_list.txt", &directory), ""))
            
        } else {

            opt.file_name.as_ref().unwrap().to_string()

        }
    }

    pub fn get(package_location: String) -> Vec<String> {
        println!("{:?}", package_location);
        let packages = fs::read(package_location)
            .expect("Should have been able to read the file");
        // This converts the list of from the file to a string.
        let packages = str::from_utf8(&packages).unwrap();
        // This takes a string and creates a vector of Strings based on whitespace.
        packages
            .split(char::is_whitespace)
            .map(ToString::to_string)
            .collect::<Vec<_>>()

    }

    pub fn install(packages: Vec<String>, package_manager: String, arch_chroot: bool) {
        let packages = packages.clone();
        let package_manager = package_manager.clone();
        for package in packages.iter() {
            if package_manager == "pacman" {
                if arch_chroot == true {
                    Command::new("arch-chroot")
                        .arg("/mnt")
                        .arg("pacman")
                        .arg("-Sy")
                        .arg("--noconfirm")
                        .arg(package)
                        // spawn is for running a command and not waiting for it to finish.
                        // .spawn()
                        .status().expect("pacman command failed to start");
                } else {
                    Command::new("pacman")
                        .arg("-Sy")
                        .arg("--noconfirm")
                        .arg(package)
                        // spawn is for running a command and not waiting for it to finish.
                        // .spawn()
                        .status().expect("pacman command failed to start");
                }
            } else if package_manager == "pacstrap" {
                    Command::new("pacstrap")
                        .arg("/mnt")
                        .arg(package)
                        .arg("--noconfirm")
                        .status().expect("pacstrap command failed to start");
            } else if package_manager == "yay" {
                    Command::new("yay")
                        .arg("-Sy")
                        .arg("--noconfirm")
                        .arg(package)
                        .status().expect("yay command failed to start");
            }
        }
    }
}
