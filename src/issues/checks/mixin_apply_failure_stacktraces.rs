use crate::{issues::issue::Issue, parse::stacktrace::model::Stacktrace};

pub(crate) fn mixin_apply_failure_stacktraces(stacktraces: &[Stacktrace]) -> Option<Issue> {
    for stacktrace in stacktraces {
        if stacktrace.exception == "org.spongepowered.asm.mixin.throwables.MixinApplyError" {
			let (_, mod_etc) = stacktrace.message.split_once("Mixin [")?;
            let (_, mod_etc) = mod_etc.split_once(" from mod ")?;
            let (mod_normalized, _) = mod_etc.split_once("] ")?;
            return Some(Issue::MixinApplyFailure(mod_normalized.to_string()));
		}
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r#"Caused by: org.spongepowered.asm.mixin.transformer.throwables.MixinTransformerError: An unexpected critical error was encountered
	at org.spongepowered.asm.mixin.transformer.MixinProcessor.applyMixins(MixinProcessor.java:381)
	at org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClass(MixinTransformer.java:237)
	at org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClassBytes(MixinTransformer.java:202)
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.getPostMixinClassByteArray(KnotClassDelegate.java:435)
	... 26 more
Caused by: org.spongepowered.asm.mixin.throwables.MixinApplyError: Mixin [shouldersurfing.fabric.compat.mixins.json:cobblemon.MixinPlayerExtensionsKt from mod shouldersurfing] from phase [DEFAULT] in config [shouldersurfing.fabric.compat.mixins.json] FAILED during APPLY
	at org.spongepowered.asm.mixin.transformer.MixinProcessor.handleMixinError(MixinProcessor.java:686)
	at org.spongepowered.asm.mixin.transformer.MixinProcessor.handleMixinApplyError(MixinProcessor.java:637)
	at org.spongepowered.asm.mixin.transformer.MixinProcessor.applyMixins(MixinProcessor.java:368)
	... 29 more
Caused by: org.spongepowered.asm.mixin.injection.throwables.InvalidInjectionException: Invalid descriptor on shouldersurfing.fabric.compat.mixins.json:cobblemon.MixinPlayerExtensionsKt from mod shouldersurfing->@Inject::traceEntityCollision(Lnet/minecraft/class_1657;FFLjava/lang/Class;Lnet/minecraft/class_1297;Lnet/minecraft/class_3959$class_242;Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfoReturnable;)V! Expected (Lnet/minecraft/class_1309;FFLjava/lang/Class;Lnet/minecraft/class_1297;Lnet/minecraft/class_3959$class_242;Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfoReturnable;)V but found (Lnet/minecraft/class_1657;FFLjava/lang/Class;Lnet/minecraft/class_1297;Lnet/minecraft/class_3959$class_242;Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfoReturnable;)V [INJECT_APPLY Applicator Phase -> shouldersurfing.fabric.compat.mixins.json:cobblemon.MixinPlayerExtensionsKt from mod shouldersurfing -> Apply Injections ->  -> Inject -> shouldersurfing.fabric.compat.mixins.json:cobblemon.MixinPlayerExtensionsKt from mod shouldersurfing->@Inject::traceEntityCollision(Lnet/minecraft/class_1657;FFLjava/lang/Class;Lnet/minecraft/class_1297;Lnet/minecraft/class_3959$class_242;Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfoReturnable;)V]"#;
        let stacktraces: Vec<Stacktrace> = Stacktrace::from_lines(text.lines()).collect();
        let issue = mixin_apply_failure_stacktraces(&stacktraces).expect("Failed to detect issue");
        assert_eq!(issue, Issue::MixinApplyFailure("shouldersurfing".to_string()));
    }
}
