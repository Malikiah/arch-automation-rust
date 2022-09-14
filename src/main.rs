use partition::Partition;
use structopt::StructOpt;
use std::fmt::{Debug};

pub mod partition;
pub mod packages;
pub mod linux;


#[derive(StructOpt,Debug)]
#[structopt(name = "Arch Automation")]

pub struct Opt {
    #[structopt(short = "i", long = "installation-type")]
    pub installation_type: Option<String>,

    #[structopt(short = "e", long = "encrypt")]
    pub encrypt: bool,
    
    #[structopt(short = "p", long = "password")]
    pub password: Option<String>,

    #[structopt(short = "l", long = "lvm")]
    pub lvm: bool,

    #[structopt(short = "d", long = "device")]
    pub device: Option<String>,

    #[structopt(short = "f", long = "package-file")]
    pub file_name: Option<String>,

    #[structopt(short = "t", long = "timezone")]
    pub timezone: Option<String>,
}

impl Opt {}


fn main() { 
    let opts = Opt::from_args();
    println!("{:?}", &opts);
    Partition::create_system(&opts);
}
