#[derive(PartialEq, Debug)]
pub enum ModLoader {
    Fabric,
    Forge,
    Vanilla,
    Quilt,
    RetroFuturaBootstrap,
}

pub fn main_class_to_mod_loader(class: &str) -> Option<ModLoader> {
    if class.starts_with("net.fabricmc.loader.") {
        return Some(ModLoader::Fabric);
    }
    if class == "io.github.zekerzhayard.forgewrapper.installer.Main" {
        return Some(ModLoader::Forge);
    }
    if class.starts_with("net.minecraft.") {
        return Some(ModLoader::Vanilla);
    }
    if class == "org.quiltmc.loader.impl.launch.knot.KnotClient" {
        return Some(ModLoader::Quilt);
    }
    if class.starts_with("com.gtnewhorizons.retrofuturabootstrap.") {
        return Some(ModLoader::RetroFuturaBootstrap);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fabric() {
        for class in ["net.fabricmc.loader.launch.knot.KnotClient", "net.fabricmc.loader.impl.launch.knot.KnotClient"] {
            let loader = main_class_to_mod_loader(class).unwrap();
            assert_eq!(loader, ModLoader::Fabric);
        }
    }

    #[test]
    fn forge() {
        let class = "io.github.zekerzhayard.forgewrapper.installer.Main";
        let loader = main_class_to_mod_loader(class).unwrap();
        assert_eq!(loader, ModLoader::Forge);
    }

    #[test]
    fn vanilla() {
        for class in ["net.minecraft.launchwrapper.Launch", "net.minecraft.client.main.Main", "net.minecraft.client.Minecraft"] {
            let loader = main_class_to_mod_loader(class).unwrap();
            assert_eq!(loader, ModLoader::Vanilla);
        }
    }

    #[test]
    fn quilt() {
        let class = "org.quiltmc.loader.impl.launch.knot.KnotClient";
        let loader = main_class_to_mod_loader(class).unwrap();
        assert_eq!(loader, ModLoader::Quilt);
    }

    #[test]
    fn retro_futura_bootstrap() {
        for class in ["com.gtnewhorizons.retrofuturabootstrap.Main", "com.gtnewhorizons.retrofuturabootstrap.MainStartOnFirstThread"] {
            let loader = main_class_to_mod_loader(class).unwrap();
            assert_eq!(loader, ModLoader::RetroFuturaBootstrap);
        }
    }
}