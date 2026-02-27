slint::include_modules!();
use sysinfo::System;
use slint::{VecModel, ComponentHandle, Model};
use std::rc::Rc;

fn main() -> Result<(), slint::PlatformError> {
    let mut sys = System::new_all();
    let main_window = MainWindow::new()?;
    main_window.window().set_position(slint::LogicalPosition::new(0.0, 0.0));
    
    let timer = slint::Timer::default();
    let main_window_weak = main_window.as_weak();

    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(500), move || {
        let main_window = main_window_weak.unwrap();
        sys.refresh_all();

        let total_ram = (sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0 * 10.0).round() / 10.0;
        let used_ram = (sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0 * 10.0).round() / 10.0;
        let cpu_num = sys.cpus().len();

        let core_infos: Vec<CoreInfo> = sys.cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| {
                let usage = cpu.cpu_usage();
                CoreInfo {
                    label: format!("Core {}: {:.1}%", i, usage).into(),
                    value: usage / 100.0,
                }
            })
            .collect();

        main_window.set_total_memory(format!("{:.1} GB used", used_ram).into());
        main_window.set_used_memory(format!("{:.1} GB total", total_ram).into());
        main_window.set_cpu_num(format!("{} CPU cores", cpu_num).into());
        main_window.set_core_data(Rc::new(VecModel::from(core_infos)).into());
    });

    main_window.run()
}