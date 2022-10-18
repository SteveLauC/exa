//! Extended ACL support for Linux

use std::{fs::metadata, path::Path};
use posix_acl::{
    Qualifier::{GroupObj, Other, UserObj, Mask},
    PosixACL,
};

pub fn dir_has_default_acl<P: AsRef<Path>>(path: P) -> bool {
    let default_acl_reader = PosixACL::read_default_acl(path.as_ref()).unwrap();

    default_acl_reader.get(UserObj).is_some() ||
        default_acl_reader.get(GroupObj).is_some() ||
        default_acl_reader.get(Other).is_some()
}

pub fn file_has_extended_access_acl<P: AsRef<Path>>(path: P) -> bool {
    PosixACL::read_acl(path.as_ref()).unwrap().get(Mask).is_some()
}

pub fn has_acl<P: AsRef<Path>>(path: P) -> bool {
    metadata(path.as_ref()).unwrap().is_dir() && dir_has_default_acl(path.as_ref()) ||
        file_has_extended_access_acl(path.as_ref())
}