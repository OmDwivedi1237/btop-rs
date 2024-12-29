use sysinfo::System;

pub fn get_memory_usage() -> f32 {
    let mut system = System::new_all();
    system.refresh_all();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let total_memory = system.total_memory();
    let used_memory = system.used_memory();

    if total_memory == 0 {
        return 0.0;
    }

    let memory_usage = ((used_memory as f32) / (total_memory as f32) * 100.0) as u32;

    return memory_usage as f32;
}
