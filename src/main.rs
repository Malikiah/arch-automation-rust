use partition::Partition;
use structopt::StructOpt;
use std::fmt::{Debug};

use linux::*;
use crate::linux::grub;
use crate::packages::Packages;

pub mod partition;
pub mod packages;
pub mod linux;
//pub mod configure_arch;

#[derive(StructOpt,Debug)]
#[structopt(name = "Arch Automation")]
pub struct Opt {
    #[structopt(short = "u", long = "user")]
    pub user: String,

    #[structopt(short = "l", long = "lvm")]
    pub lvm: bool,

    #[structopt(short = "e", long = "encrypt")]
    pub encrypt: bool,
    
    #[structopt(short = "p", long = "password")]    
    pub password: Option<String>,

    #[structopt(short = "d", long = "device")]
    pub device: String,

    #[structopt(short = "f", long = "packages-path")]
    pub packages_path: Option<String>,

    #[structopt(short = "t", long = "timezone")]
    pub timezone: String,
}

impl Opt {}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let opts = Opt::from_args();

    let linux = Linux {
        user: opts.user,
        mount_path: String::from("/mnt"),
        name: String::from("computer"),
        encrypt: opts.encrypt,
        crypt_name: String::from("crypt"),
        use_lvm: opts.lvm,
        device: opts.device,
        volume_group: "vg1",
        password: Some(opts.password),
        timezone: opts.timezone,
        packages_path: Some(opts.packages_path),
    };

//    println!("{:?}", &opts);
//    if linux.packages_path.as_ref().unwrap().is_some() {
//        println!("{:#?}", linux.packages_path);
//        let packages = Packages::get(linux.packages_path.as_ref().unwrap().as_ref().unwrap()).await;
//        
//        match packages {
//            Ok(value) => { 
//                Packages::install(value, &linux, true)
//            },
//            Err(error) => println!("{}", error),
//        }
//
//    } else {
//
//        let packages = Packages::get("https://raw.githubusercontent.com/Malikiah/arch-automation-rust/main/pkg_list.txt").await;
//        
//        match packages {
//            Ok(value) => { 
//                Packages::install(value, &linux, true)
//            },
//            Err(error) => println!("{}", error),
//        }
//
//    }
    Partition::create_system(&linux);
    arch::configure(&linux).await;
    grub::install(&linux);
   
    //Linux::umount(vec!("-a"));
    //Linux::reboot();
    Ok(())
}
