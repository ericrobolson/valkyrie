mod child_descriptor;
use child_descriptor::ChildDescriptor;

/// Voxel octree. For now, going with a naive approach. Later on will look into optimizing with the Nvidia Efficient Sparse Voxel Octrees

pub type MaterialId = u16;

/// PBR based material
pub struct Material {
    id: MaterialId,
}

/// Voxel octree
pub struct Voxtree {
    /// The length of a Voctree cube side, in mm.
    world_length_mm: u64,
    /// The entirety of the materials that make up the world
    world_materials: Vec<Material>,

    /// The children in the voctree
    children: Vec<ChildDescriptor>,
}
