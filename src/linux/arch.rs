use std::process::{Command, Stdio};
use crate::packages::Packages;
use crate::linux::Linux;
use crate::Opt;

pub fn configure(linux: &Linux) {
    let pacstrap_packages = vec!["base", "base-devel", "linux", "linux-firmware", "nvim", "amd-ucode", "lvm2", "archlinux-keyring"].into_iter().map(|s| s.to_owned()).collect();
    Packages::install(pacstrap_packages, String::from("pacstrap"), &linux, false);
    Linux::genfstab(&linux);
    //arch-chroot /mnt
    //ln -sf /usr/share/zoneinfo{opt.timezone} /etc/localtime
    //hwcloack --systohc
    //sed en_US.UTF-8 UTF-8 /etc/locale.gen
    //locale-gen
    //echo LANG=en_US.UTF-8 >> /etc/locale.conf
    //echo "computer" >> /etc/hostname
    //echo "127.0.0.1   localhost\n::1    localhost\n127.0.0.1 computer.localdomain computer"
    //>> /etc/hosts
    //
    //call install packages function
    //
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
    Linux::ln(true, format!("/usr/share/zoneinfo/{:}", linux.timezone), String::from("/etc/localtime"), &linux, true);

    Linux::sed_replace(String::from("#en_US.UTF-8 UTF-8"), String::from("en_US.UTF-8 UTF-8"), String::from("/etc/locale.gen"), true);
    
    Linux::locale_gen(&linux, true);
    
    Linux::sed_to_file(String::from("LANG=en_US.UTF-8"), String::from("/etc/locale.conf"), &linux, true);
    Linux::sed_to_file(String::from("computer"), String::from("/etc/hostname"), &linux, true);
    Linux::sed_to_file(String::from(r#"127.0.0.1   localhost\n::1    localhost\n127.0.0.1 computer.localdomain computer"#), String::from("/etc/hosts"), &linux, true);

    let packages = Packages::get(&linux.packages_path);
    Packages::install(packages, "pacman".to_string(), &linux, true);
}