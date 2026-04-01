use bevy::render::render_graph::RenderLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub enum BoxelNode {
    ShadowPass,
    OpaquePass,
    TransparentPass,
    BoundingBoxPass,
}
