//! Extended ACL support for Linux

use posix_acl::{PosixACL, Qualifier::Mask};
use std::{fs::metadata, path::Path};

pub fn has_acl<P: AsRef<Path>>(path: P) -> bool {
    metadata(path.as_ref()).unwrap().is_dir()
        && PosixACL::read_default_acl(path.as_ref())
            .unwrap()
            .get(Mask)
            .is_some()
        || (PosixACL::read_acl(path.as_ref()).unwrap().get(Mask)).is_some()
}