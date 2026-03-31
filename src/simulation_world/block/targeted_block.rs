use crate::prelude::*;
use bevy::ecs::prelude::*;

#[derive(Resource, Clone, Default)]
pub struct TargetedBlock {
    pub position: Option<IVec3>,
    pub normal: Option<IVec3>,
}
