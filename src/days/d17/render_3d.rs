#[cfg(feature = "render_3d")]
pub mod render {
    extern crate kiss3d;

    use kiss3d::{
        camera::ArcBall,
        event::WindowEvent,
        light::Light,
        nalgebra::{Point2, Point3, Translation3, UnitQuaternion, Vector3},
        scene::SceneNode,
        text::Font,
        window::Window,
    };

    use std::{
        collections::HashMap,
        path::Path,
        rc::Rc,
        time::{Duration, Instant},
    };

    use super::super::mesh_animations::mesh_anim::{CubeExpander, CubeReducer, MeshAnimation};

    type CubeMap = HashMap<(i8, usize, usize), (SceneNode, Option<Box<dyn MeshAnimation>>, bool)>;

    struct TextInfo {
        text: String,
        pos: Point2<f32>,
        color: Point3<f32>,
        font: Rc<Font>,
    }

    type WorldVec = Vec<Vec<Vec<Vec<(bool, bool)>>>>;

    struct Day17Render {
        world: WorldVec,
        start: usize,
        map_size: usize,

        window: Window,
        camera: ArcBall,
        cube_parent: SceneNode,

        level: usize,
        map: CubeMap,
        center: f32,

        gui_text: TextInfo,
        timer_levels: Duration,
        play_camera_animation: bool,

        //constants
        cycles: usize,
        camera_dist: f32,
        camera_step_dist: f32,
        time_between_levels: u64,
    }

    impl Day17Render {
        fn gen_text_info() -> TextInfo {
            TextInfo {
                text: "step 0".into(),
                pos: Point2::new(50.0, 50.0),
                color: Point3::new(1.0, 1.0, 1.0),
                font: Font::default(),
            }
        }

        fn setup_grid(window: &mut Window, cycles: f32) {
            let mut rectangle = window.add_quad(800.0, 800.0, 2, 2);
            rectangle.set_local_translation(Translation3::new(0.0, -(50.0 + cycles * 3.0), 0.0));
            rectangle.set_local_rotation(UnitQuaternion::new(
                Vector3::x() * std::f32::consts::FRAC_PI_2,
            ));
            let path = Path::new("./resources/checker.png");
            match path.exists() {
                true => rectangle.set_texture_from_file(path, "checker"),
                false => rectangle.set_color(0.0, 0.058, 0.521),
            };
        }

        fn setup_scene(cycles: usize) -> (Window, ArcBall, SceneNode) {
            let mut window = Window::new_with_size("day 17 Aoc", 1900, 1000);
            window.set_framerate_limit(Some(60));
            //window.set_background_color(0.039, 0.0, 0.360);
            window.set_light(Light::StickToCamera);
            Day17Render::setup_grid(&mut window, cycles as f32);

            let cube_group = window.add_group();
            let camera = ArcBall::new(Point3::new(0.0, 0.0, 20.0), Point3::origin());
            (window, camera, cube_group)
        }

        pub fn new(input: &[String]) -> Self {
            let cycles = 20;

            let (world, start, map_size) = crate::days::d17::init_world(input, false, cycles);
            let center = (world[0][0].len() / 2) as f32;
            let map: CubeMap = HashMap::new();

            let (window, camera, cube_parent) = Day17Render::setup_scene(cycles);

            Day17Render {
                world,
                start,
                map_size,
                window,
                camera,
                cube_parent,
                level: 0,
                map,
                center,
                gui_text: Day17Render::gen_text_info(),
                timer_levels: Duration::new(0, 0),
                play_camera_animation: true,

                cycles,
                camera_dist: 20.0,
                camera_step_dist: 2.0,
                time_between_levels: 5,
            }
        }

        fn check_events(&mut self) {
            for event in self.window.events().iter() {
                match event.value {
                    WindowEvent::Scroll(_, _, _) | WindowEvent::MouseButton(_, _, _) => {
                        self.play_camera_animation = false;
                    }
                    _ => {}
                }
            }
        }

        pub fn render(&mut self) -> i64 {
            self.update_display_state();
            self.level += 1;

            let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
            let mut start_time = Instant::now();

            //let mut test_cube = self.window.add_cube(0.9, 0.9, 0.9);
            //test_cube.set_color(0.5, 0.0, 0.675);
            //let mut modifier = CubeTest::new();

            while self.window.render_with_camera(&mut self.camera) {
                let delta = start_time.elapsed();
                start_time = Instant::now();
                self.timer_levels += delta;

                //if self.timer_levels.as_secs() >= 4 {
                //    modifier.next(&mut test_cube, &delta);
                //}
                if self.timer_levels.as_secs() >= self.time_between_levels
                    && self.level <= self.cycles
                {
                    self.step_cycle();
                }

                self.check_events();
                if self.play_camera_animation {
                    self.recoil_camera(&delta);
                    self.cube_parent.prepend_to_local_rotation(&rot);
                }
                self.animate_cubes(&delta);
                let info = &self.gui_text;
                self.window
                    .draw_text(&info.text, &info.pos, 100.0, &info.font, &info.color);
            }
            super::super::count_active(&self.world)
        }

        fn animate_cubes(&mut self, delta: &Duration) {
            for cube in self.map.values_mut() {
                match &mut cube.1 {
                    Some(anim) => {
                        if anim.as_mut().next(&mut cube.0, delta) {
                            cube.1 = None;
                            if !cube.2 {
                                cube.0.unlink();
                            }
                        }
                    }
                    None => {}
                };
            }
        }

        fn recoil_camera(&mut self, delta: &Duration) {
            if self.level <= self.cycles {
                self.camera_dist +=
                    self.camera_step_dist * (delta.as_secs_f32() / self.time_between_levels as f32);
                self.camera.set_dist(self.camera_dist);
            }
        }

        fn step_cycle(&mut self) {
            self.start -= 1;
            self.timer_levels = Duration::new(0, 0);
            crate::days::d17::step_life(&mut self.world, self.level, self.start, self.map_size, true);
            self.update_display_state();
            super::super::update_cells(&mut self.world);
            let count_cubes = super::super::count_active(&self.world);
            self.gui_text.text = format!("step {}, {} cubes", self.level, count_cubes);
            self.level += 1;
        }

        fn update_display_state(&mut self) {
            let levels = self.level as i8;

            for z in -levels..=levels {
                let level = &self.world[0][z.abs() as usize];
                for (y, line) in level.iter().enumerate() {
                    for (x, elem) in line.iter().enumerate() {
                        if elem.1 && elem.0 {
                            if let Some(cube) = self.map.get_mut(&(z, y, x)) {
                                let animation: Option<Box<dyn MeshAnimation>> =
                                    Some(Box::new(CubeReducer::new()));
                                cube.1 = animation;
                                cube.2 = false;
                            }
                        } else if (elem.1 && !elem.0) || (elem.0 && levels == 0) {
                            let (fx, fy, fz) = (x as f32, y as f32, z as f32);
                            let mut c = self.cube_parent.add_cube(0.9, 0.9, 0.9);

                            let animation: Option<Box<dyn MeshAnimation>> =
                                Some(Box::new(CubeExpander::new()));

                            c.set_local_translation(Translation3::new(
                                fx - self.center,
                                fy - self.center,
                                fz,
                            ));
                            c.set_color(1.0, 0.0, 0.0);
                            self.map.insert((z, y, x), (c, animation, true));
                        }
                    }
                }
            }
        }
    }

    pub fn start(input: &[String]) -> i64 {
        let mut day = Day17Render::new(input);
        day.render()
    }
}
