use std::process::Command;
use crate::linux::Linux;


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
    //
pub fn install(linux: &Linux) {
    Linux::sed_replace(String::from("filesystems"), String::from("encrypt lvm2 filesystems"), String::from("/etc/mkinitcpio.conf"), true);
    Linux::mkinitcpio(&linux);

    Command::new("arch-chroot")
        .arg(&linux.mount_path)
        .arg("grub-install")
        .arg("--target=x86_64-efi")
        .arg("--efi-directory=/boot")
        .arg("--bootloader-id=GRUB")
        .status().expect("grub-install failed to start");

    let uuid = Command::new("arch-chroot")
        .arg(&linux.mount_path)
        .arg("blkid")
        .arg("-s")
        .arg("UUID")
        .arg("-o")
        .arg("value")
        .arg(&linux.device)
        .status().expect("blkid failed to start");

    Linux::sed_replace(String::from(r#"GRUB_CMDLINE_LINUX="""#), format!(r#"GRUB_CMDLINE_LINUX="cryptdevice=UUID={}:cryptlvm root=/dev/{}/root""#, uuid, &linux.volume_group), String::from("/etc/default/grub"),  true);

    Command::new("arch-chroot")
        .arg(&linux.mount_path)
        .arg("grub-mkconfig")
        .arg("-o")
        .arg("/boot/grub/grub.cfg")
        .status().expect("grub-mkconfig failed to start");

    

}
