use imgui::*;

mod support;

fn main() {
    let mut system = support::init(file!());
    system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello"))
            .build(ui, || {
                ui.text(im_str!("Window A"));
            });
        Window::new(im_str!("Docking"))
            .build(ui, || {
                ui.text(im_str!("Window B"));
            });
    });
}
