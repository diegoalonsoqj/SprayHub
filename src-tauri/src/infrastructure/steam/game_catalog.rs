//! Data-driven catalog of supported Source games. Adding a new game is a single
//! entry here — the rest of the system is generic over `GameDefinition`.

use crate::domain::entities::GameDefinition;

/// The supported games. Extend this list to support more Source titles.
///
/// `sprays_relative` is the path from the install dir to the logos folder. For
/// Source games this lives inside the game's *gamedir* subfolder (e.g.
/// `Left 4 Dead 2/left4dead2/materials/vgui/logos`), not the install root.
pub const GAMES: &[GameDefinition] = &[
    GameDefinition {
        id: "left4dead2",
        name: "Left 4 Dead 2",
        app_id: 550,
        install_dir_name: "Left 4 Dead 2",
        sprays_relative: "left4dead2/materials/vgui/logos",
    },
    GameDefinition {
        id: "cstrike",
        name: "Counter-Strike: Source",
        app_id: 240,
        install_dir_name: "Counter-Strike Source",
        sprays_relative: "cstrike/materials/vgui/logos",
    },
    GameDefinition {
        id: "tf2",
        name: "Team Fortress 2",
        app_id: 440,
        install_dir_name: "Team Fortress 2",
        sprays_relative: "tf/materials/vgui/logos",
    },
    GameDefinition {
        id: "hl2dm",
        name: "Half-Life 2: Deathmatch",
        app_id: 320,
        install_dir_name: "Half-Life 2 Deathmatch",
        sprays_relative: "hl2mp/materials/vgui/logos",
    },
    GameDefinition {
        id: "garrysmod",
        name: "Garry's Mod",
        app_id: 4000,
        install_dir_name: "GarrysMod",
        sprays_relative: "garrysmod/materials/vgui/logos",
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
