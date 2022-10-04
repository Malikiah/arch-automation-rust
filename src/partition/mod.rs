use std::process::{Command, Stdio};

use crate::Opt;
use crate::packages::Packages;
use crate::linux::Linux;

use crate::linux::CryptOperation;

use crate::linux::SgDiskOperation;

use crate::linux::MkfsFormat;

#[derive(Debug)]
struct SystemPartition<'a>{
    boot_partition: Partition<'a>,
    root_partition: Partition<'a>,
}

#[derive(Debug)]
pub struct Partition<'a> {
    pub index: i8,
    pub size: String,
    pub typecode: String,
    pub label: String,
    pub device: &'a str,
}

impl Partition<'_> {

    pub fn create_system(linux: &Linux) {

        let system_partition = SystemPartition{ 
            boot_partition: Partition{
                index: 1,
                size: String::from("+512M"),
                typecode: String::from("ef00"),
                label: String::from("EFI"),
                device: &linux.device,
            },
            root_partition: Partition{
                index: 2,
                size: String::from("-0"),
                typecode: String::from("8e00"),
                label: String::from("LVM"),
                device: &linux.device,
            },
        };


        let partition_vector = vec![&system_partition.boot_partition, &system_partition.root_partition];

        Linux::cryptsetup(CryptOperation::Close, &linux);

        Linux::umount(vec!["/mnt/home", "/mnt/boot", "/mnt"]);
        
        Linux::sgdisk(SgDiskOperation::Zap, &system_partition.boot_partition);

        for partition in partition_vector {
            println!("sudo sgdisk -n {}:{:} --typecode={}:{:} --change-name={}:{:} {:}", partition.index, partition.size, partition.index, partition.typecode, partition.index, partition.label, partition.device);
            Linux::sgdisk(SgDiskOperation::Create, partition); 
        }

        let boot_device = format!("{:}2", system_partition.boot_partition.device.clone());
        let boot_path = format!("{}/boot", linux.mount_path);
        Linux::mkfs(MkfsFormat::Fat32, &boot_device);

        if linux.encrypt == true { Partition::encrypt(&linux); }
        if linux.use_lvm == true { Partition::lvm(&linux); }

        Linux::mkdir(format!("{}/boot", linux.mount_path));
        Linux::mount(&boot_device, &boot_path);
    }

    fn encrypt(linux: &Linux) {

        Linux::cryptsetup(CryptOperation::LuksFormat, linux);
        Linux::cryptsetup(CryptOperation::Open, linux);
        //let echo = Command::new("echo")
        //    .arg("-n") 
        //    .arg(format!(r#"{}"#, opts.password.as_ref().unwrap()))
        //    .stdout(Stdio::piped())
        //    .spawn().expect("echo failed to start");

        //Command::new("cryptsetup")
        //    .arg("-q")    
        //    .arg("--verbose")
        //    .arg("luksFormat")
        //    .arg(format!("{:}1", opts.device.as_ref().unwrap()))
        //    .arg("-")
        //    .stdin(echo.stdout.unwrap())
        //    .output().expect("cryptsetup failed to start");

        //let echo = Command::new("echo")
        //    .arg("-n") 
        //    .arg(format!(r#"{}"#, opts.password.as_ref().unwrap()))
        //    .stdout(Stdio::piped())
        //    .spawn().expect("echo failed to start");

        //Command::new("cryptsetup")
        //    .arg("open")
        //    .arg(format!("{:}1", opts.device.as_ref().unwrap()))
        //    .arg("crypt")
        //    .arg("-")
        //    .stdin(echo.stdout.unwrap())
        //    .status().expect("cryptsetup failed to start");
    }

    fn lvm (linux: &Linux) {
        // pvcreate /dev/mapper/crypt
        // vgreate vg1 /dev/mapper/crypt
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
        Linux::pvcreate(&linux);

        Linux::vgcreate(&linux, linux.volume_group);

        let logical_volumes: [&str;2] = ["root", "home"];
        let root_path = format!("/dev/{}/{}", linux.volume_group, logical_volumes[0]);
        let home_path = format!("/dev/{}/{}", linux.volume_group, logical_volumes[1]);

        Linux::lvcreate(String::from(logical_volumes[0]), String::from("10%VG"), linux.volume_group);
        Linux::lvcreate(String::from(logical_volumes[1]), String::from("100%FREE"), linux.volume_group);

        Linux::mkfs(MkfsFormat::Ext4, &root_path);
        Linux::mkfs(MkfsFormat::Ext4, &home_path);

        Linux::mount(&root_path, &linux.mount_path);
        Linux::mkdir(format!("{}/home", linux.mount_path));
        Linux::mount(&home_path, &format!("{}/home", linux.mount_path));

    }
}
