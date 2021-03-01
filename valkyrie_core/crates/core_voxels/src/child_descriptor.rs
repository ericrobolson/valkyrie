/// Bitmask representing voxel indexes.
pub type VoxelIndex = u8;

/// A voxel chunk. Internally it's a u64, but this enables shared functionality.
pub struct ChildDescriptor {
    /// The 16 bit pointer to the children.
    child_pointer: u16,
    /// Whether a valid voxel is at the given index
    valid_mask: u8,
    /// Whether the voxel is a leaf or not
    leaf_mask: u8,

    // If this has no children, get the material value

    // reserved bits for misc stuff
    reserved: u32,
}

impl ChildDescriptor {
    /// Returns the relative address to the children.
    pub fn child_pointer(chunk: &ChildDescriptor) -> u16 {
        chunk.child_pointer
    }

    /// Returns whether the chunk is empty or not
    pub fn is_empty(chunk: &ChildDescriptor) -> bool {
        chunk.valid_mask == 0 && chunk.leaf_mask == 0
    }
    /// Returns whether the child is empty or not
    pub fn empty_child(child: VoxelIndex, chunk: &ChildDescriptor) -> bool {
        (child & (chunk.valid_mask | chunk.leaf_mask)) == 0
    }
    /// Returns whether the child is a leaf or not
    pub fn leaf_child(child: VoxelIndex, chunk: &ChildDescriptor) -> bool {
        (child & chunk.valid_mask & chunk.leaf_mask) != 0
    }

    fn to_raw(&self) -> u64 {
        todo!()
    }

    fn from_raw(raw: u64) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ChildDescriptor_child_pointer_returns_expected_without_far() {
        let mut chunk = ChildDescriptor {
            child_pointer: u16::MAX,
            valid_mask: VoxelIndex::MAX,
            leaf_mask: VoxelIndex::MAX,
            reserved: 0,
        };

        assert_eq!(u16::MAX, ChildDescriptor::child_pointer(&chunk));

        let mut chunk = ChildDescriptor {
            child_pointer: u16::MAX - 234,
            valid_mask: VoxelIndex::MAX,
            leaf_mask: VoxelIndex::MAX,
            reserved: 0,
        };

        assert_eq!(u16::MAX - 234, ChildDescriptor::child_pointer(&chunk));
    }

    #[test]
    fn ChildDescriptor_empty_child_is_empty_returns_true() {
        let mut i = 1;
        for j in 0..8 {
            let mut chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: VoxelIndex::MAX,
                leaf_mask: VoxelIndex::MAX,
                reserved: 0,
            };

            let neg = !(i << j);

            chunk.valid_mask = neg;
            chunk.leaf_mask = neg;

            assert_eq!(true, ChildDescriptor::empty_child(i << j, &chunk));
            assert_eq!(false, ChildDescriptor::is_empty(&chunk));
        }
    }

    #[test]
    fn ChildDescriptor_empty_child_not_empty_returns_false() {
        let mut i = 1;
        for j in 0..8 {
            let mut chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: VoxelIndex::MAX,
                leaf_mask: VoxelIndex::MAX,
                reserved: 0,
            };

            let neg = !(i << j);

            chunk.leaf_mask = neg;

            assert_eq!(false, ChildDescriptor::empty_child(i << j, &chunk));
        }
        let mut i = 1;
        for j in 0..8 {
            let mut chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: VoxelIndex::MAX,
                leaf_mask: VoxelIndex::MAX,
                reserved: 0,
            };

            let neg = !(i << j);

            chunk.valid_mask = neg;

            assert_eq!(false, ChildDescriptor::empty_child(i << j, &chunk));
        }
        let mut i = 1;
        for j in 0..8 {
            let mut chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: VoxelIndex::MAX,
                leaf_mask: VoxelIndex::MAX,
                reserved: 0,
            };

            assert_eq!(false, ChildDescriptor::empty_child(i << j, &chunk));
        }
    }

    #[test]
    fn ChildDescriptor_leaf_child_is_a_leaf_returns_true() {
        let mut chunk = ChildDescriptor {
            child_pointer: 0,
            valid_mask: 0,
            leaf_mask: 0,
            reserved: 0,
        };

        let mut i = 1;
        for j in 0..8 {
            chunk.valid_mask = i << j;
            chunk.leaf_mask = i << j;

            assert_eq!(true, ChildDescriptor::leaf_child(i << j, &chunk));
        }
    }

    #[test]
    fn ChildDescriptor_leaf_child_not_a_leaf_returns_false() {
        let mut i = 1;
        for j in 0..8 {
            let chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: i << j,
                leaf_mask: 0,
                reserved: 0,
            };
            assert_eq!(false, ChildDescriptor::leaf_child(i << j, &chunk));
        }

        let mut i = 1;
        for j in 0..8 {
            let chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: 0,
                leaf_mask: i << j,
                reserved: 0,
            };

            assert_eq!(false, ChildDescriptor::leaf_child(i << j, &chunk));
        }
    }

    #[test]
    fn ChildDescriptor_is_empty_all_empty_returns_true() {
        let chunk = ChildDescriptor {
            child_pointer: 0,
            valid_mask: 0,
            leaf_mask: 0,
            reserved: 0,
        };

        let mut i = 1;
        assert_eq!(true, ChildDescriptor::is_empty(&chunk));
    }

    #[test]
    fn ChildDescriptor_is_empty_not_empty_returns_false() {
        let mut i = 1;
        for j in 0..8 {
            let chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: i << j,
                leaf_mask: 0,
                reserved: 0,
            };

            assert_eq!(false, ChildDescriptor::is_empty(&chunk));
        }

        let mut i = 1;
        for j in 0..8 {
            let chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: 0,
                leaf_mask: i << j,
                reserved: 0,
            };
            assert_eq!(false, ChildDescriptor::is_empty(&chunk));
        }

        let mut i = 1;
        for j in 0..8 {
            let chunk = ChildDescriptor {
                child_pointer: 0,
                valid_mask: i << j,
                leaf_mask: i << j,
                reserved: 0,
            };
            assert_eq!(false, ChildDescriptor::is_empty(&chunk));
        }
    }
}
