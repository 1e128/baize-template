use baizekit::app::error::{ComponentSnafu, ConfigSnafu, ResultExt};
use baizekit::app::new_app;
use baizekit::component::{AxumComponent, DbComponent, LogComponent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    new_app!()
        .register_component_factory(None::<&str>, LogComponent::new)
        .register_component_factory(None::<&str>, DbComponent::new)
        .register_component_factory(None::<&str>, |ctx| {
            let mut services = vec![];
            AxumComponent::new(ctx, services)
        })
        .run()
        .await
        .map_err(Into::into)
}
