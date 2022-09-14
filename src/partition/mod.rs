use std::process::{Command, Stdio};

use crate::Opt;
use crate::packages::Packages;
use crate::linux::Linux;

use crate::linux::CryptOperation;
use crate::linux::CryptConfig;

#[derive(Debug)]
struct SystemPartition{
    boot_partition: Partition,
    root_partition: Partition,
}

#[derive(Debug)]
pub struct Partition {
    size: String,
    typecode: String,
    label: String,
    device: String,
}

impl Partition {

    pub fn create_system(opts: &Opt) {
        Linux::cryptsetup(CryptOperation::Close, CryptConfig{Name:String::from("crypt")});

        Linux::umount(vec!["/mnt/home", "/mnt/boot", "/mnt"]);

        let system_partition = SystemPartition{ 
            boot_partition: Partition{
                size: String::from("+512M"),
                typecode: String::from("ef00"),
                label: String::from("EFI"),
                device: String::from(opts.device.as_ref().unwrap()),
            },
            root_partition: Partition{
                size: String::from("-0"),
                typecode: String::from("8e00"),
                label: String::from("LVM"),
                device: String::from(opts.device.as_ref().unwrap()),
            }
        };

        let partition_vector = vec![&system_partition.boot_partition, &system_partition.root_partition];

        Command::new("sgdisk")
            .arg("-Z")
            .arg(opts.device.as_ref().unwrap())
            .status().expect("sgdisk failed to start");

        for (i, partition) in partition_vector.iter().enumerate() {
            println!("sudo sgdisk -n {}:{:} --typecode={}:{:} --change-name={}:{:} {:}", i+1, partition.size, i+1, partition.typecode, i+1, partition.label, partition.device);
            Command::new("sgdisk")
                .arg(format!("-n {}:{:}", i+1, partition.size))
                .arg(format!("--typecode={}:{:}", i+1, partition.typecode))
                .arg(format!("--change-name={}:{:}", i+1, partition.label))
                .arg(format!("{:}", partition.device))
                .status().expect("sgdisk failed to start");
        }

        Command::new("mkfs.fat")
            .arg("-F32")
            .arg(format!("{:}2", system_partition.boot_partition.device))
            .status()
            .expect("mkfs.fat failed to start");
        if opts.encrypt == true { Partition::encrypt(&opts); }
        if opts.lvm == true { Partition::lvm(&opts); }
        //mkfs.fat -F32 system_partition.boot_partition
        //call encrypt if encrypt is true
        //call lvm if lvm is true
        let pacstrap_packages = vec!["base", "base-devel", "linux", "linux-firmware", "nvim", "amd-ucode", "lvm2", "archlinux-keyring"].into_iter().map(|s| s.to_owned()).collect();
        Packages::install(pacstrap_packages, "pacstrap".to_string(), false);
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
        Command::new("arch-chroot")
            .arg("/mnt")
            .arg("ln")
            .arg("-sf")
            .arg(format!("/usr/share/zoneinfo/{:}", opts.timezone.as_ref().unwrap()))
            .arg("/etc/localtime")
            .status().expect("arch-chroot failed to start");

        Command::new("arch-chroot")
            .arg("/mnt")
            .arg("hwclock")
            .arg("--systohc")
            .status().expect("hwclock failed to start");

        Command::new("arch-chroot")
            .arg("/mnt")
            .arg("sed")
            .arg("-i")
            .arg("'s/#en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/'")
            .arg("/etc/locale.gen")
            .status().expect("sed failed to start");

        Command::new("arch-chroot")
            .arg("/mnt")
            .arg("localegen")
            .status().expect("localegen failed to start");

        Command::new("arch-chroot")
            .arg("/mnt")
            .arg("echo")
            .arg("LANG=en_US.UTF-8")
            .arg(">>")
            .arg("/etc/locale.conf")
            .status().expect("echo failed to start");

        Command::new("arch-chroot")
            .arg("/mnt")
            .arg("echo")
            .arg("computer")
            .arg(">")
            .arg("/etc/hostname")
            .status().expect("echo failed to start");

        Command::new("arch-chroot")
            .arg("/mnt")
            .arg("echo")
            .arg(r#"127.0.0.1   localhost\n::1    localhost\n127.0.0.1 computer.localdomain computer"#)
            .arg(">")
            .arg("/etc/hosts")
            .status().expect("echo failed to start");

//        Command::new("arch-chroot")
//            .arg("/mnt")
//            .arg("")

        let package_location = Packages::location(&opts);
        let packages = Packages::get(package_location);
        Packages::install(packages, "pacman".to_string(), true);

    }

    fn encrypt(opts: &Opt) {


        let echo = Command::new("echo")
            .arg("-n") 
            .arg(format!(r#"{}"#, opts.password.as_ref().unwrap()))
            .stdout(Stdio::piped())
            .spawn().expect("echo failed to start");

        Command::new("cryptsetup")
            .arg("-q")
            .arg("--verbose")
            .arg("luksFormat")
            .arg(format!("{:}1", opts.device.as_ref().unwrap()))
            .arg("-")
            .stdin(echo.stdout.unwrap())
            .output().expect("cryptsetup failed to start");

        let echo = Command::new("echo")
            .arg("-n") 
            .arg(format!(r#"{}"#, opts.password.as_ref().unwrap()))
            .stdout(Stdio::piped())
            .spawn().expect("echo failed to start");

        Command::new("cryptsetup")
            .arg("open")
            .arg(format!("{:}1", opts.device.as_ref().unwrap()))
            .arg("crypt")
            .arg("-")
            .stdin(echo.stdout.unwrap())
            .status().expect("cryptsetup failed to start");
    }

    fn lvm (opts: &Opt) {
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
        Command::new("pvcreate")
            .arg("/dev/mapper/crypt")
            .status()
            .expect("pvcreate failed to start");

        Command::new("vgcreate")
            .arg("vg1")
            .arg("/dev/mapper/crypt")
            .status()
            .expect("vgcreate failed to start");

        Command::new("lvcreate")
            .arg("-l")
            .arg("10%VG")
            .arg("-n")
            .arg("root")
            .arg("vg1")
            .status()
            .expect("lvcreate failed to start");

        Command::new("lvcreate")
            .arg("-l")
            .arg("100%FREE")
            .arg("-n")
            .arg("home")
            .arg("vg1")
            .status().expect("lvcreate failed to start");

        Command::new("mkfs.ext4")
            .arg("-F")
            .arg("/dev/vg1/root")
            .status().expect("mkfs.ext4 failed to start");
        
        Command::new("mkfs.ext4")
            .arg("-F")
            .arg("/dev/vg1/home")
            .status().expect("mkfs.ext4 failed to start");

        Command::new("mount")
            .arg("/dev/vg1/root")
            .arg("/mnt")
            .status().expect("mount failed to start");

        Command::new("mkdir")
            .arg("/mnt/home")
            .status().expect("mkdir failed to start");

        Command::new("mount")
            .arg("/dev/vg1/home")
            .arg("/mnt/home")
            .status().expect("mount failed to start");

        Command::new("mkdir")
            .arg("/mnt/boot")
            .status().expect("mkdir failed to start");

        Command::new("mount")
            .arg(format!("{:}2", opts.device.as_ref().unwrap()))
            .arg("/mnt/boot")
            .status().expect("mount failed to start");
    }
    
}
