use sysinfo::System;

pub fn get_memory_usage() -> f32 {
    let mut system = System::new_all();
    system.refresh_all();

    std::thread::sleep(std::time::Duration::from_millis(250));

    system.refresh_all();

    let memory_usage: u64 = system.used_memory() / system.total_memory();

    return memory_usage as f32;
}
