use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext, WebGlProgram, WebGlShader};
use avila_vec3d::Mat4;
use crate::camera::VRCamera;
use crate::scene::{Scene, RenderMesh};

pub struct Renderer {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    canvas: HtmlCanvasElement,
}

impl Renderer {
    pub fn new(canvas: &HtmlCanvasElement) -> std::result::Result<Self, avila_bim_core::BimError> {
        let gl = canvas
            .get_context("webgl")
            .map_err(|_| avila_bim_core::BimError::InvalidGeometry("Failed to get WebGL context".into()))?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| avila_bim_core::BimError::InvalidGeometry("Failed to cast WebGL context".into()))?;

        // Enable depth testing
        gl.enable(WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(WebGlRenderingContext::LEQUAL);

        // Clear color
        gl.clear_color(0.1, 0.1, 0.2, 1.0);

        // Create shaders
        let vertex_shader = Self::compile_shader(
            &gl,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec3 a_position;
            uniform mat4 u_model_view_projection;

            void main() {
                gl_Position = u_model_view_projection * vec4(a_position, 1.0);
            }
            "#,
        )?;

        let fragment_shader = Self::compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            precision mediump float;
            uniform vec4 u_color;

            void main() {
                gl_FragColor = u_color;
            }
            "#,
        )?;

        let program = Self::link_program(&gl, &vertex_shader, &fragment_shader)?;

        Ok(Self {
            gl,
            program,
            canvas: canvas.clone(),
        })
    }

    fn compile_shader(gl: &WebGlRenderingContext, shader_type: u32, source: &str) -> std::result::Result<WebGlShader, avila_bim_core::BimError> {
        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
            Ok(shader)
        } else {
            Err(avila_bim_core::BimError::InvalidGeometry(gl.get_shader_info_log(&shader).unwrap_or_else(|| "Unknown shader compile error".into())))
        }
    }

    fn link_program(gl: &WebGlRenderingContext, vertex_shader: &WebGlShader, fragment_shader: &WebGlShader) -> std::result::Result<WebGlProgram, avila_bim_core::BimError> {
        let program = gl.create_program().unwrap();
        gl.attach_shader(&program, vertex_shader);
        gl.attach_shader(&program, fragment_shader);
        gl.link_program(&program);

        if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            Ok(program)
        } else {
            Err(avila_bim_core::BimError::InvalidGeometry(gl.get_program_info_log(&program).unwrap_or_else(|| "Unknown program link error".into())))
        }
    }

    pub fn render(&self, scene: &Scene, camera: &VRCamera) {
        let gl = &self.gl;

        // Clear
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        // Use program
        gl.use_program(Some(&self.program));

        // Set viewport
        gl.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);

        // Get uniform locations
        let mvp_location = gl.get_uniform_location(&self.program, "u_model_view_projection");
        let color_location = gl.get_uniform_location(&self.program, "u_color");
        let position_location = gl.get_attrib_location(&self.program, "a_position") as u32;

        // Camera matrices
        let view_proj = camera.get_view_projection_matrix();
        let mvp_matrix: [f32; 16] = unsafe { std::mem::transmute(view_proj) };

        // Render each mesh
        for mesh in &scene.meshes {
            // Set MVP matrix
            gl.uniform_matrix4fv_with_f32_array(mvp_location.as_ref(), false, &mvp_matrix);

            // Set color
            gl.uniform4f(color_location.as_ref(), mesh.color[0], mesh.color[1], mesh.color[2], mesh.color[3]);

            // Create buffers
            let vertex_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &js_sys::Float32Array::from(&mesh.vertices[..]),
                WebGlRenderingContext::STATIC_DRAW,
            );

            let index_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &js_sys::Uint32Array::from(&mesh.indices[..]),
                WebGlRenderingContext::STATIC_DRAW,
            );

            // Enable attribute
            gl.enable_vertex_attrib_array(position_location);
            gl.vertex_attrib_pointer_with_i32(position_location, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

            // Draw
            gl.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                mesh.indices.len() as i32,
                WebGlRenderingContext::UNSIGNED_INT,
                0,
            );

            // Cleanup
            gl.delete_buffer(Some(&vertex_buffer));
            gl.delete_buffer(Some(&index_buffer));
        }
    }
}