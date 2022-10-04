use std::process::{Command, Stdio};
use std::clone::Clone;
use std::marker::Copy;
use crate::partition::Partition;

pub mod arch;
pub mod grub;

#[derive(Clone)]
pub struct Linux<'a> {
    pub user: String,
    pub mount_path: String,
    pub name: String,
    pub encrypt: bool,
    pub crypt_name: String,
    pub use_lvm: bool,
    pub volume_group: &'a str,
    pub device: String,
    pub password: Option<Option<String>>,
    pub timezone: String,
    pub packages_path: String,
}


pub enum CryptOperation {
    Open,
    Close,
    LuksFormat,
}

pub struct SGDiskConfig {

}

pub enum SgDiskOperation {
    Create,
    Zap,
}

pub enum MkfsFormat {
    Fat32,
    Ext4,
}

impl Linux<'_> {
    pub fn sed_replace(search: String, replace: String, file: String, arch_chroot: bool) {
        if arch_chroot == true {
            let search_replace = format!("s/{:}/{:}/g", search, replace);
            Command::new("arch-chroot")
                .arg("/mnt")
                .arg("sed")
                .arg("-i")
                .arg(search_replace)
                .arg(file)
                .status().expect("sed failed to start");
        }
    }

    pub fn sed_to_file(text: String, file: String, linux: &Linux, arch_chroot: bool) {
        if arch_chroot == true {
            Command::new("arch-chroot")
                .arg(format!("{:}", linux.mount_path))
                .arg("sed")
                .arg("-i")
                .arg(format!(r#"$ a\{:}"#, text))
                .arg(file)
                .status().expect("echo failed to start");
        }
    }


    pub fn mount(device: &str, path: &str) {
        println!("mounting {} to {}", device, path);
        Command::new("mount")
            .arg(device)
            .arg(path)
            .status().expect("mount failed to start");
    }   

    pub fn mkdir(directory: String) {
        Command::new("mkdir")
            .arg(directory)
            .status().expect("mkdir failed to start");
    }

    pub fn umount(paths: Vec<&str>) {
        for path in paths {
            Command::new("umount")
                .arg(String::from(path))
                .status().expect("umount failed to start");
        }
    }

    pub fn sgdisk(operation: SgDiskOperation, partition: &Partition) {
        match operation {
            SgDiskOperation::Create => { Command::new("sgdisk")
                    .arg(format!("-n {}:{:}", partition.index, partition.size))
                    .arg(format!("--typecode={}:{:}", partition.index, partition.typecode))
                    .arg(format!("--change-name={}:{:}", partition.index, partition.label))
                    .arg(format!("{:}", partition.device))
                    .status().expect("sgdisk failed to start");
            },
            SgDiskOperation::Zap => {
                Command::new("sgdisk")
                    .arg("-Z")
                    .arg(&partition.device)
                    .status().expect("sgdisk failed to start");
            },
        }
    }
    
    pub fn mkfs(format: MkfsFormat, name: &str) {
        match format {
            MkfsFormat::Fat32 => {
                Command::new("mkfs.fat")
                    .arg("-F32")
                    .arg(name)
                    .status()
                    .expect("mkfs.fat failed to start");
            },
            MkfsFormat::Ext4 => {
                Command::new("mkfs.ext4")
                    .arg("-F")
                    .arg(name)
                    .status()
                    .expect("mkfs.fat failed to start");
            },
        }
    }

    pub fn pvcreate(linux: &Linux) {
        Command::new("pvcreate")
            .arg(format!("/dev/mapper/{:}", linux.crypt_name))
            .status()
            .expect("pvcreate failed to start");
    }

    pub fn vgcreate(linux: &Linux, volume_group: &str) {
        Command::new("vgcreate")
            .arg(volume_group)
            .arg(format!("/dev/mapper/{:}", linux.crypt_name))
            .status()
            .expect("vgcreate failed to start");

    }

    pub fn lvcreate(name: String, space: String, volume_group: &str){
        Command::new("lvcreate")
            .arg("-l")
            .arg(space)
            .arg("-n")
            .arg(name)
            .arg(volume_group)
            .status()
            .expect("lvcreate failed to start");
    }

    pub fn cryptsetup(operation: CryptOperation, linux: &Linux) {
        match operation {
            CryptOperation::Open => { 
                if linux.password.is_some() {
                    let echo = Command::new("echo")
                        .arg("-n") 
                        .arg(format!(r#"{}"#, linux.password.as_ref().unwrap().as_ref().unwrap()))
                        .stdout(Stdio::piped())
                        .spawn().expect("echo failed to start");
                Command::new("cryptsetup")
                    .arg("open")
                    .arg(format!("{:}1", linux.device))
                    .arg("crypt")
                    .arg("-")
                    .stdin(echo.stdout.unwrap())
                    .status().expect("cryptsetup failed to start");
                }
            },
            CryptOperation::Close => { 
                Command::new("cryptsetup").arg("close").arg(&linux.crypt_name)
                    .status().expect("cryptsetup failed to start"); 
            },
            CryptOperation::LuksFormat => {
                if linux.password.as_ref().unwrap().is_some() {
                    let echo = Command::new("echo")
                        .arg("-n") 
                        .arg(format!(r#"{}"#, linux.password.as_ref().unwrap().as_ref().unwrap()))
                        .stdout(Stdio::piped())
                        .spawn().expect("echo failed to start");
                    Command::new("cryptsetup")
                        .arg("-q")
                        .arg("--verbose")
                        .arg("luksFormat")
                        .arg(format!("{:}1", linux.device))
                        .arg("-")
                        .stdin(echo.stdout.unwrap())
                        .output().expect("cryptsetup failed to start");
                }
                else {
                    panic!("Please provide password for encryption");
                }
            },
        } 

    }

    pub fn ln(soft: bool, link_from: String, link_to: String, linux: &Linux, arch_chroot: bool) {
        if soft == true {
            if arch_chroot == true {
                Command::new("arch-chroot")
                    .arg(&linux.mount_path)
                    .arg("ln")
                    .arg("-sf")
                    .arg(format!("{:}", link_from))
                    .arg(format!("{:}", link_to))
                    .status().expect("ln failed to start");
            }
        }
    }

    pub fn genfstab(linux: &Linux) {
        Command::new("genfstab")
            .arg("-U")
            .arg(&linux.mount_path)
            .arg(">>")
            .arg(format!("{:}/etc/fstab", linux.mount_path))
            .status()
            .expect("genfstab failed to start");
    }

    pub fn hwclock(linux: &Linux, arch_chroot: bool) {
        if arch_chroot == true {
            Command::new("arch-chroot")
                .arg(&linux.mount_path)
                .arg("hwclock")
                .arg("--systohc")
                .status().expect("hwclock failed to start");
        }
    }

    pub fn locale_gen(linux: &Linux, arch_chroot: bool){
        if arch_chroot == true {
            Command::new("arch-chroot")
                .arg(&linux.mount_path)
                .arg("locale-gen")
                .status().expect("localegen failed to start");
        }
    }

    pub fn mkinitcpio(linux: &Linux){
        Command::new("arch-chroot")
            .arg(&linux.mount_path)
            .arg("mkinitcpio")
            .arg("-p")
            .arg("linux")
            .status().expect("mkinitcpio failed to start");
    }

    pub fn systemctl_start(service: String, linux: &Linux, arch_chroot: bool) {
        if arch_chroot == true {
            Command::new("arch-chroot")
                .arg(&linux.mount_path)
                .arg("systemctl")
                .arg("enable")
                .arg(service)
                .status().expect("systemctl failed to start");
        }
    }

    pub fn useradd(linux: &Linux, arch_chroot: bool) {
        if arch_chroot == true{
            Command::new("arch-chroot")
                .arg(&linux.mount_path)
                .arg("useradd")
                .arg("-mG")
                .arg("wheel")
                .arg(&linux.user)
                .status().expect("useradd failed to start");
        }
    }

    pub fn passwd(linux: &Linux, arch_chroot: bool) {
        if arch_chroot == true {
            Command::new("arch-chroot")
                .arg("passwd")
                .arg(&linux.user)
                .arg(linux.password.as_ref().unwrap().as_ref().unwrap())
                .status().expect("passwd failed to start");
        }
    }

    pub fn reboot() {
        Command::new("reboot")
            .status().expect("reboot failed to start");
    }

    pub fn check_connectivity() {
        let connected = ( || {Command::new("ping")
            .arg("-c 1")
            .arg("9.9.9.9")
            .status()
            .expect("ping: connect: Network is unreachable")})();
    }
}
