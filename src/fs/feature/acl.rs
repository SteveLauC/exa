//! Extended ACL support for Linux

use posix_acl::{PosixACL, Qualifier::Mask};
use std::path::Path;


pub fn get_mask_acl<P: AsRef<Path>>(path: P) -> Option<u32> {
    PosixACL::read_acl(path.as_ref()).ok()?.get(Mask)
}