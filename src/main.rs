extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

use tobj; // lib for importing .OBJ 3d objects

mod shader;
mod util;
//mod resources;
mod mesh;
mod scene_graph;
mod toolbox;


use scene_graph::SceneNode;
use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

//#[define M_PI 3.1415926535897932384626433832795]

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //
// The names should be pretty self explanatory
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()

fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

// == // Modify and complete the function below for the first task
// unsafe fn FUNCTION_NAME(ARGUMENT_NAME: &Vec<f32>, ARGUMENT_NAME: &Vec<u32>) -> u32 { }
unsafe fn initiate_vao(vertices: &Vec<f32>, indices: &Vec<u32>, color: &Vec<f32>, normals: &Vec<f32>) -> u32 {

    // Variables used for binding
    let mut vao: u32 = 0; // this is where the Vertex array object (vao) id is stored.
    let mut vbo: u32 = 0; // Vertex buffer object (vbo)
    let vertex_index: u32 = 0;
    let mut index_buffer_id: u32 = 1;

    // Bind initiate_vao
    gl::GenVertexArrays(1, &mut vao); // first argument is number of vao's generating and the second is a pointer to a location where it should be stored
    assert_ne!(vao, 0); // make sure 0 is not returned to vao
    gl::BindVertexArray(vao); // this will link/bind the object to shaders.

    //  --- Setup buffers for vertice coordinates --- //
    gl::GenBuffers(1, &mut vbo); // generating vbo id.
    assert_ne!(vbo, 0); // make sure 0 is not returned
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // binding the vbo to a target (first argument)

    // initializes and creates the buffer object's data store
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW);

    let vertex_components = 3; // As we operate in 3D we need 3 components
    gl::VertexAttribPointer(
        vertex_index,
        vertex_components,
        gl::FLOAT,
        gl::FALSE,
        0,
        ptr::null());

    gl::EnableVertexAttribArray(vertex_index); // This just enables vertex attribute array for the given index

    //  --- Setup buffers for vertex buffer object --- //
    gl::GenBuffers(1, &mut index_buffer_id);
    assert_ne!(index_buffer_id, 0); // make sure 0 is not returned
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_id);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW);


    // ----------- color setup ------------ //

    let mut color_index: u32 = 0;
    let color_buffer_id: u32 = 2;


    gl::GenBuffers(1, &mut color_index);
    assert_ne!(color_index, 0); // make sure 0 is not returned
    gl::BindBuffer(gl::ARRAY_BUFFER, color_index);

    let color_components = 4; // As we operate with alpha we need 4 components
    gl::VertexAttribPointer(
        color_buffer_id,
        color_components,
        gl::FLOAT,
        gl::FALSE,
        0,
        ptr::null());

    gl::EnableVertexAttribArray(color_buffer_id); // This just enables color attribute array for the given index

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(color),
        pointer_to_array(color),
        gl::STATIC_DRAW);

    // ----------- Normal setup ------------ //
    let normal_index: u32 = 3;
    let mut normal_buffer_id: u32 = 0;

    gl::GenBuffers(1, &mut normal_buffer_id);
    assert_ne!(normal_buffer_id, 0); // make sure 0 is not returned
    gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer_id);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(normals),
        pointer_to_array(normals),
        gl::STATIC_DRAW);

    gl::EnableVertexAttribArray(normal_index);

    gl::VertexAttribPointer(
        normal_index,
        3, //xyz
        gl::FLOAT,
        gl::FALSE,
        0,
        ptr::null());

    return vao
}

/* unsafe fn draw_scene(vao: u32, count: usize) {
    gl::FrontFace(gl::CW); //CCW for counter clockwise, CW for Clockwise
    gl::BindVertexArray(vao);
    gl::DrawElements(gl::TRIANGLES, count as i32, gl::UNSIGNED_INT, ptr::null()); // TRIANGLE_STRIP can be used to easier build up geometry
}
 */

unsafe fn draw_scene(node: &scene_graph::SceneNode,
    view_projection_matrix: &glm::Mat4) {
    // Check if node is drawable, set uniforms, draw
    let mut count = 0;
    if node.index_count != -1{
        gl::BindVertexArray(node.vao_id);
        gl::UniformMatrix4fv(5, 1, gl::FALSE, (node.current_transformation_matrix).as_ptr());
        //gl::UniformMatrix4fv(6, 1, gl::FALSE, (view_projection_matrix).as_ptr());
        gl::UniformMatrix4fv(6, 1, gl::FALSE, (view_projection_matrix*node.current_transformation_matrix).as_ptr());
        gl::DrawElements(gl::TRIANGLES, node.index_count as i32, gl::UNSIGNED_INT, ptr::null());


    }
    // Recurse
    for &child in &node.children {
        draw_scene(&*child, view_projection_matrix);
    }
}

unsafe fn door_animation(node: &mut std::mem::ManuallyDrop<std::pin::Pin<std::boxed::Box<scene_graph::SceneNode>>>, door_status: bool, time: f32){
    /* let t = time as f32;
    let step = 0.01f32;
    let closed_door = 0.0;
    let open_door = 0.12;
    if node.position.z == closed_door && door_status{
        while node.position.z < open_door{
            node.position.z += time*step;
        }
    }
    else{
        while node.position.z != closed_door{
            node.position.z -= time*step;
        }
    } */

    if !door_open{
        for x in (0..0.12).step_by(0.01){
            node.position.z += x;
        }

    }
    else{
        for x in (node.position.z..0.0).step_by(-0.01){
            node.position.z -= x;
        }
    }
    
    

}

unsafe fn update_node_transformations(node: &mut scene_graph::SceneNode,
    transformation_so_far: &glm::Mat4) {
    // Construct the correct transformation matrix
    // let mut trans: glm::Mat4 = glm::identity();

    // let mut transform_matrix: glm::Mat4 = glm::translation(& node.position);
    let mut transform_matrix: glm::Mat4 = *transformation_so_far;
    // let mut transform_matrix: glm::Mat4 = glm::identity();
    transform_matrix = glm::translate(&transform_matrix, &node.position);

    transform_matrix = glm::translate(&transform_matrix, &node.reference_point);

    transform_matrix = glm::rotate_y(&transform_matrix, node.rotation.x);
    transform_matrix = glm::rotate_x(&transform_matrix, node.rotation.y);
    transform_matrix = glm::rotate_z(&transform_matrix, node.rotation.z);
    transform_matrix = glm::scale(& transform_matrix, &node.scale);

    transform_matrix = glm::translate(&transform_matrix, &-node.reference_point);


    // Update the node's transformation matrix
    node.current_transformation_matrix = transform_matrix;
    // Recurse
    for &child in &node.children {
        update_node_transformations(&mut *child,
        &node.current_transformation_matrix);
    }
}


fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    let mut door_open: bool = false;
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    #[derive(Clone)]
    struct Camera {
        /// location in x-direction
        x: f32,
        y: f32,
        z: f32,
        movement_speed: f32,
        /// yaw (left-right
        yaw: f32,
        pitch: f32,
        roll: f32,
        look_sensitivity: f32
    }

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            // gl::Enable(gl::CULL_FACE); //need to disable this to make mirroring to work, havent found a work around
            //edit: By using gl::frontface we change the direction it is drawed.
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO here

        let vertices: Vec<f32> = vec![
            0.5, -0.5, -0.9,  // 0
            -0.5, -0.5, -0.9,   // 1
            -0.0, 0.5, -0.9,   // 2

            1.0, -1.0, 0.9,  // 0
            -0.6, 0.2, 0.9,   // 1
            1.0, 1.0, 0.9,   // 2

            -1.0, -1.0, 0.0,  // 0
            -1.0, 1.0, 0.0,   // 1
            0.6, -0.2, 0.0   // 2

        ];

        let indices: Vec<u32> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8
        ];

        let color: Vec<f32> = vec![

            1.0, 0.0, 0.0, 0.33,
            1.0, 0.0, 0.0, 0.33,
            1.0, 0.0, 0.0, 0.33,

            0.0, 1.0, 0.0, 0.33,
            0.0, 1.0, 0.0, 0.33,
            0.0, 1.0, 0.0, 0.33,

            0.0, 0.0, 1.0, 0.33,
            0.0, 0.0, 1.0, 0.33,
            0.0, 0.0, 1.0, 0.33
        ];

        // let input = BufReader::new(File::open("assets/teapot.obj")?);
        // let teapot: Obj = load_obj(input)?;
        //-----------------import obj ass3------------------//

        let terrain = mesh::Terrain::load("resources/lunarsurface.obj");
        // let terrain = mesh::Terrain::load("resources/lunarsurface_hp.obj");

        let helicopter = mesh::Helicopter::load("resources/helicopter.obj");




        //-----------------end import obj ass3------------------//





        // ---------------- import teapot OBJ object -------------- //

        // let mut obj_vertices: Vec<f32> = vec![];
        // let mut obj_indices: Vec<u32> = vec![];

        // let teapot = tobj::load_obj(
        //     "assets/teapot.obj",
        //     &tobj::LoadOptions {
        //         single_index: true,
        //         triangulate: true,
        //         ..Default::default()
        //     },
        // );
        // assert!(teapot.is_ok());
        // let (models, materials) = teapot.expect("Failed to load OBJ file");


        // // Materials might report a separate loading error if the MTL file wasn't found.
        // // If you don't need the materials, you can generate a default here and use that
        // // instead.

        // println!("# of models: {}", models.len());


        // for (i, m) in models.iter().enumerate() {
        //     let mesh = &m.mesh;

        //     println!("model[{}].name = \'{}\'", i, m.name);
        //     println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        //     println!(
        //         "Size of model[{}].face_arities: {}",
        //         i,
        //         mesh.face_arities.len()
        //     );

        //     let mut next_face = 0;
        //     for f in 0..mesh.face_arities.len() {
        //         let end = next_face + mesh.face_arities[f] as usize;
        //         let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
        //         println!("    face[{}] = {:?}", f, face_indices);
        //         next_face = end;
        //     }

        //     // Normals and texture coordinates are also loaded, but not printed in this example
        //     println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

        //     assert!(mesh.positions.len() % 3 == 0);
        //     for v in 0..mesh.positions.len() / 3 {
        //         println!(
        //             "    v[{}] = ({}, {}, {})",
        //             v,
        //             mesh.positions[3 * v],
        //             mesh.positions[3 * v + 1],
        //             mesh.positions[3 * v + 2]
        //         );
        //     }
        //     for v in &mesh.positions {


        //         obj_vertices.push(*v);
        //     }
        //     for i in &mesh.indices {
        //         //println!("{}", *i);
        //         obj_indices.push(*i);
        //     }
        // }

        // ------------------- end OBJ import ------------------- //

        // Initiating the vao to the triangle that are getting drawed.
        //let vao_id = unsafe{ initiate_vao(& vertices, & indices, & color) };
        let vao_terrain_id = unsafe{ initiate_vao(& terrain.vertices, & terrain.indices, & terrain.colors, & terrain.normals) };


        //------------------- vaos for helicopter----------------------// //should make this process easier
        let vao_heli_body = unsafe{ initiate_vao(& helicopter.body.vertices, & helicopter.body.indices, & helicopter.body.colors, & helicopter.body.normals) };
        let vao_heli_door = unsafe{ initiate_vao(& helicopter.door.vertices, & helicopter.door.indices, & helicopter.door.colors, & helicopter.door.normals) };
        let vao_heli_main_rotor = unsafe{ initiate_vao(& helicopter.main_rotor.vertices, & helicopter.main_rotor.indices, & helicopter.main_rotor.colors, & helicopter.main_rotor.normals) };
        let vao_heli_tail_rotor = unsafe{ initiate_vao(& helicopter.tail_rotor.vertices, & helicopter.tail_rotor.indices, & helicopter.tail_rotor.colors, & helicopter.tail_rotor.normals) };




        //------------------- end vaos for helicopter-------------------//



        //------------------- setup scene nodes -------------------//

        //
        let mut scene_root = SceneNode::new();
        let mut terrain_node = SceneNode::from_vao(vao_terrain_id, terrain.index_count);
        let mut heli_root_node = SceneNode::new();
        let mut heli_body_node = SceneNode::from_vao(vao_heli_body, helicopter.body.index_count);
        let mut heli_door_node = SceneNode::from_vao(vao_heli_door, helicopter.door.index_count);
        let mut heli_main_rotor_node = SceneNode::from_vao(vao_heli_main_rotor, helicopter.main_rotor.index_count);
        let mut heli_tail_rotor_node = SceneNode::from_vao(vao_heli_tail_rotor, helicopter.tail_rotor.index_count);


        heli_root_node.add_child(&heli_body_node);
        heli_root_node.add_child(&heli_door_node);
        heli_root_node.add_child(&heli_main_rotor_node);
        heli_root_node.add_child(&heli_tail_rotor_node);
        terrain_node.add_child(&heli_root_node);
        scene_root.add_child(&terrain_node);


        // set correct ref point
        heli_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);

        heli_root_node.print();





        //------------------- end scene setup -------------------//









        //let vao_id = unsafe{ initiate_vao(&obj_vertices, &obj_indices, & color) };
        // Basic usage of shader helper:
        // The example code below returns a shader object, which contains the field `.program_id`.
        // The snippet is not enough to do the assignment, and will need to be modified (outside of
        // just using the correct path), but it only needs to be called once
        //
        //     shader::ShaderBuilder::new()
        //        .attach_file("./path/to/shader.file")
        //        .link();
        //this returns the unsafe function to the shader variable
        let shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
                .activate() //assignment says activate it, but doesnt seemed to be needed. this only runs the useProgram function
        };


        // This needs to be dissable when using a custom frag shader*
        // unsafe {
        //     gl::UseProgram(0);
        // }

        let camera_properties_default = Camera {
            x: 1.0,
            y: 1.0,
            // Start with -1 since view-box is from -1 to 1, this way we see the scene at the beginning
            z: -1.0,
            movement_speed: 20.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            look_sensitivity: 0.008
        };

        let mut camera_properties = Camera {
            x: 1.0,
            y: 1.0,
            // Start with -1 since view-box is from -1 to 1, this way we see the scene at the beginning
            z: -1.0,
            movement_speed: 20.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            look_sensitivity: 0.008
        };

        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;
        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            unsafe{
                gl::Uniform1f(4, elapsed);
            }

            // Handle keyboard input

            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // sin and cos is used to take current yaw into account for movement
                        VirtualKeyCode::W => {
                            // camera_properties.z += delta_time * camera_properties.yaw.cos() * camera_properties.movement_speed;
                            camera_properties.z += delta_time * camera_properties.yaw.cos() * camera_properties.movement_speed;
                            camera_properties.x -= delta_time * camera_properties.yaw.sin() * camera_properties.movement_speed;
                            camera_properties.y += delta_time * camera_properties.pitch.sin() * camera_properties.movement_speed;
                        },
                        VirtualKeyCode::S => {
                            camera_properties.z -= delta_time * camera_properties.yaw.cos() * camera_properties.movement_speed;
                            camera_properties.x += delta_time * camera_properties.yaw.sin() * camera_properties.movement_speed;
                            camera_properties.y -= delta_time * camera_properties.pitch.sin() * camera_properties.movement_speed;
                        },
                        VirtualKeyCode::A => {
                            camera_properties.x += delta_time * camera_properties.yaw.cos() * camera_properties.movement_speed;
                            camera_properties.z += delta_time * camera_properties.yaw.sin() * camera_properties.movement_speed;
                        },
                        VirtualKeyCode::D => {
                            camera_properties.x -= delta_time * camera_properties.yaw.cos() * camera_properties.movement_speed;
                            camera_properties.z -= delta_time * camera_properties.yaw.sin() * camera_properties.movement_speed;
                        },
                        VirtualKeyCode::Q => {
                            camera_properties.y += delta_time * camera_properties.movement_speed;
                        },
                        VirtualKeyCode::E => {
                            camera_properties.y -= delta_time * camera_properties.movement_speed;
                        },
                        VirtualKeyCode::F => {
                            camera_properties.roll += delta_time * 100.0*camera_properties.look_sensitivity;
                        },
                        VirtualKeyCode::C => {
                            camera_properties.roll -= delta_time * 100.0*camera_properties.look_sensitivity;
                        },
                        VirtualKeyCode::R => {
                            camera_properties = camera_properties_default.clone();
                        },
                        VirtualKeyCode::T => {
                            //door_animation(&mut heli_door_node, elapsed);
                            door_open = !door_open;
                            
                        },

                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                let mut look_up_down = glm::vec2(camera_properties.yaw.cos(), camera_properties.yaw.sin());
                look_up_down = glm::normalize(&look_up_down);
                camera_properties.pitch += delta.1 * camera_properties.look_sensitivity * look_up_down[0];
                camera_properties.roll += delta.1 * camera_properties.look_sensitivity * look_up_down[1];
                // limit pich to viewing somewher between top and bottom
                camera_properties.pitch = clamp(camera_properties.pitch, -90.0f32.to_radians(), 90.0f32.to_radians());



                camera_properties.yaw += delta.0 * camera_properties.look_sensitivity;

                // println!("mouse x: {}",  delta.0);
                // println!("mouse y: {}",  delta.1);
                // println!("yaw: {}",  camera_properties.yaw);

                *delta = (0.0, 0.0);
            }

            unsafe {
                //gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


                // Issue the necessary commands to draw your scene here


                // let scale_vector: glm::Vec3 = glm::vec3(1.0, 1.0, 1.0);

                // let angle: f32 = 360.0f32.to_radians();

                //--------------- calculate camera transform matrix  -------------------//
                let direction_vector: glm::Vec3 = glm::vec3(0.0, 0.0, -6.0);
                //for ass2
                /* let camera_perspective: glm::Mat4 =
                glm::perspective(
                    SCREEN_W as f32 / SCREEN_H as f32,
                    90.0f32.to_radians(),
                    1.0,
                    100.0
                ); */
                //need to increase the farplane for assignment 3
                let camera_perspective: glm::Mat4 =
                glm::perspective(
                    SCREEN_W as f32 / SCREEN_H as f32,
                    90.0f32.to_radians(),
                    1.0,
                    1000.0
                );

                let mut transform_matrix: glm::Mat4 = glm::translation(&direction_vector);

                // update camera positioning and orientation
                let move_vector = glm::vec3(camera_properties.x, camera_properties.y, camera_properties.z);
                // move_vector = glm::normalize(&move_vector);
                transform_matrix = glm::translate(&transform_matrix, &move_vector);
                transform_matrix = glm::rotate_y(&transform_matrix, camera_properties.yaw);
                transform_matrix = glm::rotate_x(&transform_matrix, camera_properties.pitch);
                transform_matrix = glm::rotate_z(&transform_matrix, camera_properties.roll);
                transform_matrix = camera_perspective*transform_matrix;

                //gl::UniformMatrix4fv(5, 1, gl::FALSE, transform_matrix.as_ptr());


                //--------------- making the helicopter rotors spin -------------------//
                heli_main_rotor_node.rotation = glm::vec3(100.0*elapsed, 0.0, 0.0);
                heli_tail_rotor_node.rotation = glm::vec3(0.0, 50.0*elapsed, 0.0);

                //--------------- making the helicopter go in a path-------------------//
                let heading = toolbox::simple_heading_animation(elapsed);

                door_animation(&mut heli_door_node, door_open, delta_time);

                /* heli_root_node.position.x = heading.x;
                heli_root_node.position.y = 10.0 + 0.4*elapsed.sin();
                heli_root_node.position.z = heading.z;
                heli_root_node.rotation.y = heading.pitch;
                heli_root_node.rotation.x = heading.yaw;
                heli_root_node.rotation.z = heading.roll; */

                //--------------- end making the helicopter go in a path-------------------//


                //-----------------rotate to check lighting conditions (Task5a) -----------------//

                // heli_root_node.rotation.x = 3.1415926535897932384626433832795*-0.5; //+- 0.5 to rotate the helicopter


                //-----------------^rotate to check lighting conditions ^-----------------//


                update_node_transformations(&mut terrain_node, &scene_root.current_transformation_matrix);
                draw_scene(&scene_root, &transform_matrix);

                //draw_scene(indices.len()); //drawing the triangles now, this will dr aw all objects later
                /* draw_scene(vao_terrain_id, terrain.indices.len());
                draw_scene(vao_heli_body, helicopter.body.indices.len());
                draw_scene(vao_heli_door, helicopter.door.indices.len());
                draw_scene(vao_heli_main_rotor, helicopter.main_rotor.indices.len());
                draw_scene(vao_heli_tail_rotor, helicopter.tail_rotor.indices.len());
 */
                //do i have to bind something?
                /*


                 */
                //draw the elements mode: triangle, number of points/count: lenght of the indices, type and void* indices

            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    },
                    // Q => {
                    //     *control_flow = ControlFlow::Exit;
                    // }
                    _ => { }
                }
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            },
            _ => { }
        }
    });
}
