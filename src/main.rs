use partition::Partition;
use structopt::StructOpt;
use std::fmt::{Debug};

use linux::*;

pub mod partition;
pub mod packages;
pub mod linux;
//pub mod configure_arch;

#[derive(StructOpt,Debug)]
#[structopt(name = "Arch Automation")]
pub struct Opt {
    #[structopt(short = "l", long = "lvm")]
    pub lvm: bool,

    #[structopt(short = "e", long = "encrypt")]
    pub encrypt: bool,
    
    #[structopt(short = "p", long = "password")]
    pub password: Option<String>,

    #[structopt(short = "d", long = "device")]
    pub device: String,

    #[structopt(short = "f", long = "packages-path")]
    pub packages_path: String,

    #[structopt(short = "t", long = "timezone")]
    pub timezone: String,
}

impl Opt {}

fn main() { 
    let opts = Opt::from_args();

    let linux = Linux {
        mount_path: String::from("mnt"),
        name: String::from("computer"),
        encrypt: opts.encrypt,
        crypt_name: String::from("crypt"),
        use_lvm: opts.lvm,
        device: String::from(opts.device),
        volume_group: "vg1",
        password: Some(opts.password),
        timezone: String::from(opts.timezone),
        packages_path: String::from(opts.packages_path),
    };

//    println!("{:?}", &opts);
    Partition::create_system(&linux);
    arch::configure(&linux);
   
}
