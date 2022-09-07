#![allow(unused)]

use super::preprocessor::process_single;
use std::error::Error;

#[test]
fn with_std() -> Result<(), Box<dyn Error>> {
    let argv = &vec![String::new(), String::from("./folder/folder/file.sf")];
    let target = "# Assume `scoreboard objectives add click minecraft.used:minecraft.carrot_on_a_stick` to be defined in setup function
    =target @e[nbt={SelectedItem:{id:\"minecraft:carrot_on_a_stick\",tag:{CustomModelData:1}}},scores={click=1..}]
    =pos ^ ^5 ^
    =lightning minecraft:lightning_bolt
    
    do as target here run summon lightning pos
    do as target run scoreboard players set @s click 0".as_bytes().to_vec();
    let res = process_single(argv, target)?;
    println!("\n\n{}\n\n", res.contents.trim());
    assert_eq!(res.contents.trim(), "# Assume `scoreboard objectives add click minecraft.used:minecraft.carrot_on_a_stick` to be defined in setup function

    execute as @e[nbt={SelectedItem:{id:\"minecraft:carrot_on_a_stick\",tag:{CustomModelData:1}}},scores={click=1..}] at @s run summon minecraft:lightning_bolt ^ ^5 ^
    execute as @e[nbt={SelectedItem:{id:\"minecraft:carrot_on_a_stick\",tag:{CustomModelData:1}}},scores={click=1..}] run scoreboard players set @s click 0".to_string());
    Ok(())
}
