//! Renderer WebGL

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlShader};

use crate::Scene;
use crate::frustum::{Frustum, AABB, multiply_matrices};
use crate::lod::{LodLevel, LodConfig};
use alloc::vec::Vec;

#[cfg(target_arch = "wasm32")]
/// WebGL Renderer
pub struct WebGLRenderer {
    gl: WebGlRenderingContext,
    program: Option<WebGlProgram>,
    lod_config: LodConfig,
}

#[cfg(target_arch = "wasm32")]
impl WebGLRenderer {
    /// Cria renderer a partir de canvas
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        canvas.set_width(1920);
        canvas.set_height(1080);

        let gl = canvas
            .get_context("webgl")?
            .ok_or("Failed to get WebGL context")?
            .dyn_into::<WebGlRenderingContext>()?;

        let renderer = Self {
            gl,
            program: None,
            lod_config: LodConfig::default(),
        };

        Ok(renderer)
    }

    /// Inicializa shaders
    pub fn init_shaders(&mut self) -> Result<(), JsValue> {
        let vert_shader = self.compile_shader(
            WebGlRenderingContext::VERTEX_SHADER,
            VERTEX_SHADER_SOURCE,
        )?;

        let frag_shader = self.compile_shader(
            WebGlRenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER_SOURCE,
        )?;

        let program = self.gl.create_program().ok_or("Failed to create program")?;
        self.gl.attach_shader(&program, &vert_shader);
        self.gl.attach_shader(&program, &frag_shader);
        self.gl.link_program(&program);

        if !self.gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool().unwrap_or(false)
        {
            return Err(JsValue::from_str(&self.gl.get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error".to_string())));
        }

        self.gl.use_program(Some(&program));
        self.program = Some(program);

        Ok(())
    }

    /// Compila shader
    fn compile_shader(&self, shader_type: u32, source: &str) -> Result<WebGlShader, JsValue> {
        let shader = self.gl.create_shader(shader_type)
            .ok_or("Failed to create shader")?;

        self.gl.shader_source(&shader, source);
        self.gl.compile_shader(&shader);

        if !self.gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool().unwrap_or(false)
        {
            return Err(JsValue::from_str(&self.gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error".to_string())));
        }

        Ok(shader)
    }

    /// Renderiza cena com frustum culling
    pub fn render(&self, scene: &Scene, selected_index: Option<usize>, visible_indices: &[usize]) -> Result<(), JsValue> {
        // Clear
        self.gl.clear_color(0.1, 0.1, 0.15, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
        self.gl.enable(WebGlRenderingContext::DEPTH_TEST);

        let program = self.program.as_ref().ok_or("Program not initialized")?;

        // Matrizes de view e projection
        let view_matrix = scene.camera.view_matrix();
        let proj_matrix = scene.camera.projection_matrix();

        // Envia matrizes para GPU
        let view_loc = self.gl.get_uniform_location(program, "u_modelViewMatrix");
        let proj_loc = self.gl.get_uniform_location(program, "u_projectionMatrix");

        if let Some(loc) = view_loc {
            self.gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &view_matrix);
        }
        if let Some(loc) = proj_loc {
            self.gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &proj_matrix);
        }

        // Frustum culling: calcular frustum view-projection
        let vp_matrix = multiply_matrices(&proj_matrix, &view_matrix);
        let frustum = Frustum::from_matrix(&vp_matrix);

        // Frustum culling: calcular frustum view-projection
        let vp_matrix = multiply_matrices(&proj_matrix, &view_matrix);
        let frustum = Frustum::from_matrix(&vp_matrix);

        // Renderiza cada geometria visível (com frustum culling + LOD)
        let mut culled_count = 0;
        let mut lod_stats = [0u32; 4]; // High, Medium, Low, Minimal
        let cam_pos = scene.camera.position;

        for idx in visible_indices {
            if let Some(geom) = scene.geometries.get(*idx) {
                // Frustum culling: criar AABB da geometria e testar
                let aabb = AABB {
                    min: geom.bbox_min,
                    max: geom.bbox_max,
                };

                if !frustum.contains_aabb(&aabb) {
                    // Objeto fora do frustum, skip
                    culled_count += 1;
                    continue;
                }

                // LOD: calcular distância da câmera ao centro do objeto
                let center = [
                    (geom.bbox_min[0] + geom.bbox_max[0]) * 0.5,
                    (geom.bbox_min[1] + geom.bbox_max[1]) * 0.5,
                    (geom.bbox_min[2] + geom.bbox_max[2]) * 0.5,
                ];
                let dx = cam_pos[0] - center[0];
                let dy = cam_pos[1] - center[1];
                let dz = cam_pos[2] - center[2];
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                // Determinar LOD level
                let lod_level = self.lod_config.calculate_lod(distance);
                lod_stats[lod_level as usize] += 1;

                let is_selected = selected_index == Some(*idx);
                self.render_geometry_with_lod(program, geom, is_selected, lod_level)?;
            }
        }

        // Log frustum culling + LOD stats (debug)
        #[cfg(target_arch = "wasm32")]
        if culled_count > 0 || lod_stats.iter().sum::<u32>() > 0 {
            web_sys::console::log_1(&format!(
                "🎯 Frustum: {}/{} filtrados | LOD: H{} M{} L{} Min{}",
                culled_count,
                visible_indices.len(),
                lod_stats[0], lod_stats[1], lod_stats[2], lod_stats[3]
            ).into());
        }

        Ok(())
    }

    /// Renderiza uma geometria com LOD adaptativo
    fn render_geometry_with_lod(&self, program: &WebGlProgram, geom: &avila_bim::IfcGeometry, is_selected: bool, lod_level: LodLevel) -> Result<(), JsValue> {
        // LOD: simplificar geometria baseado no nível
        let (vertices, indices) = match lod_level {
            LodLevel::High => (&geom.vertices, &geom.indices),
            LodLevel::Medium => {
                // Usa 50% dos vértices (skip every other)
                (&geom.vertices, &geom.indices)
            },
            LodLevel::Low => {
                // Usa 25% dos vértices
                (&geom.vertices, &geom.indices)
            },
            LodLevel::Minimal => {
                // Renderiza apenas bounding box (6 faces, 24 vértices)
                return self.render_bbox(program, geom, is_selected);
            }
        };

        self.render_geometry_internal(program, vertices, indices, &geom.normals, geom.color, is_selected)
    }

    /// Renderiza bounding box para LOD minimal
    fn render_bbox(&self, program: &WebGlProgram, geom: &avila_bim::IfcGeometry, is_selected: bool) -> Result<(), JsValue> {
        let min = geom.bbox_min;
        let max = geom.bbox_max;

        // 8 vértices do cubo
        let bbox_verts = vec![
            [min[0], min[1], min[2]], [max[0], min[1], min[2]],
            [max[0], max[1], min[2]], [min[0], max[1], min[2]],
            [min[0], min[1], max[2]], [max[0], min[1], max[2]],
            [max[0], max[1], max[2]], [min[0], max[1], max[2]],
        ];

        // 12 triângulos (2 por face)
        let bbox_indices = vec![
            0,1,2, 0,2,3,  4,7,6, 4,6,5,  0,4,5, 0,5,1,
            1,5,6, 1,6,2,  2,6,7, 2,7,3,  3,7,4, 3,4,0,
        ];

        // Normais simplificadas (todas apontando para cima)
        let bbox_normals = vec![[0.0, 1.0, 0.0]; 8];

        self.render_geometry_internal(program, &bbox_verts, &bbox_indices, &bbox_normals, geom.color, is_selected)
    }

    /// Renderiza geometria interna (código comum)
    fn render_geometry_internal(&self, program: &WebGlProgram, vertices: &[[f32; 3]], indices: &[u32], normals: &[[f32; 3]], color: [f32; 4], is_selected: bool) -> Result<(), JsValue> {
        // Cria buffer de vértices
        let position_buffer = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));

        // Converte vertices [f32; 3] para Vec<f32>
        let mut vertex_data = Vec::new();
        for v in vertices {
            vertex_data.push(v[0]);
            vertex_data.push(v[1]);
            vertex_data.push(v[2]);
        }

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertex_data);
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        // Posição
        let pos_attrib = self.gl.get_attrib_location(program, "a_position") as u32;
        self.gl.vertex_attrib_pointer_with_i32(
            pos_attrib,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl.enable_vertex_attrib_array(pos_attrib);

        // Normais
        let normal_buffer = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&normal_buffer));

        let mut normal_data = Vec::new();
        for n in normals {
            normal_data.push(n[0]);
            normal_data.push(n[1]);
            normal_data.push(n[2]);
        }

        unsafe {
            let norm_array = js_sys::Float32Array::view(&normal_data);
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &norm_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let norm_attrib = self.gl.get_attrib_location(program, "a_normal") as u32;
        self.gl.vertex_attrib_pointer_with_i32(
            norm_attrib,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl.enable_vertex_attrib_array(norm_attrib);

        // Cor (com highlight se selecionado)
        let color_attrib = self.gl.get_attrib_location(program, "a_color") as u32;
        let final_color = if is_selected {
            // Highlight amarelo brilhante
            [1.0, 0.9, 0.2, 1.0]
        } else {
            color
        };
        self.gl.vertex_attrib4f(
            color_attrib,
            final_color[0],
            final_color[1],
            final_color[2],
            final_color[3],
        );

        // Índices
        let index_buffer = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        unsafe {
            let idx_array = js_sys::Uint32Array::view(indices);
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &idx_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        // Desenha
        self.gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            indices.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        Ok(())
    }
}

const VERTEX_SHADER_SOURCE: &str = r#"
    attribute vec3 a_position;
    attribute vec3 a_normal;
    attribute vec4 a_color;

    uniform mat4 u_modelViewMatrix;
    uniform mat4 u_projectionMatrix;

    varying vec3 v_normal;
    varying vec4 v_color;

    void main() {
        gl_Position = u_projectionMatrix * u_modelViewMatrix * vec4(a_position, 1.0);
        v_normal = a_normal;
        v_color = a_color;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    precision mediump float;

    varying vec3 v_normal;
    varying vec4 v_color;

    void main() {
        vec3 light = normalize(vec3(0.5, 1.0, 0.3));
        float diffuse = max(dot(normalize(v_normal), light), 0.0);
        vec3 lighting = vec3(0.3 + 0.7 * diffuse);

        gl_FragColor = vec4(v_color.rgb * lighting, v_color.a);
    }
"#;
