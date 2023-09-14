use clap::Parser;
use eyre::Result;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use twmap::{
    AnyTile, FrontLayer, GameLayer, Group, Layer, Load, SpeedupLayer, SwitchLayer, TeleLayer,
    TilemapLayer, TilesLayer, TuneLayer, TwMap,
};
use vek::Rgba;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The base map to compare.
    map_old: PathBuf,
    /// The map to be compared.
    map_new: PathBuf,
    /// The resulting map path.
    result: PathBuf,
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    let Cli {
        map_old,
        map_new,
        result: result_path,
    } = cli;

    if !map_old.exists() {
        eprintln!("{} map doesn't exist!", map_old.display().purple());
        return Ok(());
    }

    if !map_new.exists() {
        eprintln!("{} map doesn't exist!", map_new.display().purple());
        return Ok(());
    }

    let mut map_old = TwMap::parse_file(map_old)?;
    map_old.load()?;
    let mut map_new = TwMap::parse_file(map_new)?;
    map_new.load()?;
    let mut result = map_old.clone();

    let map_old_game: &GameLayer = map_old
        .find_physics_layer()
        .expect("couldn't find game layer");
    let map_new_game: &GameLayer = map_new
        .find_physics_layer()
        .expect("couldn't find game layer");

    let game_diff_group = diff_layer(map_old_game, map_new_game, "Game")?;
    result.groups.push(game_diff_group);

    if let (Some(map_old_layer), Some(map_new_layer)) = (
        map_old.find_physics_layer::<FrontLayer>(),
        map_new.find_physics_layer::<FrontLayer>(),
    ) {
        let diff_group = diff_layer(map_old_layer, map_new_layer, "Front")?;
        result.groups.push(diff_group);
    }

    if let (Some(map_old_layer), Some(map_new_layer)) = (
        map_old.find_physics_layer::<TeleLayer>(),
        map_new.find_physics_layer::<TeleLayer>(),
    ) {
        let diff_group = diff_layer(map_old_layer, map_new_layer, "Tele")?;
        result.groups.push(diff_group);
    }

    if let (Some(map_old_layer), Some(map_new_layer)) = (
        map_old.find_physics_layer::<SwitchLayer>(),
        map_new.find_physics_layer::<SwitchLayer>(),
    ) {
        let diff_group = diff_layer(map_old_layer, map_new_layer, "Switch")?;
        result.groups.push(diff_group);
    }

    if let (Some(map_old_layer), Some(map_new_layer)) = (
        map_old.find_physics_layer::<SpeedupLayer>(),
        map_new.find_physics_layer::<SpeedupLayer>(),
    ) {
        let diff_group = diff_layer(map_old_layer, map_new_layer, "Speedup")?;
        result.groups.push(diff_group);
    }

    if let (Some(map_old_layer), Some(map_new_layer)) = (
        map_old.find_physics_layer::<TuneLayer>(),
        map_new.find_physics_layer::<TuneLayer>(),
    ) {
        let diff_group = diff_layer(map_old_layer, map_new_layer, "Tune")?;
        result.groups.push(diff_group);
    }

    result.save_file(&result_path)?;

    Ok(())
}

fn diff_layer<T>(layer_old: &T, layer_new: &T, name: &str) -> Result<Group>
where
    T: TilemapLayer,
{
    let tiles_old = layer_old.tiles().unwrap_ref();
    let tiles_new = layer_new.tiles().unwrap_ref();

    let width = tiles_old.dim().1.min(tiles_new.dim().1);
    let height = tiles_old.dim().0.min(tiles_new.dim().0);

    let mut diff_group = Group {
        name: format!("{} Diff", name),
        ..Default::default()
    };

    let mut layer_add_tiles = TilesLayer::new((width, height));
    layer_add_tiles.name = "Added".to_string();
    layer_add_tiles.color = Rgba::new(0, 255, 0, 64);
    layer_add_tiles.tiles.load()?;
    let tiles_add = layer_add_tiles.tiles.unwrap_mut();

    let mut layer_del_tiles = TilesLayer::new((width, height));
    layer_del_tiles.name = "Deleted".to_string();
    layer_del_tiles.color = Rgba::new(255, 0, 0, 64);
    let tiles_del = layer_del_tiles.tiles.unwrap_mut();

    let mut layer_mod_tiles = TilesLayer::new((width, height));
    layer_mod_tiles.name = "Modified".to_string();
    layer_mod_tiles.color = Rgba::new(255, 255, 0, 64);
    let tiles_mod = layer_mod_tiles.tiles.unwrap_mut();

    let mut additions = 0;
    let mut deletions = 0;
    let mut modifications = 0;

    for x in 0..width {
        for y in 0..height {
            let index_old = tiles_old[(y, x)].id();
            let index_new = tiles_new[(y, x)].id();

            if index_old == 0 && index_new != 0 {
                tiles_add[(y, x)].id = 1;
                additions += 1;
            } else if index_old != 0 && index_new == 0 {
                tiles_del[(y, x)].id = 1;
                deletions += 1;
            } else if index_old != index_new {
                tiles_mod[(y, x)].id = 1;
                modifications += 1;
            }
        }
    }

    let layer_add = Layer::Tiles(layer_add_tiles);
    let layer_del = Layer::Tiles(layer_del_tiles);
    let layer_mod = Layer::Tiles(layer_mod_tiles);

    diff_group.layers.push(layer_add);
    diff_group.layers.push(layer_del);
    diff_group.layers.push(layer_mod);

    println!("Summary of changes to layer '{}':", name.purple());
    println!("- {}: {}", "Additions".green(), additions);
    println!("- {}: {}", "Deletions".red(), deletions);
    println!(
        "- {}: {}",
        "Modifications".fg_rgb::<255, 255, 0>(),
        modifications
    );

    Ok(diff_group)
}
