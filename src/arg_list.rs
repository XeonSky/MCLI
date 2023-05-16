use std::collections::LinkedList;
use std::path::Path;
use std::env;

pub fn launcher_info_settings(launcher_brand: &String,
                              launcher_version: &String) -> LinkedList::<String> {
    let setting_list = [
        format!("-Dminecraft.launcher.brand={}", launcher_brand),
        format!("-Dminecraft.launcher.version={}", launcher_version)
    ];
    let mut settings = LinkedList::<String>::new();
    for s in setting_list.iter() {
        settings.push_back(s.to_string());
    }
    return settings;
}

pub fn encoding_settings() -> LinkedList::<String> {
    let setting_list = [
        "-Dfile.encoding=UTF-8",
        "-Dsun.stdout.encoding=GB18030",
        "-Dsun.stderr.encoding=GB18030"
    ];
    let mut settings = LinkedList::<String>::new();
    for s in setting_list.iter() {
        settings.push_back(s.to_string());
    }
    return settings;
}

pub fn misc_settings(game_dir: &String) -> LinkedList::<String> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    let setting_list = [
        "-Djava.rmi.server.useCodebaseOnly=true",
        "-Dcom.sun.jndi.rmi.object.trustURLCodebase=false",
        "-Dcom.sun.jndi.cosnaming.object.trustURLCodebase=false",
        "-Dlog4j2.formatMsgNoLookups=true",
        &format!("-Dlog4j.configurationFile={}",
            Path::new(&game_dir.clone()).join("log4j2".to_string()).display()),
        "-Dfml.ignoreInvalidMinecraftCertificates=true",
        "-Dfml.ignorePatchDiscrepancies=true",
        "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdum"
    ];

    let mut settings = LinkedList::<String>::new();
    for s in setting_list.iter() {
        settings.push_back(s.to_string());
    }
    return settings;
}

pub fn memory_settings(memory_min: u32, memory_max: u32) -> LinkedList::<String> {
    let setting_list = [
        format!("-Xms{}m", memory_min),
        format!("-Xmx{}m", memory_max)
    ];
    let mut settings = LinkedList::<String>::new();
    for s in setting_list.iter() {
        settings.push_back(s.to_string());
    }
    return settings;
}

pub fn performance_settings() -> LinkedList::<String> {
    let setting_list = [
        "-XX:+UnlockExperimentalVMOptions",
        "-XX:+UseG1GC",
        "-XX:G1NewSizePercent=20",
        "-XX:G1ReservePercent=20",
        "-XX:MaxGCPauseMillis=50",
        "-XX:G1HeapRegionSize=32m",
        "-XX:-UseAdaptiveSizePolicy",
        "-XX:-OmitStackTraceInFastThrow",
        // "-XX:-DontCompileHugeMethod"
    ];
    let mut settings = LinkedList::<String>::new();
    for s in setting_list.iter() {
        settings.push_back(s.to_string());
    }
    return settings;
}

pub fn game_start_settings(game_version: &String,
                           game_dir: &String,
                           path_libraries: &String) -> LinkedList::<String> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    let native_lib_dir = "natives-windows-x86_64";

    let mut settings = LinkedList::<String>::new();

    // *This contains operation of paths
    let setting_list = [
        format!("-Dminecraft.client.jar={}.jar",
                Path::new(&game_dir.clone()).join(&game_version.clone()).display()),
        format!("-Djava.library.path={}",
                Path::new(&game_dir.clone()).join(native_lib_dir.to_string()).display())
    ];
    for s in setting_list.iter() {
        settings.push_back(s.to_string());
    }

    // Set class path (-cp)
    let mut cp_list = LinkedList::<String>::new();
    // Differs among OSs
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    for s in CP_LIST_WINDOWS_X64.iter() {
        // Make full relative paths
        let full_path = Path::new(&path_libraries.clone()).join(s);
        cp_list.push_back(full_path.display().to_string());
    }
    let path_game_jar = Path::new(&game_dir.clone()).join(format!("{}.jar", game_version));
    cp_list.push_back(path_game_jar.display().to_string());
    // Combine paths by ':' (Unix-like) or ';' Windows
    let cp_list_str = env::join_paths(cp_list.iter());
    settings.push_back("-cp".to_string());
    settings.push_back(cp_list_str.expect("REASON").to_str().expect("REASON").to_string());

    // Main class
    //settings.push_back("net.minecraft.client.main.Main".to_string());
    settings.push_back("net.fabricmc.loader.impl.launch.knot.KnotClient".to_string());

    return settings;
}

pub fn minecraft_settings(game_version: &String,
                      game_dir: &String,
                      assets_dir: &String,
                      launcher_brand: &String,
                      launcher_version: &String,
                      username: &String,
                      display_width: u32,
                      display_height: u32,
                      fullscreen: bool,
                      asset_index: u32) -> LinkedList::<String> {
    let setting_list = [
        "--version", &game_version.clone(),
        "--gameDir", &game_dir.clone(),
        "--assetsDir", &assets_dir.clone(),
        "--assetIndex", &asset_index.to_string(),
        "--versionType", &format!("{} {}", launcher_brand, launcher_version),

        "--username", &username.clone(),
        "--accessToken AABB",
        "--width", &display_width.to_string(),
        "--height", &display_height.to_string(),
    ];
    let mut settings = LinkedList::<String>::new();
    for s in setting_list.iter() {
        settings.push_back(s.to_string());
    }
    if fullscreen {
        settings.push_back("--fullscreen".to_string());
    }
    return settings;
}

pub fn minecraft_settings_server_autoconnect(
        game_version: &String,
        game_dir: &String,
        assets_dir: &String,
        launcher_brand: &String,
        launcher_version: &String,
        username: &String,
        display_width: u32,
        display_height: u32,
        fullscreen: bool,
        asset_index: u32,
        server_address: &String,
        server_port: u16) -> LinkedList::<String> {
    let mut settings = minecraft_settings(game_version,
                                          game_dir,
                                          assets_dir,
                                          launcher_brand,
                                          launcher_version,
                                          username,
                                          display_width,
                                          display_height,
                                          fullscreen,
                                          asset_index);
    settings.push_back("--server".to_string());
    settings.push_back((*server_address).clone());
    settings.push_back("--port".to_string());
    settings.push_back(server_port.to_string());
    return settings;
}

const CP_LIST_WINDOWS_X64: [&str; 75] = [
    "net/fabricmc/tiny-mappings-parser/0.3.0+build.17/tiny-mappings-parser-0.3.0+build.17.jar",
    "net/fabricmc/sponge-mixin/0.12.4+mixin.0.8.5/sponge-mixin-0.12.4+mixin.0.8.5.jar",
    "net/fabricmc/tiny-remapper/0.8.2/tiny-remapper-0.8.2.jar",
    "net/fabricmc/access-widener/2.1.0/access-widener-2.1.0.jar",
    "org/ow2/asm/asm/9.4/asm-9.4.jar",
    "org/ow2/asm/asm-analysis/9.4/asm-analysis-9.4.jar",
    "org/ow2/asm/asm-commons/9.4/asm-commons-9.4.jar",
    "org/ow2/asm/asm-tree/9.4/asm-tree-9.4.jar",
    "org/ow2/asm/asm-util/9.4/asm-util-9.4.jar",
    "net/fabricmc/intermediary/1.19.4/intermediary-1.19.4.jar",
    "net/fabricmc/fabric-loader/0.14.19/fabric-loader-0.14.19.jar",
    "com/github/oshi/oshi-core/6.2.2/oshi-core-6.2.2.jar",
    "com/google/code/gson/gson/2.10/gson-2.10.jar",
    "com/google/guava/failureaccess/1.0.1/failureaccess-1.0.1.jar",
    "com/google/guava/guava/31.1-jre/guava-31.1-jre.jar",
    "com/ibm/icu/icu4j/71.1/icu4j-71.1.jar",
    "com/mojang/authlib/3.18.38/authlib-3.18.38.jar",
    "com/mojang/blocklist/1.0.10/blocklist-1.0.10.jar",
    "com/mojang/brigadier/1.0.18/brigadier-1.0.18.jar",
    "com/mojang/datafixerupper/6.0.6/datafixerupper-6.0.6.jar",
    "com/mojang/logging/1.1.1/logging-1.1.1.jar",
    "com/mojang/patchy/2.2.10/patchy-2.2.10.jar",
    "com/mojang/text2speech/1.16.7/text2speech-1.16.7.jar",
    "commons-codec/commons-codec/1.15/commons-codec-1.15.jar",
    "commons-io/commons-io/2.11.0/commons-io-2.11.0.jar",
    "commons-logging/commons-logging/1.2/commons-logging-1.2.jar",
    "io/netty/netty-buffer/4.1.82.Final/netty-buffer-4.1.82.Final.jar",
    "io/netty/netty-codec/4.1.82.Final/netty-codec-4.1.82.Final.jar",
    "io/netty/netty-common/4.1.82.Final/netty-common-4.1.82.Final.jar",
    "io/netty/netty-handler/4.1.82.Final/netty-handler-4.1.82.Final.jar",
    "io/netty/netty-resolver/4.1.82.Final/netty-resolver-4.1.82.Final.jar",
    "io/netty/netty-transport-classes-epoll/4.1.82.Final/netty-transport-classes-epoll-4.1.82.Final.jar",
    "io/netty/netty-transport-native-unix-common/4.1.82.Final/netty-transport-native-unix-common-4.1.82.Final.jar",
    "io/netty/netty-transport/4.1.82.Final/netty-transport-4.1.82.Final.jar",
    "it/unimi/dsi/fastutil/8.5.9/fastutil-8.5.9.jar",
    "net/java/dev/jna/jna-platform/5.12.1/jna-platform-5.12.1.jar",
    "net/java/dev/jna/jna/5.12.1/jna-5.12.1.jar",
    "net/sf/jopt-simple/jopt-simple/5.0.4/jopt-simple-5.0.4.jar",
    "org/apache/commons/commons-compress/1.21/commons-compress-1.21.jar",
    "org/apache/commons/commons-lang3/3.12.0/commons-lang3-3.12.0.jar",
    "org/apache/httpcomponents/httpclient/4.5.13/httpclient-4.5.13.jar",
    "org/apache/httpcomponents/httpcore/4.4.15/httpcore-4.4.15.jar",
    "org/apache/logging/log4j/log4j-api/2.19.0/log4j-api-2.19.0.jar",
    "org/apache/logging/log4j/log4j-core/2.19.0/log4j-core-2.19.0.jar",
    "org/apache/logging/log4j/log4j-slf4j2-impl/2.19.0/log4j-slf4j2-impl-2.19.0.jar",
    "org/joml/joml/1.10.5/joml-1.10.5.jar",
    "org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1.jar",
    "org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1-natives-windows.jar",
    "org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1-natives-windows-arm64.jar",
    "org/lwjgl/lwjgl-glfw/3.3.1/lwjgl-glfw-3.3.1-natives-windows-x86.jar",
    "org/lwjgl/lwjgl-jemalloc/3.3.1/lwjgl-jemalloc-3.3.1.jar",
    "org/lwjgl/lwjgl-jemalloc/3.3.1/lwjgl-jemalloc-3.3.1-natives-windows.jar",
    "org/lwjgl/lwjgl-jemalloc/3.3.1/lwjgl-jemalloc-3.3.1-natives-windows-arm64.jar",
    "org/lwjgl/lwjgl-jemalloc/3.3.1/lwjgl-jemalloc-3.3.1-natives-windows-x86.jar",
    "org/lwjgl/lwjgl-openal/3.3.1/lwjgl-openal-3.3.1.jar",
    "org/lwjgl/lwjgl-openal/3.3.1/lwjgl-openal-3.3.1-natives-windows.jar",
    "org/lwjgl/lwjgl-openal/3.3.1/lwjgl-openal-3.3.1-natives-windows-arm64.jar",
    "org/lwjgl/lwjgl-openal/3.3.1/lwjgl-openal-3.3.1-natives-windows-x86.jar",
    "org/lwjgl/lwjgl-opengl/3.3.1/lwjgl-opengl-3.3.1.jar",
    "org/lwjgl/lwjgl-opengl/3.3.1/lwjgl-opengl-3.3.1-natives-windows.jar",
    "org/lwjgl/lwjgl-opengl/3.3.1/lwjgl-opengl-3.3.1-natives-windows-arm64.jar",
    "org/lwjgl/lwjgl-opengl/3.3.1/lwjgl-opengl-3.3.1-natives-windows-x86.jar",
    "org/lwjgl/lwjgl-stb/3.3.1/lwjgl-stb-3.3.1.jar",
    "org/lwjgl/lwjgl-stb/3.3.1/lwjgl-stb-3.3.1-natives-windows.jar",
    "org/lwjgl/lwjgl-stb/3.3.1/lwjgl-stb-3.3.1-natives-windows-arm64.jar",
    "org/lwjgl/lwjgl-stb/3.3.1/lwjgl-stb-3.3.1-natives-windows-x86.jar",
    "org/lwjgl/lwjgl-tinyfd/3.3.1/lwjgl-tinyfd-3.3.1.jar",
    "org/lwjgl/lwjgl-tinyfd/3.3.1/lwjgl-tinyfd-3.3.1-natives-windows.jar",
    "org/lwjgl/lwjgl-tinyfd/3.3.1/lwjgl-tinyfd-3.3.1-natives-windows-arm64.jar",
    "org/lwjgl/lwjgl-tinyfd/3.3.1/lwjgl-tinyfd-3.3.1-natives-windows-x86.jar",
    "org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1.jar",
    "org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-windows.jar",
    "org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-windows-arm64.jar",
    "org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-windows-x86.jar",
    "org/slf4j/slf4j-api/2.0.1/slf4j-api-2.0.1.jar",
];

