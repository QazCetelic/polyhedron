use crate::{entries::entry::LogEntry, issues::issue::Issue, parse::stacktrace::model::Stacktrace};

pub(crate) fn mixin_apply_failure_entry(entry: &LogEntry) -> Option<Issue> {
    let (mod_name, _rest) = entry
        .contents
        .strip_prefix("Mixin apply for mod ")?
        .split_once(" failed ")?;
    Some(Issue::MixinApplyFailure(mod_name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r#"[00:11:25] [main/ERROR]: Mixin apply for mod owo failed owo.mixins.json:serialization.NbtCompoundMixin from mod owo -> net.minecraft.class_2487: org.spongepowered.asm.mixin.transformer.throwables.InvalidMixinException @Shadow method method_10551 in owo.mixins.json:serialization.NbtCompoundMixin from mod owo was not located in the target class net.minecraft.class_2487. Using refmap owo-lib-refmap.json
org.spongepowered.asm.mixin.transformer.throwables.InvalidMixinException: @Shadow method method_10551 in owo.mixins.json:serialization.NbtCompoundMixin from mod owo was not located in the target class net.minecraft.class_2487. Using refmap owo-lib-refmap.json
	at org.spongepowered.asm.mixin.transformer.MixinPreProcessorStandard.attachSpecialMethod(MixinPreProcessorStandard.java:436)
	at org.spongepowered.asm.mixin.transformer.MixinPreProcessorStandard.attachShadowMethod(MixinPreProcessorStandard.java:412)
	at org.spongepowered.asm.mixin.transformer.MixinPreProcessorStandard.attachMethods(MixinPreProcessorStandard.java:340)
	at org.spongepowered.asm.mixin.transformer.MixinPreProcessorStandard.attach(MixinPreProcessorStandard.java:299)
	at org.spongepowered.asm.mixin.transformer.MixinPreProcessorStandard.createContextFor(MixinPreProcessorStandard.java:277)
	at org.spongepowered.asm.mixin.transformer.MixinInfo.createContextFor(MixinInfo.java:1291)
	at org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.apply(MixinApplicatorStandard.java:203)
	at org.spongepowered.asm.mixin.transformer.TargetClassContext.apply(TargetClassContext.java:437)
	at org.spongepowered.asm.mixin.transformer.TargetClassContext.applyMixins(TargetClassContext.java:418)
	at org.spongepowered.asm.mixin.transformer.MixinProcessor.applyMixins(MixinProcessor.java:352)
	at org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClass(MixinTransformer.java:237)
	at org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClassBytes(MixinTransformer.java:202)
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.getPostMixinClassByteArray(KnotClassDelegate.java:435)
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.tryLoadClass(KnotClassDelegate.java:336)
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.loadClass(KnotClassDelegate.java:231)
	at net.fabricmc.loader.impl.launch.knot.KnotClassLoader.loadClass(KnotClassLoader.java:119)
	at java.base/java.lang.ClassLoader.loadClass(ClassLoader.java:526)
	at knot//net.minecraft.class_9135.method_57998(class_9135.java:285)
	at knot//net.minecraft.class_9135.<clinit>(class_9135.java:296)
	at knot//net.minecraft.class_8824.<clinit>(class_8824.java:40)
	at knot//net.minecraft.class_2588.<clinit>(class_2588.java:47)
	at knot//net.minecraft.class_2561.method_43471(class_2561.java:153)
	at knot//net.minecraft.class_2156.<clinit>(class_2156.java:24)
	at knot//net.minecraft.class_155.<clinit>(class_155.java:276)
	at knot//net.minecraft.client.main.Main.main(Main.java:127)
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:514)
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72)
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)"#;
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries
            .iter()
            .filter_map(|e| mixin_apply_failure_entry(e))
            .next()
            .expect("Failed to determine issue");
        assert_eq!(issue, Issue::MixinApplyFailure("owo".to_string()));
    }

    #[test]
    fn example_2() {
        let text = r#"[20:52:08] [Render thread/ERROR]: Mixin apply for mod fightorflight failed fightorflight.mixins_common.json:PokemonEntityMixin from mod fightorflight -> com.cobblemon.mod.common.entity.pokemon.PokemonEntity: org.spongepowered.asm.mixin.injection.throwables.InvalidInjectionException Critical injection failure: @Inject annotation on registerFOFGoals could not find any targets matching 'Lcom/cobblemon/mod/common/entity/pokemon/PokemonEntity;method_5959()V' in com/cobblemon/mod/common/entity/pokemon/PokemonEntity. Using refmap fightorflight-common-refmap.json [INJECT_PREPARE Applicator Phase -> fightorflight.mixins_common.json:PokemonEntityMixin from mod fightorflight -> Prepare Injections -> handler$cmg000$fightorflight$registerFOFGoals(Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfo;)V -> Parse ->  -> Validate Targets]
org.spongepowered.asm.mixin.injection.throwables.InvalidInjectionException: Critical injection failure: @Inject annotation on registerFOFGoals could not find any targets matching 'Lcom/cobblemon/mod/common/entity/pokemon/PokemonEntity;method_5959()V' in com/cobblemon/mod/common/entity/pokemon/PokemonEntity. Using refmap fightorflight-common-refmap.json [INJECT_PREPARE Applicator Phase -> fightorflight.mixins_common.json:PokemonEntityMixin from mod fightorflight -> Prepare Injections -> handler$cmg000$fightorflight$registerFOFGoals(Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfo;)V -> Parse ->  -> Validate Targets]
	at org.spongepowered.asm.mixin.injection.selectors.TargetSelectors.validate(TargetSelectors.java:346) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.injection.struct.InjectionInfo.readAnnotation(InjectionInfo.java:369) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.injection.struct.InjectionInfo.<init>(InjectionInfo.java:340) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.injection.struct.InjectionInfo.<init>(InjectionInfo.java:331) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.injection.struct.CallbackInjectionInfo.<init>(CallbackInjectionInfo.java:48) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at java.base/jdk.internal.reflect.DirectConstructorHandleAccessor.newInstance(DirectConstructorHandleAccessor.java:62) ~[?:?]
	at java.base/java.lang.reflect.Constructor.newInstanceWithCaller(Constructor.java:502) ~[?:?]
	at java.base/java.lang.reflect.Constructor.newInstance(Constructor.java:486) ~[?:?]
	at org.spongepowered.asm.mixin.injection.struct.InjectionInfo$InjectorEntry.create(InjectionInfo.java:196) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.injection.struct.InjectionInfo.parse(InjectionInfo.java:664) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinTargetContext.prepareInjections(MixinTargetContext.java:1399) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.prepareInjections(MixinApplicatorStandard.java:731) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.applyMixin(MixinApplicatorStandard.java:315) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.apply(MixinApplicatorStandard.java:246) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.TargetClassContext.apply(TargetClassContext.java:437) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.TargetClassContext.applyMixins(TargetClassContext.java:418) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinProcessor.applyMixins(MixinProcessor.java:352) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClass(MixinTransformer.java:237) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClassBytes(MixinTransformer.java:202) ~[sponge-mixin-0.16.3 mixin.0.8.7.jar:0.16.3 mixin.0.8.7]
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.getPostMixinClassByteArray(KnotClassDelegate.java:435) ~[fabric-loader-0.17.2.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.tryLoadClass(KnotClassDelegate.java:336) ~[fabric-loader-0.17.2.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.loadClass(KnotClassDelegate.java:231) ~[fabric-loader-0.17.2.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClassLoader.loadClass(KnotClassLoader.java:119) ~[fabric-loader-0.17.2.jar:?]
	at java.base/java.lang.ClassLoader.loadClass(ClassLoader.java:526) ~[?:?]
	at knot/com.necro.raid.dens.fabric.network.NetworkMessages.registerPayload(NetworkMessages.java:28) ~[cobblemonraiddens-fabric-0.5.1 1.21.1.jar:?]
	at knot/com.necro.raid.dens.fabric.CobblemonRaidDensFabric.onInitialize(CobblemonRaidDensFabric.java:53) ~[cobblemonraiddens-fabric-0.5.1 1.21.1.jar:?]
	at net.fabricmc.loader.impl.FabricLoaderImpl.invokeEntrypoints(FabricLoaderImpl.java:405) [fabric-loader-0.17.2.jar:?]
	at net.fabricmc.loader.impl.game.minecraft.Hooks.startClient(Hooks.java:52) [fabric-loader-0.17.2.jar:?]
	at knot/net.minecraft.class_310.<init>(class_310.java:477) [client-intermediary.jar:?]
	at knot/net.minecraft.client.main.Main.main(Main.java:239) [client-intermediary.jar:?]
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:506) [fabric-loader-0.17.2.jar:?]
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72) [fabric-loader-0.17.2.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23) [fabric-loader-0.17.2.jar:?]
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129) [NewLaunch.jar:?]
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70) [NewLaunch.jar:?]"#;
        let entries: Vec<LogEntry> = LogEntry::from_lines(text.lines());
        let issue = entries
            .iter()
            .filter_map(|e| mixin_apply_failure_entry(e))
            .next()
            .expect("Failed to determine issue");
        assert_eq!(issue, Issue::MixinApplyFailure("fightorflight".to_string()));
    }
}
