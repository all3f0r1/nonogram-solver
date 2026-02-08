mod drawing;
mod edge_detection;
mod grid;
mod gui;
mod image_generator;
mod image_parser;
mod solver;

#[cfg(feature = "ocr")]
mod ocr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gui::run_gui()
}
