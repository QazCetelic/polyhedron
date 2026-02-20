use std::str::Lines;

#[derive(Debug)]
pub struct StacktraceParser {
    exception: Option<String>,
    message: Option<String>,
    lines: Vec<StacktraceLine>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct StacktraceLine {
    pub class: String,
    pub method: String,
    pub source: String,
    pub rest: String,
}

impl StacktraceLine {
    #[allow(dead_code)]
    pub fn get_jar(&self) -> Option<String> {
        // "~[lwjgl-2.9.4-nightly-20150209.jar:?]"
        let without_brackets = self.rest.strip_suffix(']')?.trim_start_matches('~').strip_prefix('[')?;
        // "lwjgl-2.9.4-nightly-20150209.jar:?"
        let (source, _) = without_brackets.split_once(':')?;
        if source.ends_with(".jar") {
            Some(source.to_string())
        }
        else {
            None
        }
    }

    /// Gets the relative file path of the source code and the line
    pub fn get_relative_path(&self) -> Option<(String, usize)> {
        // "EntryPoint.java:70"
        let (class_name, line) = self.source.split_once(".java:")?;
        let line: usize = line.parse().ok()?;
        if !valid_java_identifier(class_name) || !self.class.ends_with(class_name) {
            return None;
        }
        let mut path = String::with_capacity(self.class.len());
        let mut iter = self.class.split('.').peekable();
        while let Some(part) = iter.next() {
            path.push_str(part);
            if iter.peek().is_some() {
                path.push('/');
            }
        }
        path.push_str(".java");
        Some((path, line))
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dioxius", derive(Clone, PartialEq))]
pub struct Stacktrace {
    pub exception: String,
    pub message: String,
    pub lines: Vec<StacktraceLine>,
}

impl Stacktrace {
    pub fn from_lines(lines: Lines) -> impl Iterator<Item = Stacktrace> {
        let mut parser = StacktraceParser::new();
        // Throws in extra line, to indirectly call finalize
        lines.chain(vec![""]).filter_map(move |line| parser.parse_line(line))
    }
}

// "java.lang.Throwable" -> true, "ERROR" -> false
fn valid_java_identifier(identifier: &str) -> bool {
    let mut chars = identifier.chars();
    let Some(first_char) = chars.next() else {
        return false;
    };
    if !first_char.is_ascii_alphabetic() { // First char must be /[a-zA-Z]/
        return false;
    }
    for c in chars {
        if c != '$' && c != '_' && !c.is_ascii_alphabetic() {
            return false;
        }
    }
    return true;
}

// "java.lang.Throwable" -> true
pub(crate) fn is_valid_classname(classname: &str) -> bool {
    let classname_parts = classname.split('.');
    let mut part_cnt = 0_usize;
    for part in classname_parts {
        part_cnt += 1;
        if !valid_java_identifier(part) {
            return false;
        }
    }
    if part_cnt < 2 {
        return false;
    }
    return true;
}

// "OpenGL debug message: java.lang.Throwable: id=0, source=SHADER…" -> {"java.lang.Throwable", "id=0, source=SHADER…"}
fn parse_exception<'a>(line: &'a str) -> Option<(&'a str, &'a str)> {
    let (exception, msg) = line.split_once(": ")?;
    if is_valid_classname(exception) {
        Some((exception, msg))
    }
    else {
        parse_exception(msg)
    }
}

#[allow(dead_code)]
impl StacktraceParser {
    pub fn new() -> Self {
        StacktraceParser { 
            exception: None,
            message: None,
            lines: Vec::new()
        }
    }

    fn parse_trace_line(&mut self, line: &str) -> Option<StacktraceLine> {
        const FOUR_SPACES: &str = "    ";
        let without_space = line.strip_prefix("\t").or(line.strip_prefix(FOUR_SPACES))?;
        let without_at = without_space.strip_prefix("at ")?;
        let (class_and_method, source_and_rest) = without_at.split_once('(')?;
        let (class, method) = class_and_method.rsplit_once('.')?;
        let (source, rest) = source_and_rest.split_once(')')?;
        let rest = rest.trim_ascii_start();
        Some(StacktraceLine {
            class: class.to_string(),
            method: method.to_string(),
            source: source.to_string(),
            rest: rest.to_string(),
        })
    }

    pub fn parse_line(&mut self, line: &str) -> Option<Stacktrace> {
        if let Some((exception, message)) = parse_exception(line.strip_prefix("Caused by: ").unwrap_or(line)) { // "org.lwjgl.LWJGLException: Could not choose GLX13 config"
            let trace_opt = self.finalize();
            self.exception = Some(exception.to_string());
            self.message = Some(message.to_string());
            trace_opt
        }
        else {
            let indent = line.starts_with('\t') || line.starts_with("    ");
            let traces_started = !self.lines.is_empty();
            match (indent, traces_started, &mut self.message) {
                (true, _, Some(_)) => {
                    if let Some(trace) = self.parse_trace_line(line) { // "   at org..."
                        self.lines.push(trace);
                        None
                    }
                    else { // Failed to parse line
                        self.finalize()
                    }
                },
                (false, false, Some(msg)) => {
                    msg.push('\n');
                    msg.push_str(line);
                    None
                },
                (_, _, _) => self.finalize(),
            }
        }
    }

    pub fn finalize(&mut self) -> Option<Stacktrace> {
        if self.lines.is_empty() {
            self.exception = None;
            self.message = None;
            None
        }
        else {
            let exception = self.exception.take();
            let message = self.message.take();
            let stripped_msg = message.map(|s| if let Some(msg) = s.strip_suffix("Stacktrace:") { msg.trim_ascii_end().to_string() } else { s } );
            let lines = std::mem::take(&mut self.lines);
            let trace = Stacktrace {
                exception: exception?,
                message: stripped_msg?,
                lines,
            };
            self.lines = Vec::new();
            Some(trace)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_exception_prefix() {
        let text = "OpenGL debug message: java.lang.Throwable: id=0, source=SHADER…";
        let (exception, msg) = parse_exception(text).expect("Failed to parse exception");
        assert_eq!(exception, "java.lang.Throwable");
        assert_eq!(msg, "id=0, source=SHADER…");
    }

    #[test]
    fn parse_trace_line() {
        let line = "\tat org.lwjgl.opengl.LinuxDisplayPeerInfo.initDefaultPeerInfo(Native Method)";
        let mut parser = StacktraceParser::new();
        let trace_line = parser.parse_trace_line(line).expect("Failed to parse");
        assert_eq!(trace_line.class, "org.lwjgl.opengl.LinuxDisplayPeerInfo");
        assert_eq!(trace_line.method, "initDefaultPeerInfo");
        assert_eq!(trace_line.source, "Native Method");
        assert_eq!(trace_line.rest, "");
    }

    #[test]
    fn parse_trace_line_with_jar() {
        let line = "\tat org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105) [NewLaunch.jar:?]";
        let mut parser = StacktraceParser::new();
        let trace_line = parser.parse_trace_line(line).expect("Failed to parse");
        assert_eq!(trace_line.rest, "[NewLaunch.jar:?]");
        assert_eq!(trace_line.get_jar().expect("Failed to get jar"), "NewLaunch.jar");
    }

    #[test]
    fn parse_trace_line_with_class() {
        let line = "\tat net.minecraftforge.client.ForgeHooksClient.createDisplay(ForgeHooksClient.java:327) ~[ForgeHooksClient.class:?]";
        let mut parser = StacktraceParser::new();
        let trace_line = parser.parse_trace_line(line).expect("Failed to parse");
        assert_eq!(trace_line.class, "net.minecraftforge.client.ForgeHooksClient");
    }

    #[test]
    fn parse_crash_report() {
        let crash_report = "---- Minecraft Crash Report ----
// Would you like a cupcake?

Time: 2025-09-16 22:58:36 CEST
Description: Initializing game

org.lwjgl.LWJGLException: Could not choose GLX13 config
\tat org.lwjgl.opengl.LinuxDisplayPeerInfo.initDefaultPeerInfo(Native Method)
\tat org.lwjgl.opengl.LinuxDisplayPeerInfo.<init>(LinuxDisplayPeerInfo.java:61)
\tat org.lwjgl.opengl.LinuxDisplay.createPeerInfo(LinuxDisplay.java:828)
\tat org.lwjgl.opengl.DrawableGL.setPixelFormat(DrawableGL.java:61)
\tat org.lwjgl.opengl.Display.create(Display.java:846)
\tat org.lwjgl.opengl.Display.create(Display.java:757)
\tat org.lwjgl.opengl.Display.create(Display.java:739)
\tat net.minecraft.client.Minecraft.func_71384_a(Minecraft.java:452)
\tat net.minecraft.client.Minecraft.func_99999_d(Minecraft.java:7099)
\tat net.minecraft.client.main.Main.main(SourceFile:148)
\tat sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
\tat sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)
\tat sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
\tat java.lang.reflect.Method.invoke(Method.java:498)
\tat net.minecraft.launchwrapper.Launch.launch(Launch.java:135)
\tat net.minecraft.launchwrapper.Launch.main(Launch.java:28)
\tat org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
\tat org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
\tat org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
    ";
        let lines = crash_report.lines();
        let mut traces = Stacktrace::from_lines(lines);
        let trace = traces.next().expect("Failed to get trace");
        assert!(traces.next().is_none());

        assert_eq!(trace.exception, "org.lwjgl.LWJGLException");
        assert_eq!(trace.message, "Could not choose GLX13 config");
        assert_eq!(trace.lines.len(), 19);
        let first_trace_line = trace.lines.first().expect("No trace lines");
        assert_eq!(first_trace_line.method, "initDefaultPeerInfo");
        let last_trace_line = trace.lines.last().expect("No trace lines");
        assert_eq!(last_trace_line.source, "EntryPoint.java:70");
    }

    #[test]
    fn parse_multiline_message() {
        let crash_report = "OpenGL debug message: java.lang.Throwable: id=0, source=SHADER COMPILER, type=ERROR, severity=HIGH, message='SHADER_ID_COMPILE error has been generated. GLSL compile failed for shader 1, \"\": ERROR: 0:3: '#extension' :  'GL_ARB_gpu_shader_int64' is not supported
ERROR: 0:5: 'uint64_t' : undeclared identifier 
ERROR: 0:5: 'a' : syntax error syntax error

'
java.lang.Throwable: id=0, source=SHADER COMPILER, type=ERROR, severity=HIGH, message='SHADER_ID_COMPILE error has been generated. GLSL compile failed for shader 1, \"\": ERROR: 0:3: '#extension' :  'GL_ARB_gpu_shader_int64' is not supported
ERROR: 0:5: 'uint64_t' : undeclared identifier 
ERROR: 0:5: 'a' : syntax error syntax error

'
\tat knot//net.minecraft.class_1008.wrapOperation$bol000$voxy$wrapDebug(class_1008.java:519)
\tat knot//net.minecraft.class_1008.method_4224(class_1008.java:105)
\tat knot//org.lwjgl.opengl.GLDebugMessageCallbackI.callback(GLDebugMessageCallbackI.java:46)
\tat knot//org.lwjgl.opengl.GL20C.glCompileShader(Native Method)
\tat knot//me.cortex.voxy.client.core.gl.Capabilities.testShaderCompilesOk(Capabilities.java:202)
\tat knot//me.cortex.voxy.client.core.gl.Capabilities.<init>(Capabilities.java:62)
\tat knot//me.cortex.voxy.client.core.gl.Capabilities.<clinit>(Capabilities.java:32)
\tat knot//me.cortex.voxy.client.VoxyClient.initVoxyClient(VoxyClient.java:31)
\tat knot//com.mojang.blaze3d.systems.RenderSystem.handler$boo000$voxy$injectInit(RenderSystem.java:1522)
\tat knot//com.mojang.blaze3d.systems.RenderSystem.initRenderer(RenderSystem.java:209)
\tat knot//net.minecraft.class_310.<init>(class_310.java:533)
\tat knot//net.minecraft.client.main.Main.main(Main.java:234)
\tat net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:514)
\tat net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72)
\tat net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23)
\tat org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:102)
\tat org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
\tat org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
    ";
        let lines = crash_report.lines();
        let mut traces = Stacktrace::from_lines(lines);
        let trace = traces.next().expect("Failed to get trace");
        assert_eq!(trace.exception, "java.lang.Throwable");
        assert!(trace.message.starts_with("id=0, source=SHADER COMPILER, type=ERROR, severity=HIGH, message='SHADER_ID_COMPILE error has been generated."));
        assert!(trace.message.ends_with("ERROR: 0:5: 'a' : syntax error syntax error\n\n'"));
    }

    #[test]
    fn parse_caused_by() {
        let crash_report = "java.lang.RuntimeException: Could not execute entrypoint stage 'client' due to errors, provided by 'betteradvancements' at 'betteradvancements.fabric.BetterAdvancements'!
	at net.fabricmc.loader.impl.FabricLoaderImpl.lambda$invokeEntrypoints$0(FabricLoaderImpl.java:409)
	at net.fabricmc.loader.impl.util.ExceptionUtil.gatherExceptions(ExceptionUtil.java:33)
	at net.fabricmc.loader.impl.FabricLoaderImpl.invokeEntrypoints(FabricLoaderImpl.java:407)
	at net.fabricmc.loader.impl.game.minecraft.Hooks.startClient(Hooks.java:53)
	at knot//net.minecraft.class_310.<init>(class_310.java:475)
	at knot//net.minecraft.client.main.Main.main(Main.java:234)
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:514)
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72)
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23)
	at org.prismlauncher.launcher.impl.StandardLauncher.launch(StandardLauncher.java:105)
	at org.prismlauncher.EntryPoint.listen(EntryPoint.java:129)
	at org.prismlauncher.EntryPoint.main(EntryPoint.java:70)
Caused by: java.lang.IncompatibleClassChangeError: class betteradvancements.common.gui.BetterAdvancementsScreenButton overrides final method net.minecraft.class_4264.method_48579(Lnet/minecraft/class_332;IIF)V
	at java.base/java.lang.ClassLoader.defineClass1(Native Method)
	at java.base/java.lang.ClassLoader.defineClass(ClassLoader.java:1027)
	at java.base/java.security.SecureClassLoader.defineClass(SecureClassLoader.java:150)
	at net.fabricmc.loader.impl.launch.knot.KnotClassLoader.defineClassFwd(KnotClassLoader.java:165)
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.tryLoadClass(KnotClassDelegate.java:368)
	at net.fabricmc.loader.impl.launch.knot.KnotClassDelegate.loadClass(KnotClassDelegate.java:231)
	at net.fabricmc.loader.impl.launch.knot.KnotClassLoader.loadClass(KnotClassLoader.java:119)
	at java.base/java.lang.ClassLoader.loadClass(ClassLoader.java:526)
	at knot//betteradvancements.fabric.config.ConfigFileHandler.readFromConfig(ConfigFileHandler.java:70)
	at knot//betteradvancements.fabric.BetterAdvancements.onInitializeClient(BetterAdvancements.java:12)
	at net.fabricmc.loader.impl.FabricLoaderImpl.invokeEntrypoints(FabricLoaderImpl.java:405)
	... 9 more";
        let lines = crash_report.lines();
        let mut traces = Stacktrace::from_lines(lines);
        
        let trace_1 = traces.next().expect("Failed to get trace");
        assert_eq!(trace_1.exception, "java.lang.RuntimeException");
        assert_eq!(trace_1.message, "Could not execute entrypoint stage 'client' due to errors, provided by 'betteradvancements' at 'betteradvancements.fabric.BetterAdvancements'!");
        assert_eq!(trace_1.lines.len(), 12);
        assert_eq!(trace_1.lines[0].method, "lambda$invokeEntrypoints$0");
        assert_eq!(trace_1.lines[5].method, "main");
        assert_eq!(trace_1.lines[7].source, "Knot.java:72");
        assert_eq!(trace_1.lines[11].class, "org.prismlauncher.EntryPoint");
        let file_path_1 = trace_1.lines[11].get_relative_path().unwrap();
        assert_eq!(file_path_1.0, "org/prismlauncher/EntryPoint.java");
        assert_eq!(file_path_1.1, 70);
        
        let trace_2 = traces.next().expect("Failed to get trace");
        assert_eq!(trace_2.exception, "java.lang.IncompatibleClassChangeError");
        assert_eq!(trace_2.message, "class betteradvancements.common.gui.BetterAdvancementsScreenButton overrides final method net.minecraft.class_4264.method_48579(Lnet/minecraft/class_332;IIF)V");
        assert_eq!(trace_2.lines.len(), 11);
        assert_eq!(trace_2.lines[0].method, "defineClass1");
        assert_eq!(trace_2.lines[5].method, "loadClass");
        assert_eq!(trace_2.lines[8].source, "ConfigFileHandler.java:70");
        assert_eq!(trace_2.lines[10].class, "net.fabricmc.loader.impl.FabricLoaderImpl");
    }

    #[test]
    fn strips_message() {
        let text = "
        	Mod file: /Users/********/Library/Application Support/PrismLauncher/instances/1.21.1(1)/minecraft/mods/create-1.21.1-6.0.9.jar
	Failure message: Create (create) has failed to load correctly
		org.spongepowered.asm.mixin.transformer.throwables.MixinTransformerError: An unexpected critical error was encountered
	Mod version: 6.0.9
	Mod issues URL: https://github.com/Creators-of-Create/Create/issues
	Exception message: org.spongepowered.asm.mixin.transformer.throwables.InvalidMixinException: @Shadow field simplifiedEntityColliders was not located in the target class com.simibubi.create.content.contraptions.Contraption. No refMap loaded.
Stacktrace:
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinPreProcessorStandard.attachFields(MixinPreProcessorStandard.java:624) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinPreProcessorStandard.attach(MixinPreProcessorStandard.java:302) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinPreProcessorStandard.createContextFor(MixinPreProcessorStandard.java:277) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinInfo.createContextFor(MixinInfo.java:1288) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinApplicatorStandard.apply(MixinApplicatorStandard.java:203) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.TargetClassContext.apply(TargetClassContext.java:437) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.TargetClassContext.applyMixins(TargetClassContext.java:418) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinProcessor.applyMixins(MixinProcessor.java:363) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.mixin.transformer.MixinTransformer.transformClass(MixinTransformer.java:250) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar%23161!/:0.15.2+mixin.0.8.7] {}
	at TRANSFORMER/libjf_unsafe_v0@3.17.4+forge/io.gitlab.jfronny.libjf.unsafe.asm.AsmTransformer.transformClass(AsmTransformer.java:127) ~[libjf-unsafe-v0-3.17.4+forge.jar%23604!/:?] {re:classloading}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.service.modlauncher.MixinTransformationHandler.processClass(MixinTransformationHandler.java:131) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/org.spongepowered.mixin/org.spongepowered.asm.launch.MixinLaunchPluginLegacy.processClass(MixinLaunchPluginLegacy.java:131) ~[sponge-mixin-0.15.2+mixin.0.8.7.jar:0.15.2+mixin.0.8.7] {}
	at MC-BOOTSTRAP/cpw.mods.modlauncher@11.0.5/cpw.mods.modlauncher.serviceapi.ILaunchPluginService.processClassWithFlags(ILaunchPluginService.java:156) ~[modlauncher-11.0.5.jar:11.0.5+main.901c6ea8] {}
	at MC-BOOTSTRAP/cpw.mods.modlauncher@11.0.5/cpw.mods.modlauncher.LaunchPluginHandler.offerClassNodeToPlugins(LaunchPluginHandler.java:94) ~[modlauncher-11.0.5.jar:?] {}
";
        let lines = text.lines();
        let mut traces = Stacktrace::from_lines(lines);
        let trace = traces.next().expect("Failed to get trace");
        assert_eq!(trace.message, "@Shadow field simplifiedEntityColliders was not located in the target class com.simibubi.create.content.contraptions.Contraption. No refMap loaded.")
    }
}