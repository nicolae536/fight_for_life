use amethyst::{Application, GameDataBuilder};
use amethyst::renderer::{RenderFlat2D, RenderingBundle, RenderToWindow, types::DefaultBackend};
use amethyst::utils::application_root_dir;

mod fight_for_life;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir().unwrap();
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                    RenderToWindow::from_config_path(display_config_path)?
                        // window color
                        .with_clear([0.0, 0.0, 0.0, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
        ).unwrap();
    let assets_dir = app_root.join("assets");

    let mut game = Application::new(
        assets_dir,
        fight_for_life::FightForLife,
        game_data,
    ).unwrap();
    game.run();
    Ok(())
}
