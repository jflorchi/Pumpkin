use crate::biome::Biome;
use pumpkin_util::version::JavaMinecraftVersion;

/// Remaps a biome registry id from the server's current version (26.2) to the
/// equivalent id for an older client version.
///
/// Unlike block states, biome ids have no generated per-version mapping table, since
/// there is no upstream mapping data source for them. This only accounts for the one
/// known difference between 26.1 and 26.2: `minecraft:sulfur_caves` was inserted
/// alphabetically at id 53 in 26.2, which doesn't exist in 26.1 and shifts every biome
/// registered after it by one.
#[must_use]
pub fn remap_biome_id_for_version(biome_id: u8, version: JavaMinecraftVersion) -> u8 {
    const SULFUR_CAVES_ID: u8 = Biome::SULFUR_CAVES.id;

    match version {
        JavaMinecraftVersion::V_26_1 => match biome_id.cmp(&SULFUR_CAVES_ID) {
            std::cmp::Ordering::Less => biome_id,
            std::cmp::Ordering::Equal => Biome::LUSH_CAVES.id,
            std::cmp::Ordering::Greater => biome_id - 1,
        },
        _ => biome_id,
    }
}
