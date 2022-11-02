use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
    },
    scene::{
        base::BaseBuilder, camera::CameraBuilder, graph::Graph, node::Node, pivot::PivotBuilder,
        transform::TransformBuilder,
    },
};

pub struct CameraController {
    pivot: Handle<Node>,
    camera: Handle<Node>,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl CameraController {
    pub fn new(graph: &mut Graph) -> Self {
        let camera;
        let pivot = PivotBuilder::new(BaseBuilder::new().with_children(&[{
            camera = CameraBuilder::new(
                BaseBuilder::new().with_local_transform(TransformBuilder::new().build()),
            )
            .with_z_far(48.0)
            .build(graph);
            camera
        }]))
        .build(graph);

        Self {
            pivot,
            camera,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }

    pub fn rotate_by(&mut self, yaw: f32, pitch: f32, roll: f32) {
        self.yaw += yaw;
        self.pitch += pitch;
        self.roll += roll;
    }

    pub fn update(&mut self, graph: &mut Graph) {
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw)
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), self.roll);

        graph[self.pivot]
            .local_transform_mut()
            .set_rotation(rotation);
    }

    pub fn get_rotation(&self, graph: &Graph) -> UnitQuaternion<f32> {
        **graph[self.pivot].local_transform().rotation()
    }

    pub fn set_rotation(&mut self, graph: &mut Graph, rotation: UnitQuaternion<f32>) {
        graph[self.pivot]
            .local_transform_mut()
            .set_rotation(rotation);
    }

    pub fn set_position(&mut self, graph: &mut Graph, position: Vector3<f32>) {
        graph[self.pivot]
            .local_transform_mut()
            .set_position(position);
    }
}
