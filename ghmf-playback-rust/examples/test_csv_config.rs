use ghmf_playback::config::CsvConfig;
use tracing_subscriber;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Testing CSV configuration loading...\n");
    
    // Load configuration
    match CsvConfig::load_from_dir("Config") {
        Ok(config) => {
            println!("✅ Configuration loaded successfully!\n");
            
            println!("📊 Statistics:");
            println!("  - Colors: {}", config.colors.len());
            println!("  - Fixtures: {}", config.fixtures.len());
            println!("  - FCW Mappings: {}", config.fcw_mappings.len());
            
            // Test color lookup
            println!("\n🎨 Sample Colors:");
            if let Some(color) = config.get_color(1) {
                println!("  Color 001: {} ({})", color.hex_color, color.description);
            }
            if let Some(color) = config.get_color(8) {
                println!("  Color 008: {} ({})", color.hex_color, color.description);
            }
            
            // Test fixture lookup
            println!("\n💡 Sample Fixtures:");
            if let Some(fixture) = config.get_fixture(1) {
                println!("  Fixture 1: {} at DMX channel {} ({:?})", 
                    fixture.note, fixture.dmx_channel, fixture.format);
            }
            if let Some(fixture) = config.get_fixture(42) {
                println!("  Fixture 42: {} at DMX channel {} ({:?})", 
                    fixture.note, fixture.dmx_channel, fixture.format);
            }
            
            // Test FCW mapping
            println!("\n🎯 Sample FCW Mappings:");
            if let Some(mapping) = config.get_fcw_mapping(51) {
                println!("  FCW 051 affects {} fixtures", mapping.fixture_directives.len());
            }
            if let Some(mapping) = config.get_fcw_mapping(17) {
                println!("  FCW 017 affects {} fixtures", mapping.fixture_directives.len());
            }
            
            println!("\n✨ All tests passed!");
        }
        Err(e) => {
            eprintln!("❌ Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    }
}
