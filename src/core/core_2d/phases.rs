
use core::ops::Range;

use bevy::asset::UntypedAssetId;
use bevy::render::render_phase::PhaseItemBatchSetKey;

use bevy::ecs::prelude::*;
use bevy::math::FloatOrd;
use bevy::render::{
    render_phase::{
        BinnedPhaseItem, CachedRenderPipelinePhaseItem, DrawFunctionId,
        PhaseItem, PhaseItemExtraIndex, SortedPhaseItem,
    },
    render_resource::{
        BindGroupId, CachedRenderPipelineId,
    },
    sync_world::MainEntity,
};


/// Opaque 2D [`BinnedPhaseItem`]s.
pub struct Opaque2d {
    /// Determines which objects can be placed into a *batch set*.
    ///
    /// Objects in a single batch set can potentially be multi-drawn together,
    /// if it's enabled and the current platform supports it.
    pub batch_set_key: BatchSetKey2d,
    /// The key, which determines which can be batched.
    pub bin_key: Opaque2dBinKey,
    /// An entity from which data will be fetched, including the mesh if
    /// applicable.
    pub representative_entity: (Entity, MainEntity),
    /// The ranges of instances.
    pub batch_range: Range<u32>,
    /// An extra index, which is either a dynamic offset or an index in the
    /// indirect parameters list.
    pub extra_index: PhaseItemExtraIndex,
}

/// Data that must be identical in order to batch phase items together.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Opaque2dBinKey {
    /// The identifier of the render pipeline.
    pub pipeline: CachedRenderPipelineId,
    /// The function used to draw.
    pub draw_function: DrawFunctionId,
    /// The asset that this phase item is associated with.
    ///
    /// Normally, this is the ID of the mesh, but for non-mesh items it might be
    /// the ID of another type of asset.
    pub asset_id: UntypedAssetId,
    /// The ID of a bind group specific to the material.
    pub material_bind_group_id: Option<BindGroupId>,
}

impl PhaseItem for Opaque2d {
    #[inline]
    fn entity(&self) -> Entity {
        self.representative_entity.0
    }

    fn main_entity(&self) -> MainEntity {
        self.representative_entity.1
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.bin_key.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }

    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
}

impl BinnedPhaseItem for Opaque2d {
    // Since 2D meshes presently can't be multidrawn, the batch set key is
    // irrelevant.
    type BatchSetKey = BatchSetKey2d;

    type BinKey = Opaque2dBinKey;

    fn new(
        batch_set_key: Self::BatchSetKey,
        bin_key: Self::BinKey,
        representative_entity: (Entity, MainEntity),
        batch_range: Range<u32>,
        extra_index: PhaseItemExtraIndex,
    ) -> Self {
        Opaque2d {
            batch_set_key,
            bin_key,
            representative_entity,
            batch_range,
            extra_index,
        }
    }
}

/// 2D meshes aren't currently multi-drawn together, so this batch set key only
/// stores whether the mesh is indexed.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BatchSetKey2d {
    /// True if the mesh is indexed.
    pub indexed: bool,
}

impl PhaseItemBatchSetKey for BatchSetKey2d {
    fn indexed(&self) -> bool {
        self.indexed
    }
}

impl CachedRenderPipelinePhaseItem for Opaque2d {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.bin_key.pipeline
    }
}

/// Alpha mask 2D [`BinnedPhaseItem`]s.
pub struct AlphaMask2d {
    /// Determines which objects can be placed into a *batch set*.
    ///
    /// Objects in a single batch set can potentially be multi-drawn together,
    /// if it's enabled and the current platform supports it.
    pub batch_set_key: BatchSetKey2d,
    /// The key, which determines which can be batched.
    pub bin_key: AlphaMask2dBinKey,
    /// An entity from which data will be fetched, including the mesh if
    /// applicable.
    pub representative_entity: (Entity, MainEntity),
    /// The ranges of instances.
    pub batch_range: Range<u32>,
    /// An extra index, which is either a dynamic offset or an index in the
    /// indirect parameters list.
    pub extra_index: PhaseItemExtraIndex,
}

/// Data that must be identical in order to batch phase items together.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlphaMask2dBinKey {
    /// The identifier of the render pipeline.
    pub pipeline: CachedRenderPipelineId,
    /// The function used to draw.
    pub draw_function: DrawFunctionId,
    /// The asset that this phase item is associated with.
    ///
    /// Normally, this is the ID of the mesh, but for non-mesh items it might be
    /// the ID of another type of asset.
    pub asset_id: UntypedAssetId,
    /// The ID of a bind group specific to the material.
    pub material_bind_group_id: Option<BindGroupId>,
}

impl PhaseItem for AlphaMask2d {
    #[inline]
    fn entity(&self) -> Entity {
        self.representative_entity.0
    }

    #[inline]
    fn main_entity(&self) -> MainEntity {
        self.representative_entity.1
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.bin_key.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }

    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
}

impl BinnedPhaseItem for AlphaMask2d {
    // Since 2D meshes presently can't be multidrawn, the batch set key is
    // irrelevant.
    type BatchSetKey = BatchSetKey2d;

    type BinKey = AlphaMask2dBinKey;

    fn new(
        batch_set_key: Self::BatchSetKey,
        bin_key: Self::BinKey,
        representative_entity: (Entity, MainEntity),
        batch_range: Range<u32>,
        extra_index: PhaseItemExtraIndex,
    ) -> Self {
        AlphaMask2d {
            batch_set_key,
            bin_key,
            representative_entity,
            batch_range,
            extra_index,
        }
    }
}

impl CachedRenderPipelinePhaseItem for AlphaMask2d {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.bin_key.pipeline
    }
}

/// Transparent 2D [`SortedPhaseItem`]s.
pub struct Transparent2d {
    pub sort_key: FloatOrd,
    // pub entity: (Entity, MainEntity),
    pub entity: Entity,
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
    pub batch_range: Range<u32>,
    // pub extracted_index: usize,
    pub extra_index: PhaseItemExtraIndex,
    // Whether the mesh in question is indexed (uses an index buffer in
    // addition to its vertex buffer).
    // pub indexed: bool,
}

impl PhaseItem for Transparent2d {
    #[inline]
    fn entity(&self) -> Entity {
        // self.entity.0
        self.entity
    }

    #[inline]
    fn main_entity(&self) -> MainEntity {
        // self.entity.1
        MainEntity::from(Entity::PLACEHOLDER)
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    #[inline]
    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }

    #[inline]
    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
}

impl SortedPhaseItem for Transparent2d {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        self.sort_key
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        // radsort is a stable radix sort that performed better than `slice::sort_by_key` or `slice::sort_unstable_by_key`.
        radsort::sort_by_key(items, |item| item.sort_key().0);
    }

    fn indexed(&self) -> bool {
        // self.indexed
        false
    }
}

impl CachedRenderPipelinePhaseItem for Transparent2d {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

