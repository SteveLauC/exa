//! Extended ACL support for Linux

use posix_acl::{
    PosixACL,
    Qualifier::{GroupObj, Mask, Other, UserObj},
};
use std::{fs::symlink_metadata, path::Path};

pub fn dir_has_default_acl<P: AsRef<Path>>(path: P) -> bool {
    let acl = PosixACL::read_default_acl(path.as_ref()).unwrap();
    acl.get(UserObj).is_some() || acl.get(GroupObj).is_some() || acl.get(Other).is_some()
}

pub fn file_has_extended_access_acl<P: AsRef<Path>>(path: P) -> bool {
    PosixACL::read_acl(path.as_ref())
        .unwrap()
        .get(Mask)
        .is_some()
}

pub fn has_acl<P: AsRef<Path>>(path: P) -> bool {
    let metadata = symlink_metadata(path.as_ref()).unwrap();
    if metadata.is_symlink() {
        return false;
    } else {
        symlink_metadata(path.as_ref()).unwrap().is_dir() && dir_has_default_acl(path.as_ref())
            || file_has_extended_access_acl(path.as_ref())
    }
}
