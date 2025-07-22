use baizekit::api::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::default()
        .with_project_path("./".to_string())
        .with_handlers_dir("./src/service".to_string())
        .with_app_state("crate::setup::state::AppState".to_string())
        .with_output_path("src/setup".to_string())
        .with_output_name("http".to_string())
        .build()?;

    Ok(())
}
