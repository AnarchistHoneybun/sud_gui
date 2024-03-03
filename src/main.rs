use eframe::egui::{CentralPanel, CtxRef};
use eframe::epi::{App, Frame};
use eframe::{NativeOptions, run_native};

struct Sudoku;

impl App for Sudoku {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, Sudoku!");
        });
    }

    fn name(&self) -> &str {
        "Sudoku"
    }
}


fn main() {
    let app = Sudoku;
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}