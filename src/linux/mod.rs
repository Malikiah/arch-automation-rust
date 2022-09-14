use std::process::{Command, Stdio};

pub struct Linux {
    pub arch_chroot: bool,
}

pub struct CryptConfig {
    pub Name: String,
}
pub enum CryptOperation {
    Open,
    Close,
    LuksFormat,
}

impl Linux {
    pub fn sed(search: String, replace: String) {
        return
    }

    pub fn mount(device: String, path: String) {

    }   

    pub fn umount(paths: Vec<&str>) {
        for path in paths {
            Command::new("umount")
                .arg(String::from(path))
                .status().expect("umount failed to start");
        }
    }
    
    pub fn cryptsetup(operation: CryptOperation, cryptconfig: CryptConfig) {
        match operation {
            CryptOperation::Open => { },
            CryptOperation::Close => { 
                Command::new("cryptsetup").arg("close").arg(cryptconfig.Name)
                    .status().expect("cryptsetup failed to start"); 
            },
            CryptOperation::LuksFormat => {},
        } 

    }

    pub fn check_connectivity() {
        let connected = ( || {Command::new("ping")
            .arg("-c 1")
            .arg("9.9.9.9")
            .status()
            .expect("ping: connect: Network is unreachable")})();
    }
}
