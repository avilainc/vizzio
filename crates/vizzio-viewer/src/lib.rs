//! Vizzio Viewer Library - WASM bindings

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use avila_bim::{IfcModel, tree::{TreeNode, build_tree_from_geometries, tree_to_json}};

#[cfg(target_arch = "wasm32")]
use avila_vision::{Scene, WebGLRenderer};

#[cfg(target_arch = "wasm32")]
use avila_vision::raycast::{Ray, RayHit};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"🏗️ Vizzio Viewer WASM initialized".into());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct VizzioViewer {
    scene: Scene,
    renderer: Option<WebGLRenderer>,
    selected_index: Option<usize>,
    tree: Option<TreeNode>,
    visible_indices: Vec<usize>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl VizzioViewer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            renderer: None,
            selected_index: None,
            tree: None,
            visible_indices: Vec::new(),
        }
    }

    /// Inicializa renderer no canvas
    #[wasm_bindgen]
    pub fn init_canvas(&mut self, canvas_id: &str) -> Result<(), JsValue> {
        let mut renderer = WebGLRenderer::new(canvas_id)?;
        renderer.init_shaders()?;
        self.renderer = Some(renderer);
        Ok(())
    }

    /// Carrega arquivo IFC
    #[wasm_bindgen]
    pub fn load_ifc(&mut self, ifc_content: &str) -> Result<(), JsValue> {
        let model = IfcModel::from_step(ifc_content)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        let geometries = model.extract_geometry()
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        self.scene.load_ifc_model(geometries);

        // Constrói árvore hierárquica
        self.tree = Some(build_tree_from_geometries(&self.scene.geometries));

        // Inicialmente todos visíveis
        self.visible_indices = (0..self.scene.geometries.len()).collect();

        web_sys::console::log_1(&format!("✅ IFC loaded: {} geometries", self.scene.geometries.len()).into());

        Ok(())
    }

    /// Renderiza frame
    #[wasm_bindgen]
    pub fn render(&self) -> Result<(), JsValue> {
        if let Some(renderer) = &self.renderer {
            renderer.render(&self.scene, self.selected_index, &self.visible_indices)?;
        }
        Ok(())
    }

    /// Orbita câmera
    #[wasm_bindgen]
    pub fn orbit_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.scene.camera.orbit(delta_x * 0.01, delta_y * 0.01);
    }

    /// Zoom câmera
    #[wasm_bindgen]
    pub fn zoom_camera(&mut self, delta: f32) {
        self.scene.camera.zoom(delta);
    }

    /// Define matriz de view para XR
    #[wasm_bindgen]
    pub fn set_xr_view_matrix(&mut self, matrix: Vec<f32>) {
        if matrix.len() == 16 {
            self.scene.camera.view_matrix = matrix.try_into().unwrap();
        }
    }

    /// Define matriz de projeção para XR
    #[wasm_bindgen]
    pub fn set_xr_projection_matrix(&mut self, matrix: Vec<f32>) {
        if matrix.len() == 16 {
            self.scene.camera.projection_matrix = matrix.try_into().unwrap();
        }
    }

    /// Obtém número de geometrias carregadas
    #[wasm_bindgen]
    pub fn geometry_count(&self) -> usize {
        self.scene.geometries.len()
    }

    /// Testa seleção de objeto com raycasting
    ///
    /// # Arguments
    /// * `screen_x` - Coordenada X do mouse normalizada (-1 a 1)
    /// * `screen_y` - Coordenada Y do mouse normalizada (-1 a 1)
    ///
    /// # Returns
    /// Índice da geometria selecionada, ou -1 se nada foi atingido
    #[wasm_bindgen]
    pub fn pick_object(&mut self, screen_x: f32, screen_y: f32) -> i32 {
        let view_matrix = self.scene.camera.view_matrix();
        let proj_matrix = self.scene.camera.projection_matrix();

        let ray = Ray::from_screen(screen_x, screen_y, &view_matrix, &proj_matrix);

        if let Some(hit) = ray.intersect_geometries(&self.scene.geometries) {
            self.selected_index = Some(hit.geometry_index);
            web_sys::console::log_1(&format!(
                "🎯 Objeto selecionado: {} (distância: {:.2}m)",
                hit.geometry_index, hit.distance
            ).into());
            hit.geometry_index as i32
        } else {
            self.selected_index = None;
            -1
        }
    }

    /// Obtém índice do objeto atualmente selecionado
    #[wasm_bindgen]
    pub fn get_selected_index(&self) -> i32 {
        self.selected_index.map(|idx| idx as i32).unwrap_or(-1)
    }

    /// Limpa seleção
    #[wasm_bindgen]
    pub fn clear_selection(&mut self) {
        self.selected_index = None;
    }

    /// Obtém informações do objeto selecionado
    /// Retorna string JSON com: {"id": 123, "type": "IFCWALL", "vertices": 8, "triangles": 12}
    #[wasm_bindgen]
    pub fn get_selected_info(&self) -> String {
        if let Some(idx) = self.selected_index {
            if let Some(geom) = self.scene.geometries.get(idx) {
                return format!(
                    "{{\"id\":{},\"type\":\"{}\",\"vertices\":{},\"triangles\":{}}}",
                    geom.entity_id,
                    geom.entity_type,
                    geom.vertices.len(),
                    geom.indices.len() / 3
                );
            }
        }

        "{}".to_string()
    }

    /// Obtém árvore hierárquica em JSON
    #[wasm_bindgen]
    pub fn get_tree_json(&self) -> String {
        if let Some(tree) = &self.tree {
            tree_to_json(tree)
        } else {
            "{}".to_string()
        }
    }

    /// Define visibilidade de um nó da árvore
    #[wasm_bindgen]
    pub fn set_node_visibility(&mut self, node_id: &str, _visible: bool) {
        if let Some(tree) = &mut self.tree {
            if let Some(node) = tree.find_node_mut(node_id) {
                // Toggle visibility
                let new_visibility = !node.visible;
                node.set_visibility_recursive(new_visibility);

                // Atualiza lista de índices visíveis
                self.visible_indices.clear();
                if let Some(tree) = &self.tree {
                    tree.collect_visible_geometry_indices(&mut self.visible_indices);
                }

                web_sys::console::log_1(&format!(
                    "👁️ Visibilidade: {} objetos visíveis",
                    self.visible_indices.len()
                ).into());
            }
        }
    }

    /// Expande/colapsa nó da árvore
    #[wasm_bindgen]
    pub fn toggle_node_expanded(&mut self, node_id: &str) {
        if let Some(tree) = &mut self.tree {
            if let Some(node) = tree.find_node_mut(node_id) {
                node.expanded = !node.expanded;
            }
        }
    }

    /// Seleciona objeto pela árvore
    #[wasm_bindgen]
    pub fn select_from_tree(&mut self, geometry_index: i32) {
        if geometry_index >= 0 && (geometry_index as usize) < self.scene.geometries.len() {
            self.selected_index = Some(geometry_index as usize);
        } else {
            self.selected_index = None;
        }
    }
}
