#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) {
    // Start the app using eframe's web runner
    eframe::WebRunner::new().start(
            canvas_id,
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(HouseholdApp::new(cc))),
        )
        .expect("Failed to start eframe");
}