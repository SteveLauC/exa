//! Extended ACL support for Linux

use posix_acl::{
    PosixACL,
    Qualifier::{GroupObj, Mask, Other, UserObj},
};
use std::{fs::symlink_metadata, path::Path};

pub fn dir_has_default_acl<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(acl) = PosixACL::read_default_acl(path.as_ref()) {
        return acl.get(UserObj).is_some() || acl.get(GroupObj).is_some() || acl.get(Other).is_some();
    }

    // This fs may does not support ACL
    false
}

pub fn file_has_extended_access_acl<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(acl) = PosixACL::read_acl(path.as_ref()) {
        return acl.get(Mask).is_some();
    }
    // This fs may does not support ACL
    false
}

pub fn has_acl<P: AsRef<Path>>(path: P) -> bool {
    let metadata = symlink_metadata(path.as_ref()).unwrap();
    if metadata.is_symlink() {
        false
    } else if metadata.is_dir() {
        dir_has_default_acl(path.as_ref())
    } else{
        file_has_extended_access_acl(path.as_ref())
    }
}
