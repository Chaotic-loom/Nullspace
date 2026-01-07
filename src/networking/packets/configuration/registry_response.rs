use std::collections::HashSet;
use crate::networking::connection::Connection;
use crate::networking::data_types::registries::RegistryData;

pub async fn send_all_registries(connection: &mut Connection) -> anyhow::Result<()> {
    println!("Sending Registry Data...");

    let whitelist: HashSet<&'static str> = HashSet::from([
        "minecraft_damage_type.bin",
        "minecraft_worldgen_biome.bin",
        "minecraft_dimension_type.bin",
        "minecraft_cat_variant.bin",
        "minecraft_chicken_variant.bin",
        "minecraft_cow_variant.bin",
        "minecraft_frog_variant.bin",
        "minecraft_pig_variant.bin",
        "minecraft_wolf_variant.bin",
        "minecraft_wolf_sound_variant.bin",
        "minecraft_painting_variant.bin",
        "minecraft_zombie_nautilus_variant.bin",
        "minecraft_timeline.bin",
        //"minecraft_tags_timeline.bin",
    ]);

    for filename in RegistryData::iter() {
        let name = filename.as_ref();

        if !whitelist.contains(name) {
            continue;
        }

        if name == "packet_tags.bin" {
            continue;
        }

        if let Some(file) = RegistryData::get(name) {
            connection.send_raw_packet(0x07, &file.data).await?;
            println!("Sent registry: {}", name);
        }
    }

    // Sends the "Update tags - 0x0D" packet
    if let Some(file) = RegistryData::get("packet_tags.bin") {
        println!("Sending Tag Update...");
        connection.send_raw_packet(0x0D, &file.data).await?;
    }

    Ok(())
}