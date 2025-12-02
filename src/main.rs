use cyclone_fireworks::fireworks::{
    build_default_rules, FireworkWorld,
};
use cyclone_fireworks::precision::Real;
use cyclone_fireworks::random::Random;

use std::ffi::CString;
use std::num::NonZeroU32;
use std::time::Instant;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay};
use glutin::prelude::*;
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};

use glutin_winit::DisplayBuilder;

use raw_window_handle::{HasWindowHandle};

use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent, KeyEvent},
    event_loop::{EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

use gl;



// ---------- Small GL helpers ----------

fn compile_shader(src: &str, shader_type: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(src).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut status = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut i8,
            );
            eprintln!("Shader compile error: {}", String::from_utf8_lossy(&buf));
            panic!("Shader compilation failed");
        }
        shader
    }
}
fn link_program(vs: u32, fs: u32) -> u32 {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut status = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut i8,
            );
            eprintln!("Program link error: {}", String::from_utf8_lossy(&buf));
            panic!("Program link failed");
        }

        gl::DetachShader(program, vs);
        gl::DetachShader(program, fs);
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        program
    }
}
fn init_gl_resources() -> (u32, u32, u32) {
    // Simple point shader: takes position + color, draws GL_POINTS
    const VERT_SRC: &str = r#"
        #version 330 core
        layout (location = 0) in vec2 a_pos;
        layout (location = 1) in vec3 a_color;
        out vec3 v_color;
        void main() {
            gl_Position = vec4(a_pos, 0.0, 1.0);
            v_color = a_color;
            gl_PointSize = 4.0;
        }
    "#;

    const FRAG_SRC: &str = r#"
        #version 330 core
        in vec3 v_color;
        out vec4 FragColor;
        void main() {
            FragColor = vec4(v_color, 1.0);
        }
    "#;

    // These are safe wrappers we already made, with internal unsafe blocks.
    let vs = compile_shader(VERT_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FRAG_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    // VAO + VBO
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // layout: x, y, r, g, b -> 5 floats
        let stride = (5 * std::mem::size_of::<f32>()) as i32;

        // position: location = 0, 2 floats
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            std::ptr::null(),
        );

        // color: location = 1, 3 floats, offset = 2 * sizeof(f32)
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (2 * std::mem::size_of::<f32>()) as *const _,
        );

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // make sure point size from shader works
        gl::Enable(gl::PROGRAM_POINT_SIZE);
    }

    (program, vao, vbo)
}

fn render_world(world: &FireworkWorld, program: u32, vao: u32, vbo: u32) {
    // Build vertex buffer on CPU first (safe)
    let mut vertices: Vec<f32> = Vec::new();

    for fw in &world.fireworks {
        if fw.type_id == 0 {
            continue;
        }

        let pos = fw.particle.get_position();
        if pos.y < 0.0 {
            continue;
        }

        // World → NDC like glOrtho(-50, 50, 0, 60, -1, 1)
        //let ndc_x = (pos.x / 50.0).clamp(-1.0, 1.0);
        //let ndc_y = (pos.y / 60.0) * 2.0 - 1.0;
        let ndc_x = (pos.x / 12.5).clamp(-1.0, 1.0);
        let ndc_y = (pos.y / 15.0) * 2.0 - 1.0;

        // Color by type
        let (r, g, b) = match fw.type_id {
            1 => (1.0, 0.0, 0.0),
            2 => (1.0, 0.5, 0.0),
            3 => (1.0, 1.0, 0.0),
            4 => (0.0, 1.0, 0.0),
            5 => (0.0, 1.0, 1.0),
            6 => (0.4, 0.4, 1.0),
            7 => (1.0, 0.0, 1.0),
            8 => (1.0, 1.0, 1.0),
            9 => (1.0, 0.5, 0.5),
            _ => (1.0, 1.0, 1.0),
        };

        vertices.extend_from_slice(&[
            ndc_x as f32,
            ndc_y as f32,
            r,
            g,
            b,
        ]);

        // If you later want reflections, add them here too.
    }

    let count = (vertices.len() / 5) as i32;
    if count == 0 {
        return;
    }

    unsafe {
        gl::UseProgram(program);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STREAM_DRAW,
        );

        gl::DrawArrays(gl::POINTS, 0, count);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }
}
/*
fn spawn_all_rules(world: &mut FireworkWorld, rng: &mut Random) {
    let rule_count = world.rules.len() as u32;
    for type_id in 1..=rule_count {
        world.spawn_single(type_id, None, rng);
    }
} */

// ---------- Main ----------
fn main() {
    // ----- Physics: FireworkWorld + RNG -----

    let mut rng = Random::with_seed(12345);
    let rules = build_default_rules();
    let mut world = FireworkWorld::new(rules);
    world.spawn_single(1, None, &mut rng); // start with a single rocket
   /* let mut rng = Random::with_seed(12345);
    let rules = build_default_rules();
    let mut world = FireworkWorld::new(rules);

    // Spawn one of each rule at startup
    spawn_all_rules(&mut world, &mut rng); */
    let mut next_launch_time = Instant::now();
    let launch_interval = std::time::Duration::from_millis(500); // every 0.5 sec


    // ----- Window / GL init -----
    let event_loop = EventLoop::new().expect("Failed to create winit event loop");

    // Build window attributes (title + size)
    let window_attributes: WindowAttributes = Window::default_attributes()
        .with_title("Rust + Glutin 0.32 Fireworks")
        .with_inner_size(LogicalSize::new(800.0, 600.0));


    // Tell DisplayBuilder we want a window with those attributes
    let display_builder = DisplayBuilder::new()
        .with_window_attributes(Some(window_attributes));

    // Basic config template (no special requirements)
    let template = ConfigTemplateBuilder::new();

    // Create window + GL config
    let (window_opt, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            // choose a config (here: highest sample count, like before)
            configs
                .reduce(|best, cfg| if cfg.num_samples() > best.num_samples() { cfg } else { best })
                .unwrap()
        })
        .expect("Failed to build window + GL config");

    let window = window_opt.expect("No window was created");

    // ----- Create GL context + surface -----
    let window_handle = window.window_handle().unwrap().as_raw();

    // Context attributes (use default OpenGL; you can request a version here if you want)
    let context_attributes = ContextAttributesBuilder::new()
        .build(Some(window_handle));

    // Create an OpenGL context that is not yet current
    let not_current_gl_context = unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &context_attributes)
            .expect("Failed to create GL context")
    };

    // Create a window surface
    let size = window.inner_size();
    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window_handle,
        NonZeroU32::new(size.width.max(1)).unwrap(),
        NonZeroU32::new(size.height.max(1)).unwrap(),
    );

    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .expect("Failed to create window surface")
    }; 

    // Make the context current
    let gl_context: PossiblyCurrentContext = not_current_gl_context
        .make_current(&gl_surface)
        .expect("Failed to make GL context current");

    // ----- Load OpenGL function pointers -----
    gl::load_with(|symbol| {
        // Convert the Rust &str into a C-compatible string
        let cstr = std::ffi::CString::new(symbol).unwrap();
        gl_config.display().get_proc_address(&cstr) as *const _
    });

    unsafe {
        gl::Viewport(0, 0, size.width as i32, size.height as i32);
        gl::ClearColor(0.0, 0.0, 0.1, 1.0);
    }

    // Our VAO/VBO + shaders
    let (program, vao, vbo) = init_gl_resources();

    let mut last_frame = Instant::now();

    // ----- Event loop (replaces GLUT main loop + callbacks) -----
    #[allow(deprecated)]
    let _ = event_loop.run(move |event, elwt| {
        match event {
            // All window-related events for *this* window
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }

                    WindowEvent::Resized(new_size) => {
                        gl_surface.resize(
                            &gl_context,
                            NonZeroU32::new(new_size.width.max(1)).unwrap(),
                            NonZeroU32::new(new_size.height.max(1)).unwrap(),
                        );
                        unsafe {
                            gl::Viewport(
                                0,
                                0,
                                new_size.width as i32,
                                new_size.height as i32,
                            );
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        let now = Instant::now();
                        let dt_secs = (now - last_frame).as_secs_f32().min(0.05);
                        last_frame = now;

                        let dt: Real = dt_secs as Real;
                        // automatic periodic launch
                        if now >= next_launch_time {
                            world.spawn_single(1, None, &mut rng);
                            next_launch_time = now + launch_interval;
                        }

                        world.update(dt, &mut rng);

                        unsafe {
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                            render_world(&world, program, vao, vbo);
                        }
                        gl_surface.swap_buffers(&gl_context).unwrap();
                    }

                    WindowEvent::KeyboardInput {
                        event:
                        KeyEvent {
                            physical_key,
                            state,
                            ..
                        },
                        ..
                    } => {
                        if state == ElementState::Pressed {
                            match physical_key {
                                PhysicalKey::Code(KeyCode::Escape) => elwt.exit(),
                                PhysicalKey::Code(KeyCode::Digit1) => world.spawn_single(1, None, &mut rng),
                                PhysicalKey::Code(KeyCode::Digit2) => world.spawn_single(2, None, &mut rng),
                                PhysicalKey::Code(KeyCode::Digit3) => world.spawn_single(3, None, &mut rng),
                                PhysicalKey::Code(KeyCode::Digit4) => world.spawn_single(4, None, &mut rng),
                                PhysicalKey::Code(KeyCode::Digit5) => world.spawn_single(5, None, &mut rng),
                                PhysicalKey::Code(KeyCode::Digit6) => world.spawn_single(6, None, &mut rng),
                                PhysicalKey::Code(KeyCode::Digit7) => world.spawn_single(7, None, &mut rng),
                                PhysicalKey::Code(KeyCode::Digit8) => world.spawn_single(8, None, &mut rng),
                                PhysicalKey::Code(KeyCode::Digit9) => world.spawn_single(9, None, &mut rng),
                                _ => {}
                            }
                        }
                    }

                    _ => {}
                }
            }

            // Like GLUT idle: ask the OS for a redraw every loop
            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => {}
        }
    });
}



    /* Testing for Fireworld and rules for Fireworks --------------------------------------
    let mut rng = Random::with_seed(12345);
    let rules = build_default_rules();
    let mut world = FireworkWorld::new(rules);

    // Spawn a single type-1 rocket (like pressing '1' in Millington's demo)
    world.spawn_single(1, None, &mut rng);

    let dt: Real = 0.1;

    println!("Simulating fireworks world:");
    for step in 0..50 {
        println!("--- step {} ---", step);

        let mut active = 0;
        for (i, fw) in world.fireworks.iter().enumerate() {
            if fw.type_id > 0 {
                active += 1;
                let pos = fw.particle.get_position();
                println!(
                    "slot {:3} | type {:2} | age={:5.2} | pos=({:6.2}, {:6.2}, {:6.2})",
                    i,
                    fw.type_id,
                    fw.age,
                    pos.x, pos.y, pos.z
                );
            }
        }

        println!("active fireworks: {}", active);

        world.update(dt, &mut rng);
    }
    */

    /* Testing for a Single Firework ----------------------------------------------------------
    let mut rng = Random::with_seed(12345);

    //A simple rule, loosely based on rules[0] in the Millington code.as
    let rule = FireworkRule::new(
        1,                              //type_id
        0.5, 1.4,                       //min_age, max_age
        Vector3::new(-5.0, 25.0, -5.0), //min_velocity
        Vector3::new(5.0, 28.0, 5.0),   //max_velocity
        0.1,                            //damping
        vec![Payload::new(3, 5)],        //one payload, type 3, count 5 (not used yet)
    );

    // Tracking a small number of fireworks
    const MAX_FIREWORKS: usize = 5;
    let mut fireworks: [Firework; MAX_FIREWORKS] = [Firework::default(); MAX_FIREWORKS];

    // Create one root firework in slot 0.
    rule.create(&mut fireworks[0], None, &mut rng);

    let dt: Real = 0.1;

    println!("Simulating a single firework:");
    for step in 0..20 {
        let fw = &mut fireworks[0];
        let dead = fw.update(dt);
        let pos = fw.particle.get_position();
        println!(
            "step {:2}: age={:5.2}, pos=({:6.2}, {:6.2}, {:6.2}), dead = {}",
            step, fw.age, pos.x, pos.y, pos.z, dead
        );
        if dead {
            println!("Firework died at step {step}");
            break;
        }
    }
*/

    /* Testing Random -----------------------------------------------------------------------

        // Create a reproducible random stream
        let mut rng = Random::with_seed(12345);

        println!("=== Random binomial numbers ===");
        for i in 0..5 {
            let val: Real = rng.random_binomial(1.0);
            println!("binomial[{i}] = {val}");
        }

        println!("\n=== Random binomial vectors (scale = 1.0) ===");
        for i in 0..5 {
            let v: Vector3 = rng.random_vector_scale(1.0);
            println!("vector[{i}] = ({:.3}, {:.3}, {:.3})", v.x, v.y, v.z);
        }

        println!("\n=== Random XZ vectors (scale = 2.0) ===");
        for i in 0..5 {
            let v: Vector3 = rng.random_xz_vector(2.0);
            println!("xz_vector[{i}] = ({:.3}, {:.3}, {:.3})", v.x, v.y, v.z);
        } */

