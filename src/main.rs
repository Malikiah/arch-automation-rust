use std::process::Command;
use std::process::Stdio;
use std::fs;
use std::str;
use std::fmt::{Debug};
//use std::fmt::{self, Display, Debug, Formatter, Result};
//use std::str::FromStr;
use structopt::StructOpt;
use regex::Regex;

// use std::env;
// fn type_of(_: T) -> &'static str {
//     type_name::()
// }
#[derive(StructOpt,Debug)]
#[structopt(name = "Arch Automation")]

struct Opt {
    #[structopt(short = "i", long = "installation-type")]
    installation_type: Option<String>,

    #[structopt(short = "e", long = "encrypt")]
    encrypt: bool,

    #[structopt(short = "l", long = "lvm")]
    lvm: bool,

    #[structopt(short = "d", long = "device")]
    device: Option<String>,

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
        let package_location = Packages::location(self);

        let packages = Packages::get(package_location);

        // This calls the install packages function with the full path to the package list
        Packages::install(packages);
    }
}

struct Packages {
    
}

impl Packages {

    fn location(opt: &Opt) -> String {
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

    fn get(package_location: String) -> Vec<String> {
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

    fn install(packages: Vec<String>) {
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
    }
    
}

//#[derive(Debug)]
//enum InstallationType {
//    FULL,
//    PREINSTALL,
//    SETUP,
//    USER,
//    POSTSETUP,
//}

//impl Display for InstallationType {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//        write!(f, "{:}", self)
//    }
//}
//
//impl FromStr for InstallationType {
//    type Err = ();
//
//    fn from_str(input: &str) -> Result {
//        match input {
//            "full" => Ok(InstallationType::FULL),
//            "preinstall" => Ok(InstallationType::PREINSTALL),
//            "setup" => Ok(InstallationType::SETUP),
//            _ => Err(()),
//        }
//            
//    }
//}
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
    check_connectivity();
    let opts = Opt::from_args();
    println!("{:?}", &opts);
   // println!("{:?}", &opts.file_name.unwrap());
    //Opt::lvm(&opts);
    //Opt::encryption(&opts);
    Opt::packages(&opts);
//    for opt in opts {
//        println!("{:?}", opt);
//    }
    // install_packages takes an arguement of packages and ownership of the packages variable to be
    // dereferenced when the function is complete.
    // install_packages();
}


fn check_connectivity() {
    Command::new("ping")
        .arg("-c 1")
        .arg("9.9.9.9")
        .status()
        .expect("ping: connect: Network is unreachable");
}

