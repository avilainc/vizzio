use wasm_bindgen::prelude::*;
use avila_frontend::*;
use avila_bim_core::*;
use avila_ifc::IfcParser;
use std::collections::HashMap;
use web_sys::{console, window, HtmlCanvasElement};

mod renderer;
mod camera;
mod scene;

use renderer::Renderer;
use camera::VRCamera;
use scene::Scene;

#[wasm_bindgen]
pub struct CivilVRApp {
    ifc_model: Option<BimModel>,
    renderer: Option<Renderer>,
    camera: VRCamera,
    scene: Scene,
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl CivilVRApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CivilVRApp {
        console_error_panic_hook::set_once();
        console::log_1(&"??? Initializing Civil VR App".into());

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("vr-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        CivilVRApp {
            ifc_model: None,
            renderer: None,
            camera: VRCamera::new(),
            scene: Scene::new(),
            canvas,
        }
    }

    #[wasm_bindgen]
    pub fn load_ifc(&mut self, ifc_content: &str) {
        console::log_1(&"?? Loading IFC model...".into());

        // Stub IFC loader - create simple model
        let mut model = BimModel::new("Stub IFC Model", IfcSchema::Ifc4);

        // Add some sample elements
        let wall = BimElement::new("IFCWALL");
        model.add_element(wall);

        let column = BimElement::new("IFCCOLUMN");
        model.add_element(column);

        let beam = BimElement::new("IFCBEAM");
        model.add_element(beam);

        console::log_1(&format!("? Loaded stub IFC model: {} elements", model.elements.len()).into());

        self.ifc_model = Some(model);
        match self.build_scene() {
            Ok(_) => console::log_1(&"Scene built".into()),
            Err(e) => console::error_1(&format!("Scene error: {}", e).into()),
        }
        match self.initialize_renderer() {
            Ok(_) => console::log_1(&"Renderer initialized".into()),
            Err(e) => console::error_1(&format!("Renderer error: {}", e).into()),
        }
    }

    fn build_scene(&mut self) -> Result<()> {
        if let Some(model) = &self.ifc_model {
            self.scene.clear();

            // Add elements to scene
            for (guid, element) in &model.elements {
                if let Some(geometry) = &element.geometry {
                    match self.scene.add_element(element, geometry) {
                        Ok(_) => (),
                        Err(e) => return Err(avila_bim_core::BimError::InvalidGeometry(e)),
                    }
                }
            }

            // Set camera to center of model
            if let Some(bounds) = self.scene.get_bounds() {
                self.camera.set_position(bounds.center());
            }
        }

        Ok(())
    }

    fn initialize_renderer(&mut self) -> Result<()> {
        let renderer = Renderer::new(&self.canvas)?;
        self.renderer = Some(renderer);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn render_frame(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            renderer.render(&self.scene, &self.camera);
        }
    }

    #[wasm_bindgen]
    pub fn update_camera(&mut self, delta_time: f32, move_forward: bool, move_backward: bool, strafe_left: bool, strafe_right: bool) {
        self.camera.update(delta_time, move_forward, move_backward, strafe_left, strafe_right);
    }

    #[wasm_bindgen]
    pub fn rotate_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.camera.rotate(delta_x, delta_y);
    }
}
````````

This completes the code transformation as per the provided changes. The final result is a syntactically valid, properly formatted, and correctly indented Rust file, with the specified code changes incorporated.