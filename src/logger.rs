use log::LevelFilter;



pub fn setup_logger() {
	let _ = env_logger::builder().filter_level(LevelFilter::Info).try_init();
}