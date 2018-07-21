// acl.rs

#![no_std]

extern crate libm;

#[macro_use]
mod ext4;
mod memory;

// Convert from filesystem to in-memory representation
pub fn ext4_acl_from_disk(size: size_t) {
    let end: char = value + size;
    let mut n: i32;
    let mut count: i32;

    /* if (!value)
          return NULL; */

    if size < sizeof(ext4_acl_header) {
        return ERR_PTR;
    }

    let mut temp = ext4_acl_header * value;
    /*	if (((ext4_acl_header *)value)->a_version !=
	    cpu_to_le32(EXT4_ACL_VERSION))
		return ERR_PTR(-EINVAL); */

    value = value + sizeof(ext4_acl_header);
    count = ext4_acl_count(size);
    if count < 0 { 
        return ERR_PTR;
    }

    if count == 0 {
        return NULL;
    }
    acl = posix_acl_alloc(count, GFP_NOFS);
    /* 	if (!acl)
		return ERR_PTR(-ENOMEM); */
    
    loop {
        /* ext4_acl_entry *entry =
			(ext4_acl_entry *)value;
		if ((char *)value + sizeof(ext4_acl_entry_short) > end)
			goto fail;
		acl->a_entries[n].e_tag  = le16_to_cpu(entry->e_tag);
		acl->a_entries[n].e_perm = le16_to_cpu(entry->e_perm);

		switch (acl->a_entries[n].e_tag) {
		case ACL_USER_OBJ:
		case ACL_GROUP_OBJ:
		case ACL_MASK:
		case ACL_OTHER:
			value = (char *)value +
				sizeof(ext4_acl_entry_short);
			break;

		case ACL_USER:
			value = (char *)value + sizeof(ext4_acl_entry);
			if ((char *)value > end)
				goto fail;
			acl->a_entries[n].e_uid =
				make_kuid(&init_user_ns,
					  le32_to_cpu(entry->e_id));
			break;
		case ACL_GROUP:
			value = (char *)value + sizeof(ext4_acl_entry);
			if ((char *)value > end)
				goto fail;
			acl->a_entries[n].e_gid =
				make_kgid(&init_user_ns,
					  le32_to_cpu(entry->e_id));
			break;

		default:
			goto fail;
		} */

        n = n - 1;
        if n == 0 { break; }
    }

    if value == end {
        panic!("Error in acl.rs; line 84.");
    } else {
        // goto fail;
    }
    return acl;
}
