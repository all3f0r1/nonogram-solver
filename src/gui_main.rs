mod grid;
mod solver;
mod image_parser;
mod image_generator;
mod drawing;
mod edge_detection;
mod gui;

#[cfg(feature = "ocr")]
mod ocr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gui::run_gui()
}
