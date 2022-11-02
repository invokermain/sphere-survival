use std::sync::Arc;

use fyrox::{
    core::{
        algebra::{Matrix4, Vector3},
        inspect::prelude::*,
        parking_lot::lock_api::Mutex,
        pool::Handle,
        reflect::Reflect,
        visitor::prelude::*,
    },
    scene::{
        base::BaseBuilder,
        collider::{ColliderBuilder, ColliderShape},
        mesh::{
            surface::{SurfaceBuilder, SurfaceData, SurfaceSharedData},
            MeshBuilder,
        },
        node::Node,
        rigidbody::{RigidBodyBuilder, RigidBodyType},
        Scene,
    },
};

#[derive(Visit, Reflect, Inspect, Default, Debug, Clone, Copy)]
pub struct Ball {
    body: Handle<Node>,
}

impl Ball {
    pub fn new(scene: &mut Scene, position: Vector3<f32>) -> Self {
        let body;
        let root = MeshBuilder::new(
            BaseBuilder::new().with_name("Ball").with_children(&[
                {
                    body = RigidBodyBuilder::new(BaseBuilder::new())
                        .with_body_type(RigidBodyType::Dynamic)
                        .with_gravity_scale(0.0)
                        .with_ccd_enabled(true)
                        .build(&mut scene.graph);
                    body
                },
                ColliderBuilder::new(BaseBuilder::new().with_name("PlayerCollider"))
                    .with_shape(ColliderShape::ball(5.0))
                    .with_friction(0.0)
                    .build(&mut scene.graph),
            ]),
        )
        .with_surfaces(vec![SurfaceBuilder::new(SurfaceSharedData::new(
            SurfaceData::make_sphere(
                32,
                32,
                1.0,
                &Matrix4::new_nonuniform_scaling(&Vector3::new(5.0, 5.0, 5.0)),
            ),
        ))
        .build()])
        .build(&mut scene.graph);

        let transform: &mut fyrox::scene::transform::Transform =
            scene.graph[body].as_rigid_body_mut().local_transform_mut();

        transform.set_position(position);

        Self { body }
    }
}
