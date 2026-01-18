use crate::{issues::issue::Issue, parse::stacktrace::Stacktrace};

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CriticalInjectionFailure {
    pub method: String,
    pub mixin: String,
    pub normalized_mod_name: String,
}

pub(crate) fn critical_injection_failure(stacktrace: &Stacktrace) -> Option<Issue> {
    if stacktrace.exception != "org.spongepowered.asm.mixin.injection.throwables.InjectionError" {
        return None;
    }
    let msg = stacktrace.message.strip_prefix("Critical injection failure: ")?;
    let (method_etc, mod_etc) = msg.split_once(" from mod ")?;
    let (method, mixin) = method_etc.split_once(" in ")?;
    let (mod_name, rest) = mod_etc.split_once(" failed injection check, (")?;
    let (_, rest) = rest.split_once(") succeeded. Scanned ")?;
    let (_, rest) = rest.split_once(" target(s). ")?;
    if !rest.starts_with("No refMap loaded") && !rest.starts_with("Using refmap ") {
        return None;
    }

    Some(Issue::CriticalInjectionFailure(Box::new(CriticalInjectionFailure {
        method: method.to_string(),
        mixin: mixin.to_string(),
        normalized_mod_name: mod_name.to_string(),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let text = r#"
Caused by: org.spongepowered.asm.mixin.injection.throwables.InjectionError: Critical injection failure: Constant modifier method onFluidFallModifier(F)F in mixins.iguanatweaksreborn.json:EntityMixin from mod iguanatweaksreborn failed injection check, (0/1) succeeded. Scanned 1 target(s). Using refmap mixins.iguanatweaksreborn.refmap.json
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.injection.struct.InjectionInfo.postInject(InjectionInfo.java:468)
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinTargetContext.applyInjections(MixinTargetContext.java:1384)
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.applyInjections(MixinApplicatorStandard.java:1062)
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.applyMixin(MixinApplicatorStandard.java:402)
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.apply(MixinApplicatorStandard.java:327)
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.TargetClassContext.apply(TargetClassContext.java:421)
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.TargetClassContext.applyMixins(TargetClassContext.java:403)
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinProcessor.applyMixins(MixinProcessor.java:363)
	... 58 more
"#;
        let stacktrace = Stacktrace::from_lines(text.lines()).next().expect("Failed to parse stacktrace");
        let issue = critical_injection_failure(&stacktrace).expect("Failed to get issue");
        let Issue::CriticalInjectionFailure(injection_failure) = issue else { panic!("Not the right issue"); };
        assert_eq!(injection_failure.method, "Constant modifier method onFluidFallModifier(F)F");
        assert_eq!(injection_failure.mixin, "mixins.iguanatweaksreborn.json:EntityMixin");
        assert_eq!(injection_failure.normalized_mod_name, "iguanatweaksreborn");
    }

    #[test]
    fn test_2() {
        let text = r#"
Caused by: org.spongepowered.asm.mixin.injection.throwables.InjectionError: Critical injection failure: Callback method onMovementInputUpdate(Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfo;)V in moonlight.mixins.json:LocalPlayerMixin from mod moonlight failed injection check, (0/1) succeeded. Scanned 0 target(s). Using refmap moonlight-fabric-refmap.json
	at org.spongepowered.asm.mixin.injection.struct.InjectionInfo.postInject(InjectionInfo.java:531) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinTargetContext.applyInjections(MixinTargetContext.java:1490) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.applyInjections(MixinApplicatorStandard.java:752) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.applyMixin(MixinApplicatorStandard.java:330) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.apply(MixinApplicatorStandard.java:246) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.TargetClassContext.apply(TargetClassContext.java:437) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.TargetClassContext.applyMixins(TargetClassContext.java:418) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinProcessor.applyMixins(MixinProcessor.java:352) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClass(MixinTransformer.java:237) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClassBytes(MixinTransformer.java:202) ~[sponge-mixin-0.16.5 mixin.0.8.7.jar:0.16.5 mixin.0.8.7]
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.getPostMixinClassByteArray(KnotClassDelegate.java:435) ~[fabric-loader-0.18.1.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.tryLoadClass(KnotClassDelegate.java:336) ~[fabric-loader-0.18.1.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.loadClass(KnotClassDelegate.java:231) ~[fabric-loader-0.18.1.jar:?]
	at net.fabricmc.loader.impl.launch.knot.KnotClassLoader.loadClass(KnotClassLoader.java:119) ~[fabric-loader-0.18.1.jar:?]
	at java.base/java.lang.ClassLoader.loadClass(ClassLoader.java:526) ~[?:?]
	at knot/net.minecraft.client.main.Main.main(Main.java:221) ~[minecraft-1.21.4-client.jar:?]
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:514) ~[fabric-loader-0.18.1.jar:?]
	... 5 more
"#;
        let stacktrace = Stacktrace::from_lines(text.lines()).next().expect("Failed to parse stacktrace");
        let issue = critical_injection_failure(&stacktrace).expect("Failed to get issue");
        let Issue::CriticalInjectionFailure(injection_failure) = issue else { panic!("Not the right issue"); };
        assert_eq!(injection_failure.method, "Callback method onMovementInputUpdate(Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfo;)V");
        assert_eq!(injection_failure.mixin, "moonlight.mixins.json:LocalPlayerMixin");
        assert_eq!(injection_failure.normalized_mod_name, "moonlight");
    }

    #[test]
    fn test_3() {
        let text = r#"
Caused by: org.spongepowered.asm.mixin.injection.throwables.InjectionError: Critical injection failure: Redirector cullParticles(Lnet/minecraft/client/particle/Particle;Lcom/mojang/blaze3d/vertex/VertexConsumer;Lnet/minecraft/client/Camera;F)V in cullparticles.mixins.json:MixinParticleEngine from mod cullparticles failed injection check, (0/1) succeeded. Scanned 0 target(s). No refMap loaded.
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.injection.struct.InjectionInfo.postInject(InjectionInfo.java:531) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinTargetContext.applyInjections(MixinTargetContext.java:1490) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.applyInjections(MixinApplicatorStandard.java:752) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.applyMixin(MixinApplicatorStandard.java:330) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.apply(MixinApplicatorStandard.java:246) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.TargetClassContext.apply(TargetClassContext.java:437) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.TargetClassContext.applyMixins(TargetClassContext.java:418) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinProcessor.applyMixins(MixinProcessor.java:363) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	... 30 more
"#;
        let stacktrace = Stacktrace::from_lines(text.lines()).next().expect("Failed to parse stacktrace");
        let issue = critical_injection_failure(&stacktrace).expect("Failed to get issue");
        let Issue::CriticalInjectionFailure(injection_failure) = issue else { panic!("Not the right issue"); };
        assert_eq!(injection_failure.method, "Redirector cullParticles(Lnet/minecraft/client/particle/Particle;Lcom/mojang/blaze3d/vertex/VertexConsumer;Lnet/minecraft/client/Camera;F)V");
        assert_eq!(injection_failure.mixin, "cullparticles.mixins.json:MixinParticleEngine");
        assert_eq!(injection_failure.normalized_mod_name, "cullparticles");
    }
}