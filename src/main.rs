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
}

struct Packages {
    location: String,
    packages: Vec<String>,
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

    fn install(packages: Vec<&str>, package_manager: &str) {

        if package_manager == "pacman" {
                for package in packages.iter() {
                    Command::new("pacman")
                        .arg("-Sy")
                        .arg("--noconfirm")
                        .arg(package)
                        // spawn is for running a command and not waiting for it to finish.
                        // .spawn()
                        .status()
                        .expect("pacman command failed to start");
                    }
        } else if package_manager == "pacstrap" {
                for package in packages.iter() {
                    Command::new("pacstrap")
                        .arg("/mnt")
                        .arg(package)
                        .status()
                        .expect("pacman command failed to start");
                    }
        } else if package_manager == "yay" {
                for package in packages.iter() {
                    Command::new("yay")
                        .arg("-Sy")
                        .arg("--noconfirm")
                        .arg(package)
                        .status()
                        .expect("pacman command failed to start");
                    }
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


#[derive(Debug)]
struct SystemPartition{
    boot_partition: Partition,
    root_partition: Partition,
}

#[derive(Debug)]
struct Partition {
    size: String,
    typecode: String,
    label: String,
    device: String,
    
}

//impl Iterator for SystemPartition {
//    type Item = Partition;
//
//    fn next(&mut self) -> Option<Self::Item> {
//    }
//}

impl Partition {

    fn create_system(opts: &Opt) {
        let system_partition = SystemPartition{ 
            boot_partition: Partition{
                size: String::from("+512M"),
                typecode: String::from("ef00"),
                label: String::from("EFI"),
                device: String::from(opts.device.as_ref().unwrap()),
            },
            root_partition: Partition{
                size: String::from("0"),
                typecode: String::from("8e00"),
                label: String::from("LVM"),
                device: String::from(opts.device.as_ref().unwrap()),
            }
        };
//        let boot_partition = Partition{
//                size: String::from("+512M"),
//                typecode: String::from("ef00"),
//                label: String::from("EFI"),
//                device: String::from(opts.device.as_ref().unwrap()),
//            };
//        let root_partition = Partition{
//                size: String::from("0"),
//                typecode: String::from("8e00"),
//                label: String::from("LVM"),
//                device: String::from(opts.device.as_ref().unwrap()),
//            };
        let partition_vector = vec![&system_partition.boot_partition, &system_partition.root_partition];
        for (i, partition) in partition_vector.iter().enumerate() {
            println!("sudo sgdisk -n {}:{:} --typecode={}:{:} --change-name={}:{:} {:}", i+1, partition.size, i+1, partition.typecode, i+1, partition.label, partition.device);
//            Command::new("sgdisk")
//                .arg(format!("-n {}:{:?}", i+1, partition.size))
//                .arg(format!("--typecode={}:{:}", i+1, partition.typecode)
//                .arg(format!("--change-name={}:{:}", i+1, partition.label)
//                .arg(format!("{:#?}", partition.device)
//                .status() 
//                .expect("sgdisk failed to start");
                
        }

        Command::new("mkfs.fat")
            .arg("-F32")
            .arg(format!("{:}1", system_partition.boot_partition.device))
            .status()
            .expect("mkfs.fat failed to start");
        if opts.encrypt == true { Partition::encrypt(&opts); }
        if opts.lvm == true { Partition::lvm(&opts); }
        //mkfs.fat -F32 system_partition.boot_partition
        //call encrypt if encrypt is true
        //call lvm if lvm is true
        let pacstrap_packages = vec!["linux", "linux-firmware", "nvim", "amd-ucode", "lvm2"];
        Packages::install(pacstrap_packages, "pacstrap");
        //call pacstrap
        //genfstab -U /mnt >> /mnt/etc/fstab
        Command::new("genfstab")
            .arg("-U")
            .arg("/mnt")
            .arg(">>")
            .arg("/mnt/etc/fstab")
            .status()
            .expect("genfstab failed to start");
        
        //arch-chroot /mnt
        //ln -sf /usr/share/zoneinfo{opt.timezone} /etc/localtime
        //hwcloack --systohc
        //sed en_US.UTF-8 UTF-8 /etc/locale.gen
        //locale-gen
        //echo LANG=en_US.UTF-8 >> /etc/locale.conf
        //echo "computer" >> /etc/hostname
        //echo "127.0.0.1   localhost\n::1    localhost\n127.0.0.1 computer.localdomain computer"
        //>> /etc/hosts
        //call install packages function
        // edit /etc/mkinitcpio.conf to include encrypt lvm2 before the filesystems hook
        // mkinitcpio -p linux
        // grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB
        // get the UUID of the lvm partition using blkid
        // edit /etc/default/grub to have GRUB_CMDLINE_LINUX="cryptdevice=UUID={uuid}:cryptlvm root=/dev/vg1/root
        // grub-mkconfig -o /boot/grub/grub.cfg
        // systemctl enable NetworkManager
        // systemctl enable bluetooth
        // useradd -mG wheel default
        // passwd {password}
        // edit /etc/sudoers to replace # %wheel ALL=(ALL) ALL with %wheel ALL=(ALL) ALL
        // exit
        // umount -a
        // reboot
    }

    fn encrypt(opts: &Opt) {
        Command::new("cryptsetup")
            .arg("-q")
            .arg("--verbose")
            .arg("luksFormat")
            .arg(format!("{:#?}", opts.device))
            .status() 
            .expect("cryptsetup failed to start");

        match opts.lvm {
            true => { Command::new("crypsetup")
                        .arg("open")
                        .arg(format!("{:#?}", opts.device))
                        .arg("cryptlvm")
                        .status() 
                        .expect("cryptsetup failed to start");
            },
            false => { Command::new("crypsetup")
                        .arg("open")
                        .arg(format!("{:#?}", opts.device))
                        .arg("crypt")
                        .status()
                        .expect("cryptsetup failed to start");
            },
        }
    }
            
    fn lvm (opts: &Opt) {
        // pvcreate /dev/mapper/cryptlvm
        // vgreate vg1 /dev/mapper/cryptlvm
        // lvcreate -l 10%VG -n root vg1
        // lvcreate -l 100%FREE -n home vg1
        // mkfs.ext4 /dev/vg1/root
        // mkfs.ext4 /dev/vg1/home
        // mount /dev/vg1/root /mnt
        // mkdir /mnt/home
        // mount /dev/vg1/home /mnt/home
        // mkdir /mnt/boot
        // mount opts.device1 /mnt/boot
        //
    }
    
}

fn check_connectivity() {
    let connected = ( || {Command::new("ping")
        .arg("-c 1")
        .arg("9.9.9.9")
        .status()
        .expect("ping: connect: Network is unreachable")})();
}

fn main() { 
    check_connectivity();
    let opts = Opt::from_args();
    println!("{:?}", &opts);
    //Partition::create_system(&opts);
   // println!("{:?}", &opts.file_name.unwrap());
    //Opt::lvm(&opts);
    //Opt::encryption(&opts);
    //let package_location = Packages::location(opt);
    //let packages = Packages::get(package_location);
    //Packages::install(packages);
//    for opt in opts {
//        println!("{:?}", opt);
//    }
    // install_packages takes an arguement of packages and ownership of the packages variable to be
    // dereferenced when the function is complete.
    // install_packages();
}
