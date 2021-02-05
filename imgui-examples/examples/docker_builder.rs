 mod support;
 
 fn main() {
     let mut system = support::init(file!());
     system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

    let mut first_time = true;
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello")).build(ui, || {
            ui.text(im_str!("Hello"));
        });
        Window::new(im_str!("Docking")).build(ui, || {
            ui.text(im_str!("Docking"));
        });
        Window::new(im_str!("Awesome")).build(ui, || {
            ui.text(im_str!("Awesome"));
        });

        if first_time {
            first_time = false;
            imgui::Dock::new().build(|root| {
                root.size([512_f32, 512_f32])
                    .position([0_f32, 0_f32])
                    .split(
                        imgui::Direction::Left,
                        0.7_f32,
                        |left| {
                            left.dock_window(im_str!("Hello"));
                        },
                        |right| {
                               right.split(imgui::Direction::Up, 0.5_f32,
                                |top| {
                                    top.dock_window(im_str!("Docking"));
                                },
                                |bottom| {
                                    bottom.dock_window(im_str!("Awesome"));
                                }
                               );
                        },
                    )
            })
        }
    });
 }
