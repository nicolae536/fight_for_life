use amethyst::{Application, GameDataBuilder};
use amethyst::utils::application_root_dir;

mod fight_for_life;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir().unwrap();
    // let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default();
    let assets_dir = app_root.join("assets");

    let mut game = Application::new(
        assets_dir,
        fight_for_life::FightForLife,
        game_data,
    ).unwrap();
    game.run();
    Ok(())
}
