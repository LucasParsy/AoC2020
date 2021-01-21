#[cfg(feature = "render_3d")]
pub mod mesh_anim {
    use std::time::Duration;
    use kiss3d::{nalgebra::{Point3, UnitQuaternion, Vector3}, scene::SceneNode};

    pub trait MeshAnimation {
        fn new() -> Self
        where
            Self: Sized;
        fn next(&mut self, mesh: &mut SceneNode, delta: &Duration) -> bool;
    }

    pub struct CubeDecomposer {
        timer: Duration,
        anim_speed: f32,
    }

    impl MeshAnimation for CubeDecomposer {
        fn new() -> Self {
            CubeDecomposer {
                timer: Duration::new(0, 0),
                anim_speed: 0.3,
            }
        }

        fn next(&mut self, cube: &mut SceneNode, delta: &Duration) -> bool {
            let mut res = false;
            self.timer += *delta;
            if self.timer.as_secs_f32() >= self.anim_speed {
                self.timer = Duration::new(0, 0);
                cube.modify_faces(&mut |faces: &mut Vec<Point3<u16>>| {
                    //for (index, face) in faces.iter().enumerate() {
                    //        println!("face {} {}", index, face);
                    //}
                    if faces.pop() == None {
                        res = true;
                    }
                });
            }
            res
        }
    }

    pub struct CubeRecomposer {
        timer: Duration,
        anim_speed: f32,
        faces: Vec<Point3<u16>>,
        is_start: bool,
    }

    impl MeshAnimation for CubeRecomposer {
        fn new() -> Self {
            CubeRecomposer {
                timer: Duration::new(0, 0),
                anim_speed: 0.3,
                faces: Vec::new(),
                is_start: true,
            }
        }

        fn next(&mut self, cube: &mut SceneNode, delta: &Duration) -> bool {
            if self.is_start {
                self.is_start = false;
                cube.modify_faces(&mut |cube_faces| self.faces.append(cube_faces));
                self.faces.reverse();
            }
            if self.faces.is_empty() {
                return true;
            }

            self.timer += *delta;
            if self.timer.as_secs_f32() >= self.anim_speed {
                self.timer = Duration::new(0, 0);
                cube.modify_faces(&mut |faces: &mut Vec<Point3<u16>>| {
                    faces.push(self.faces.pop().unwrap());
                });
            }
            false
        }
    }

    pub struct CubeReducer {
        scale: f32,
        anim_speed: f32,
    }

    impl MeshAnimation for CubeReducer {
        fn new() -> Self {
            CubeReducer {
                scale: 1.0,
                anim_speed: 0.75,
            }
        }

        fn next(&mut self, cube: &mut SceneNode, delta: &Duration) -> bool {
            self.scale -= delta.as_secs_f32() * self.anim_speed;
            if self.scale.is_sign_negative() {
                return true;
            }
            //println!("delta: {:?}", delta);
            cube.set_local_scale(self.scale, self.scale, self.scale);
            false
        }
    }

    pub struct CubeExpander {
        scale: f32,
        anim_speed: f32,
    }

    impl MeshAnimation for CubeExpander {
        fn new() -> Self {
            CubeExpander {
                scale: 0.0,
                anim_speed: 0.75,
            }
        }

        fn next(&mut self, cube: &mut SceneNode, delta: &Duration) -> bool {
            self.scale += delta.as_secs_f32() * self.anim_speed;
            if self.scale >= 0.9 {
                return true;
            }
            //println!("delta: {:?}", delta);
            cube.set_local_scale(self.scale, self.scale, self.scale);
            false
        }
    }

    pub struct CubeTest {
        timer: Duration,
        anim_speed: f32,
    }

    impl MeshAnimation for CubeTest {
        fn new() -> Self {
            CubeTest {
                timer: Duration::new(0, 0),
                anim_speed: 0.1,
            }
        }

        fn next(&mut self, cube: &mut SceneNode, delta: &Duration) -> bool {
            let mut res = false;
            self.timer += *delta;
            self.timer = Duration::new(0, 0);
            let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
            let rot2 = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.014);

            cube.prepend_to_local_rotation(&rot);
            cube.prepend_to_local_rotation(&rot2);

            cube.modify_vertices(&mut |vertices: &mut Vec<Point3<f32>>| {
                for (index, vert) in vertices.iter_mut().enumerate() {
                    println!("face {} {} {} {}", index, vert.x, vert.y, vert.z);
                    res = true;

                    for elem in vert.iter_mut() {
                        if elem.abs() >= 0.01 {
                            let sign = if *elem < 0.0 { -1.0 } else { 1.0 };
                            *elem -= delta.as_secs_f32() * self.anim_speed *  sign;
                            res = false;
                            break;
                        }
                    }
                }
            });
            res
        }
    }
}
