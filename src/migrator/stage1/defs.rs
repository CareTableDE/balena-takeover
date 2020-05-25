use std::fmt::{self, Display};

pub const DEV_TYPE_INTEL_NUC: &str = "intel-nuc";
pub const DEV_TYPE_GEN_X86_64: &str = "genericx86-64-ext";
pub const DEV_TYPE_RPI3: &str = "raspberrypi3";
pub const DEV_TYPE_RPI2: &str = "raspberry-pi2";
pub const DEV_TYPE_RPI4_64: &str = "raspberrypi4-64";
// pub const DEV_TYPE_BBG: &str = "beaglebone-green";
// pub const DEV_TYPE_BBB: &str = "beaglebone-black";
// pub const DEV_TYPE_BBXM: &str = "beagleboard-xm";
pub const SYS_UEFI_DIR: &str = "/sys/firmware/efi";

#[derive(Debug, Clone, Copy)]
pub(crate) enum DeviceType {
    BeagleboneGreen,
    // BeagleboneBlack,
    BeagleboardXM,
    IntelNuc,
    RaspberryPi2,
    RaspberryPi3,
    RaspberryPi4,
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},",
            match self {
                Self::IntelNuc => "X68_64/Intel Nuc",
                Self::BeagleboneGreen => "Beaglebone Green",
                Self::BeagleboardXM => "Beagleboard XM",
                Self::RaspberryPi2 => "Raspberry Pi 2",
                Self::RaspberryPi3 => "Raspberry Pi 3",
                Self::RaspberryPi4 => "Raspberry Pi 4",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) enum OSArch {
    AMD64,
    #[cfg(target_os = "linux")]
    ARMHF,
    I386,
    /*
        ARM64,
        ARMEL,
        MIPS,
        MIPSEL,
        Powerpc,
        PPC64EL,
        S390EX,
    */
}