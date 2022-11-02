use std::process::{Command};
use std::fs;
use std::str;
use reqwest;
use crate::linux::Linux;

pub struct Packages;

enum PackageManager {
    Pacstrap,
    Pacman,
    Yay,
    Aura,
}

impl Packages {

    pub fn location<'a>(linux: &'a Linux) -> &'a str {
        //if linux.packagesPath.is_none() && linux.installationType == "FULL" {
        //    let directory = Command::new("pwd")
        //        .stdout(Stdio::piped())
        //        .output()
        //        .expect("pwd command failed to start");
        //    // This regex is to remove the (\,",\n) characters from the pwd command
        //    let regex = Regex::new(r#"(\\n|\\|")"#).unwrap();
        //    // this converts the utf8 encoded vector into a string.
        //    let directory = String::from_utf8(directory.stdout).unwrap();
        //    
        //    // This combines the string with a default file name to create a full path.
        //    format!("{:}", regex.replace_all(&format!("{:?}/pkg_list.txt", &directory), ""))
        //    
        //} else {
        &linux.packages_path.as_ref().unwrap().as_ref().unwrap()
        //}
    }

    pub async fn get(package_location: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        if package_location.starts_with("https://") || package_location.starts_with("http://") {
            println!("here");
            let packages = reqwest::get(package_location)
                            .await?
                            .text()
                            .await?;
            println!("{:#?}", packages);
            println!("test");
            Ok(packages
                .split(char::is_whitespace)
                .map(ToString::to_string)
                .collect::<Vec<_>>())
        } else {
            println!("{:?}", package_location);
            let packages = fs::read(package_location)
                .expect("Should have been able to read the file");
            // This converts the list of from the file to a string.
            let packages = str::from_utf8(&packages).unwrap();
            Ok(packages
                .split(char::is_whitespace)
                .map(ToString::to_string)
                .collect::<Vec<_>>())
        }
        // This takes a string and creates a vector of Strings based on whitespace.

    }

    pub fn install(packages: Vec<String>, linux: &Linux, arch_chroot: bool) {
            
        let mut package_manager: PackageManager = PackageManager::Pacstrap;

        for package in packages {
            if package.contains("[pacman]") { package_manager = PackageManager::Pacman; }
            if package.contains("[yay]") { package_manager = PackageManager::Yay; }
            if package.contains("[aura]") { package_manager = PackageManager::Aura; }

            match package_manager {
                 PackageManager::Pacstrap => {
                     
                     Command::new("pacstrap")
                        .arg("/mnt")
                        .arg(package)
                        .arg("--noconfirm")
                        .status().expect("pacstrap command failed to start");
                 },
                 PackageManager::Pacman => {
                    println!("pacman");
                    if arch_chroot == true {
                        Command::new("arch-chroot")
                            .arg(&linux.mount_path)
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
                 },
                 PackageManager::Yay => {
                    println!("yay");
                    Command::new("yay")
                        .arg("-Sy")
                        .arg("--noconfirm")
                        .arg(package)
                        .status().expect("yay command failed to start");
                 },
                 PackageManager::Aura => {
                    println!("aura");
                    Command::new("aura")
                        .arg("-Sy")
                        .arg("--noconfirm")
                        .arg(package)
                        .status().expect("yay command failed to start");
                 },
            }
        }
        //let packages = packages.clone();
        //let package_manager = package_manager.clone();
    }
}
