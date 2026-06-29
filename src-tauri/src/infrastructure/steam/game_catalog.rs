//! Data-driven catalog of supported Source games. Adding a new game is a single
//! entry here — the rest of the system is generic over `GameDefinition`.

use crate::domain::entities::GameDefinition;

/// Relative path, from a game's install dir, to the sprays/logos folder.
/// This is the same for all listed Source games.
const LOGOS: &str = "materials/vgui/logos";

/// The supported games. Extend this list to support more Source titles.
pub const GAMES: &[GameDefinition] = &[
    GameDefinition {
        id: "left4dead2",
        name: "Left 4 Dead 2",
        app_id: 550,
        install_dir_name: "Left 4 Dead 2",
        sprays_relative: LOGOS,
    },
    GameDefinition {
        id: "cstrike",
        name: "Counter-Strike: Source",
        app_id: 240,
        install_dir_name: "Counter-Strike Source",
        sprays_relative: LOGOS,
    },
    GameDefinition {
        id: "tf2",
        name: "Team Fortress 2",
        app_id: 440,
        install_dir_name: "Team Fortress 2",
        sprays_relative: LOGOS,
    },
    GameDefinition {
        id: "hl2dm",
        name: "Half-Life 2: Deathmatch",
        app_id: 320,
        install_dir_name: "Half-Life 2 Deathmatch",
        sprays_relative: LOGOS,
    },
    GameDefinition {
        id: "garrysmod",
        name: "Garry's Mod",
        app_id: 4000,
        install_dir_name: "GarrysMod",
        sprays_relative: LOGOS,
    },
];

/// Look up a catalog entry by its slug id.
pub fn find_by_id(id: &str) -> Option<&'static GameDefinition> {
    GAMES.iter().find(|g| g.id == id)
}

/// Look up a catalog entry by its Steam App ID.
pub fn find_by_app_id(app_id: u32) -> Option<&'static GameDefinition> {
    GAMES.iter().find(|g| g.app_id == app_id)
}
