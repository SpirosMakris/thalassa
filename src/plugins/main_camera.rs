use bevy::{
    prelude::*,
    render::camera::{Camera, PerspectiveProjection, VisibleEntities},
    render::render_graph::base
};

const CAMERA_SPEED: f32 = 1000.0;

/// A set of options for MainCamera
pub struct MainCameraOptions {
    /// Speed the camera moves at
    pub speed: f32,
    /// Mouse related camera motion sensitivity
    pub sensitivity: f32,
    /// Camera current pitch. Enforced by MainCameraPlugin motion system
    pub pitch: f32,
    /// Camera current yaw. Enforced by MainCameraPlugin motion system
    pub yaw: f32,
}

impl Default for MainCameraOptions {
    fn default() -> Self {
        Self {
            speed: 20.0,
            sensitivity: 3.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

/** MainCamera bundle
 * Spawn this to instantiate a MainCamera
**/
#[derive(Bundle)]
struct MainCamera {
    pub options: MainCameraOptions,
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            options: MainCameraOptions::default(),
            camera: Camera {
                name: Some(base::camera::CAMERA3D.to_string()),
                ..Default::default()
            },
            perspective_projection: PerspectiveProjection::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}


pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(ss_setup_main_camera.system())
            .add_system(s_camera_movement.system());
    }
}

fn _________ss_setup_main_camera(mut commands: Commands) {
    commands.spawn(Camera3dComponents {
        transform: Transform::new_sync_disabled(Mat4::face_toward(
            Vec3::new(50.0, 100.0, 100.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )),
        ..Default::default()
    })
    .with(MainCamera::default());
}

fn ss_setup_main_camera(mut commands: Commands) {
    commands.spawn(MainCamera {
        translation: Translation(Vec3::new(0.0, 50.0, 50.0)),
        rotation: Rotation(Quat::from_rotation_x(-std::f32::consts::PI / 5.0)),
        ..Default::default()
    });
}

fn s_camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&MainCameraOptions, &mut Translation, &Rotation)>
) {
    let axis_h = movement_axis(&keyboard_input, KeyCode::Left, KeyCode::Right);
    let axis_v = movement_axis(&keyboard_input, KeyCode::Up, KeyCode::Down);
    //println!("V: {}", axis_v);

    let axis_elev = movement_axis(&keyboard_input, KeyCode::PageUp, KeyCode::PageDown);

    for (options, mut translation, rotation) in &mut query.iter() {
        let delta_forw = flat_forward_vector(rotation)
            * axis_v
            * options.speed
            * time.delta_seconds;
        
        let delta_strafe = flat_strafe_vector(rotation)
            * axis_h
            * options.speed
            * time.delta_seconds;
        
        let delta_elev = Vec3::unit_y()
            * axis_elev
            * options.speed
            * time.delta_seconds;
            
        
        translation.0 += delta_forw + delta_strafe + delta_elev;
        // println!("{:?}", translation.0);
    }
}

fn movement_axis(
    input: &Res<Input<KeyCode>>,
    axis_pos_key: KeyCode,
    axis_neg_key: KeyCode
) -> f32 {
    let mut axis = 0.0;

    if input.pressed(axis_pos_key) {
        axis += 1.0;
    }

    if input.pressed(axis_neg_key) {
        axis -= 1.0;
    }

    axis
}

/// Gets a unit forward vector (Z axis)
fn forward_vector(rotation: &Rotation) -> Vec3 {
    rotation.mul_vec3(Vec3::unit_z()).normalize()
}

/// Get a `flattened` (i.e Y = 0) forward unit vector (Z axis)
fn flat_forward_vector(rotation: &Rotation) -> Vec3 {
    let f = forward_vector(rotation);
    let f_flat = Vec3::new(f.x(), 0.0, f.z()).normalize();

    f_flat
}

/// Get a strafe (left-right axis) unit vector
fn flat_strafe_vector(rotation: &Rotation) -> Vec3 {
    // Rotate flat forward vector by 90 degrees
    Rotation::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(flat_forward_vector(rotation))
        .normalize()
}

fn _______s_camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&MainCamera, &Camera, &mut Translation)>
) {
    for (_, _, mut translation) in &mut query.iter() {
        let mut direction = Vec3::zero();

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        translation.0 += time.delta_seconds * direction * CAMERA_SPEED;
    }
}
