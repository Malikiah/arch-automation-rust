use std::process::Command;
use std::process::Stdio;
use std::fs;
use std::str;
use structopt::StructOpt;
use regex::Regex;

// use std::env;
// fn type_of(_: T) -> &'static str {
//     type_name::()
// }
#[derive(StructOpt,Debug)]
#[structopt(name = "Arch Automation")]

struct Opt {
    #[structopt(short = "e", long = "encrypt")]
    encrypt: bool,

    #[structopt(short = "l", long = "lvm")]
    lvm: bool,

    #[structopt(short = "p", long = "package-list", name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,
}

impl Opt {
    fn lvm(&self) {
        if self.lvm == true {
            println!("Creating LVM Partitions");
        }
    }
    fn encryption(&self) {
        if self.encrypt == true {
            println!("Encrypting");
        }
    }
    fn packages(&self) {
        if self.file_name.is_none() {
            let pwd = Command::new("pwd")
                .stdout(Stdio::piped())
                .output()
                .expect("pwd command failed to start");
            let rgx = Regex::new(r#"(\\n|\\|")"#).unwrap();
            let pwd = String::from_utf8(pwd.stdout).unwrap();
            let pwd = format!("{:}", rgx.replace_all(&format!("{:?}/pkg_list.txt", &pwd), ""));
            println!("{:}", pwd); 
            install_packages(&pwd);

        } else {
            install_packages(self.file_name.as_ref().unwrap());
        }
    }
}

//impl<T, Idx> Index<Idx> for Opt where {
//    type Output = T;
//
//    #[inline(always)]
//    fn index(&self, index: Idx) -> &Self::Output {
//        self.slice.index(index)
//    }
//}
//impl<T> Iterator for Opt {
//    type Item = T;
//    //fn into_array(self) -> [Self::Item; 3] {[self.encrypt, self.lvm, self.file_name]}
//    fn next(&mut self) -> Option<Self::Item> {
//        println!("{:?}", T);
//        //if self.encrypt == true {
//        //    println!("{:?}", self.encrypt);
//        //    return Some(self.encrypt).to_string();
//        //}
//        return None;
//    }
//}

fn main() { 
    let opts = Opt::from_args();
    println!("{:?}", &opts);
   // println!("{:?}", &opts.file_name.unwrap());
    Opt::lvm(&opts);
    Opt::encryption(&opts);
    Opt::packages(&opts);
//    for opt in opts {
//        println!("{:?}", opt);
//    }
    // install_packages takes an arguement of packages and ownership of the packages variable to be
    // dereferenced when the function is complete.
    // install_packages();
}

fn install_packages(file_name: &str) {
    // let packages = vec!["sddm", "curl", "taco"];
    println!("{:?}", file_name);
    let packages = fs::read(file_name)
        .expect("Should have been able to read the file");
    // This converts the list of from the file to a string.
    let packages = str::from_utf8(&packages).unwrap();
    // This the string into a vector of Strings based on whitespace.
    let packages: Vec<String> = packages
        .split(char::is_whitespace)
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    
    for package in packages.iter() {
        Command::new("sudo")
            .arg("pacman")
            .arg("-Sy")
            .arg("--noconfirm")
            .arg(package)
            // spawn is for running a command and not waiting for it to finish.
            // .spawn()
            .status()
            .expect("pacman command failed to start");
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



