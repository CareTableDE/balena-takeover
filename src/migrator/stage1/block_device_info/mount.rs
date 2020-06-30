use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use log::{debug, trace};

use crate::common::{Error, Result, ToError};
use crate::ErrorKind;

#[derive(Clone, Debug)]
pub(crate) struct Mount {
    mountpoint: PathBuf,
    fs_type: String,
}

impl Mount {
    pub fn get_mountpoint(&self) -> &Path {
        self.mountpoint.as_path()
    }

    #[allow(dead_code)]
    pub fn get_fs_type(&self) -> &str {
        self.fs_type.as_str()
    }
}

pub(crate) type MountTab = HashMap<PathBuf, Mount>;

impl Mount {
    pub fn from_mtab() -> Result<MountTab> {
        let mtab_str =
            read_to_string("/etc/mtab").upstream_with_context("Failed to read from '/etc/mtab'")?;

        let mut mounts: MountTab = MountTab::new();

        for (line_no, line) in mtab_str.lines().enumerate() {
            let columns: Vec<&str> = line.split_whitespace().collect();
            if columns.len() < 3 {
                return Err(Error::with_context(
                    ErrorKind::InvParam,
                    &format!("Failed to parse /etc/mtab line {} : '{}'", line_no, line),
                ));
            }

            let device_name = columns[0];
            if device_name.starts_with("/dev/") {
                let mount = Mount {
                    mountpoint: PathBuf::from(columns[1]),
                    fs_type: columns[2].to_string(),
                };

                debug!("from_mtab: processing mount {:?}", mount);
                mounts.insert(PathBuf::from(device_name), mount);
            } else {
                trace!("from_mtab: not processing line {}", line);
            }
        }

        Ok(mounts)
    }
}
