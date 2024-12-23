use sysinfo::System;

pub fn get_cpu_usage() -> f32 {
    let mut system = System::new_all();
    system.refresh_all();

    std::thread::sleep(std::time::Duration::from_millis(250));
    
    system.refresh_cpu_usage();
    
    let cpu_count = system.cpus().len() as f32;
    let average_cpu_usage: f32 = system.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .sum::<f32>() / cpu_count;
    average_cpu_usage
}