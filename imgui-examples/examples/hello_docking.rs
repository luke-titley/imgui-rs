use imgui::*;

mod support;

fn main() {
    let mut system = support::init(file!());
    system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

    let mut first_time = true;
    system.main_loop(move |_, ui| {


        imgui::Window::new(im_str!("Main"))
            .flags(
                // No borders etc for top-level window
                imgui::WindowFlags::NO_DECORATION | imgui::WindowFlags::NO_MOVE |
                // Show menu bar
                imgui::WindowFlags::MENU_BAR |
                // Don't want the dock area to be dockable!
                .imgui::WindowFlags::NO_DOCKING |
                // Don't raise window on focus (as it'll clobber floating windows)
                imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS | imgui::WindowFlags::NO_NAV_FOCUS,
                )
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(ui.io().display_size, imgui::Condition::Always)
            .build(ui, || {
                ui.dockspace();

                ui.menu_bar(|| {

                    ui.menu(im_str!("File"), true, || {
                        imgui::MenuItem::new(im_str!("Quit")).enabled(true).build(ui);
                    });
                    ui.menu(im_str!("Help"), true, || {
                        ui.button(im_str!("Version"), [100.0, 20.0]);
                    });
                });

            });

        imgui::Window::new(im_str!("Node graph"))
            .build(ui, || {
            });
    });
}
