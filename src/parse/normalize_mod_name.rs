fn find_version_index(s: &str) -> Option<usize> {
    let mut char_iter = s.chars().enumerate().peekable();
    while let Some((index, c)) = char_iter.next() {
        if c.is_ascii_digit() && char_iter.peek()?.1 == '.' {
            return Some(index);
        }
    }
    None
}

/// Produces strings that can be used to match mods to errors
pub(crate) fn normalize_mod_name(name: &str) -> String {
    let mut name = name.to_ascii_lowercase();

    fn take_left(s: &mut String, split: &str) {
        if let Some((n, _)) = s.split_once(split) {
            *s = n.to_string();
        }
    }
    take_left(&mut name, "-alpha");
    take_left(&mut name, "-beta");
    take_left(&mut name, "-fabric");
    take_left(&mut name, "-forge");
    take_left(&mut name, "-quilt");
    take_left(&mut name, "-mc");

    if let Some(ver_index) = find_version_index(&name) {
        if let Some(wo_version) = name.get(..ver_index) {
            name = wo_version.to_string();
        }
    }
    name = name
        .replace('-', "")
        .replace('_', "")
        .replace('[', "")
        .replace('\'', "")
        .replace(' ', "");

    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_index() {
        let index = find_version_index("natures_spirit-2.2.5-1.20.1").expect("Failed to find index");
        assert_eq!(index, 15);
    }

    #[test]
    fn simple() {
        assert_eq!(normalize_mod_name("BetterAdvancements-Fabric-1.21.10-0.4.7.48"), "betteradvancements");
        assert_eq!(normalize_mod_name("create-fabric-6.0.8.0 build.1734-mc1.20.1"), "create");
        assert_eq!(normalize_mod_name("natures_spirit-2.2.5-1.20.1"), "naturesspirit");
        assert_eq!(normalize_mod_name("MagicDoorknob-1.21.1-1.2.1.2415.jar"), "magicdoorknob");
        assert_eq!(normalize_mod_name("createmetallurgy-0.0.7-C6-1.20.1"), "createmetallurgy");
        assert_eq!(normalize_mod_name("fzzy_config-0.7.1 1.21.3"), "fzzyconfig");
        assert_eq!(normalize_mod_name("mcwbiomesoplenty-fabric-1.21.5-1.2"), "mcwbiomesoplenty");
        assert_eq!(normalize_mod_name("integratedterminals-1.21.1-neoforge-1.6.16.jar"), "integratedterminals");
        assert_eq!(normalize_mod_name("midnightlib-forge-1.9.2 1.20.1"), "midnightlib");
        assert_eq!(normalize_mod_name("MagicBees-Beta-3.0.4"), "magicbees");
        assert_eq!(normalize_mod_name("foamfix-0.6.3-anarchy"), "foamfix");
        assert_eq!(normalize_mod_name("entity_texture_features-7.0.4-1.21.9-fabric"), "entitytexturefeatures");
        assert_eq!(normalize_mod_name("EffectTimerPlus-Forge-1.20.1-1.1.1"), "effecttimerplus");
        assert_eq!(normalize_mod_name("ferritecore-8.0.2-fabric.jar"), "ferritecore");
        assert_eq!(normalize_mod_name("visuality-0.7.7 1.21"), "visuality");
        assert_eq!(normalize_mod_name("DynamicTrees-1.12.2-0.9.5"), "dynamictrees");
        assert_eq!(normalize_mod_name("BetterBuildersWands-0.13.3-GTNH"), "betterbuilderswands");
        assert_eq!(normalize_mod_name("ViaFabricPlus-4.3.0"), "viafabricplus");
        assert_eq!(normalize_mod_name("logprot-1.19.2-1.9"), "logprot");
        assert_eq!(normalize_mod_name("treeplacer-forge-1.19.2-1.1.1"), "treeplacer");
        assert_eq!(normalize_mod_name("Craftable-Saddles-[1.12]-1.5"), "craftablesaddles");
        assert_eq!(normalize_mod_name("ftb-library-forge-2001.2.11"), "ftblibrary");
        assert_eq!(normalize_mod_name("sophisticatedbackpacks-1.20.1-3.24.6.1366"), "sophisticatedbackpacks");
        assert_eq!(normalize_mod_name("letmedespawn-1.20.x-forge-1.4.4.jar"), "letmedespawn");
        assert_eq!(normalize_mod_name("jei-1.18.2-9.7.2.1001"), "jei");
        assert_eq!(normalize_mod_name("Jade-1.20.1-forge-11.6.3"), "jade");
        assert_eq!(normalize_mod_name("createultimine-1.21.1-neoforge-1.1.1.jar"), "createultimine");
        assert_eq!(normalize_mod_name("IAS-Fabric-1.21.1-9.0.2-alpha.1"), "ias");
        assert_eq!(normalize_mod_name("Dave's Potioneering-forge-1.20.1-13"), "davespotioneering");
        assert_eq!(normalize_mod_name("lostworlds-1.20-0.0.3"), "lostworlds");
        assert_eq!(normalize_mod_name("particle_core-0.2.6 1.21.6"), "particlecore");
        assert_eq!(normalize_mod_name("duckling-fabric-1.20.4-4.0.0"), "duckling");
        assert_eq!(normalize_mod_name("GuardRibbits-1.20.1-Forge-1.0.4.jar"), "guardribbits");
    }
}