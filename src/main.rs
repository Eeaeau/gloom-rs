extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

// extra stuff
// extern crate obj;
// use std::fs::File;
// use std::io::BufReader;
// use obj::{load_obj, Obj};


// use obj::{load_obj, Obj};


mod shader;
mod util;


use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

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

struct CameraProperties {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,

}




// == // Modify and complete the function below for the first task
// unsafe fn FUNCTION_NAME(ARGUMENT_NAME: &Vec<f32>, ARGUMENT_NAME: &Vec<u32>) -> u32 { }




unsafe fn initiate_vao(vertices: &Vec<f32>, indices: &Vec<u32>, color: &Vec<f32>) -> u32 {

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

    return vao
}

unsafe fn draw_scene(count: usize) {
    gl::FrontFace(gl::CW); //CCW for counter clockwise, CW for Clockwise
    gl::DrawElements(gl::TRIANGLES, count as i32, gl::UNSIGNED_INT, ptr::null()); // TRIANGLE_STRIP can be used to easier build up geometry
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
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

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

        // --- TASK 1 --- //

        //triangles data

        //1 triangle
        /* let vertices: Vec<f32> = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];
        let indices: Vec<u32> = vec![0, 1, 2]; */

        //5 triangles, in this case the z axis will always be 0
        /* let vertices: Vec<f32> = vec![
            -1.0, -1.0, 0.0,  // 0
            -1.0, 1.0, 0.0,   // 1
            -0.5, 0.0, 0.0,   // 2
            0.0, 1.0, 0.0,    // 3
            1.0, 1.0, 0.0,    // 4
            0.0, -1.0, 0.0,   // 5
            0.0, 0.5, 0.0,    // 6
            0.0, -0.5, 0.0,   // 7
            0.5, 0.0, 0.0,     // 8
            1.0, -1.0, 0.0    // 9
        ]; */
        //remember to draw the correct order. right corner->top-> left corner
        /* let indices: Vec<u32> = vec![
            5, 2, 0,
            1, 2, 3,
            9, 8, 5,
            3, 8, 4,
            7, 8, 6,
            6, 2, 7
        ]; */


        // --- TASK 2 --- //

        // task 2a
        // == // Set up your vao here
        // unsafe {
        //     let draw_vao: u32 = 0;
        //     gl::BindVertexArray(draw_vao);
        //     let vao = initiate_vao(& v1, & indices);
        // }

        /* let vertices: Vec<f32> = vec![
            0.6, -0.8, 1.0,  // 0
            0.0, 0.4, 0.0,   // 1
            -0.8, -0.2, 1.0   // 2
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2
        ]; */

        // task 2b

        /* let indices: Vec<u32> = vec![
            2, 1, 0
        ];*/

        // task 2d
        // changing the simple.vert files positions

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

        // let test: glm::Mat4 = glm::mat4x4(
        //     1.0, 0.0, 0.0, 0.0,
        //     0.0, 1.0, 0.0, 0.0,
        //     0.0, 0.0, 1.0, 0.0,
        //     0.0, 0.0, 0.0, 1.0);

        // let input = BufReader::new(File::open("assets/teapot.obj")?);
        // let teapot: Obj = load_obj(input)?;

        // let teapot = tobj::load_obj("assets/teapot.obj", tobj::LoadOptions::default()); 
    

        // Initiating the vao to the triangle that are getting drawed.
        let vao_id = unsafe{ initiate_vao(& vertices, & indices, & color) };

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

        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;

        let mut camera_motion = CameraProperties {
            x: 1.0,
            y: 1.0,
            z: -1.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        };

        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            unsafe{
                gl::Uniform1f(4, elapsed);
            }

            let speed_x = 0.5;
            let speed_y = 0.5;
            let speed_z = 0.5;
            let speed_yaw = 0.5;
            let speed_pitch = 0.5;
            let speed_roll = 0.5;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::W => {
                            camera_motion.z += delta_time*speed_z;
                        },
                        VirtualKeyCode::A => {
                            camera_motion.x += delta_time*speed_x;
                        },
                        VirtualKeyCode::S => {
                            camera_motion.z -= delta_time*speed_z;
                        },
                        VirtualKeyCode::D => {
                            camera_motion.x -= delta_time*speed_x;
                        },
                        VirtualKeyCode::Q => {
                            camera_motion.y += delta_time*speed_z;
                        },
                        VirtualKeyCode::E => {
                            camera_motion.y -= delta_time*speed_y;
                        },
                        VirtualKeyCode::Left => {
                            camera_motion.yaw += delta_time*speed_yaw;
                        },
                        VirtualKeyCode::Right => {
                            camera_motion.yaw -= delta_time*speed_yaw;
                        },
                        VirtualKeyCode::Up => {
                            camera_motion.pitch -= delta_time*speed_pitch;
                        },
                        VirtualKeyCode::Down => {
                            camera_motion.pitch += delta_time*speed_pitch;
                        },
                        VirtualKeyCode::T => {
                            camera_motion.roll += delta_time*speed_roll;
                        },
                        VirtualKeyCode::F => {
                            camera_motion.roll -= delta_time*speed_roll;
                        },
                        
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {



                *delta = (0.0, 0.0);
            }

            unsafe {
                gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 

                // Issue the necessary commands to draw your scene here 

                let scale_vector: glm::Vec3 = glm::vec3(1.0, 1.0, 1.0);
                let mut camera_rotation_vector: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
                let direction_vector: glm::Vec3 = glm::vec3(0.0, 0.0, -6.0+5.0*elapsed.sin());

                
                // let angle: f32 = 360.0f32.to_radians();
                
                //let mut identity: glm::Mat4 = glm::identity();

                //er dette riktig

                let translate_z: glm::Mat4 = glm::mat4(
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 49.5, -50.5,
                    0.0, 0.0, 0.0, 1.0, 
                );

                
                let cam: glm::Mat4 =
                glm::perspective(
                    6.0/8.0,
                    90.0,
                    1.0,
                    100.0
                );

                let mut motion_matrix: glm::Mat4 = cam * translate_z;

                
                //let transform_matrix: glm::Mat4 = cam * glm::translation(&direction_vector)*glm::rotation(10.0*elapsed, &glm::vec3(1.0, 0.0, 0.0)) * glm::scaling(&scale_vector);

                motion_matrix = glm::translate(&motion_matrix, &glm::vec3(camera_motion.x, 0.0, 0.0));
                motion_matrix = glm::translate(&motion_matrix, &glm::vec3(0.0, camera_motion.y, 0.0));
                motion_matrix = glm::translate(&motion_matrix, &glm::vec3(0.0, 0.0, camera_motion.z));
                motion_matrix = glm::rotate_y(&motion_matrix, camera_motion.yaw);
                motion_matrix = glm::rotate_x(&motion_matrix, camera_motion.pitch);
                motion_matrix = glm::rotate_z(&motion_matrix, camera_motion.roll);

                gl::UniformMatrix4fv(5, 1, gl::FALSE, motion_matrix.as_ptr());
                                
                                
                                
                draw_scene(vertices.len()); //drawing the triangles now, this will draw all objects later
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
