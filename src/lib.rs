use std::fs;

use zed_extension_api::{
    self as zed, current_platform, download_file, latest_github_release,
    set_language_server_installation_status, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result, Worktree,
};

const SERVER_DIR: &str = "lemminx-maven-server";
const LEMMINX_JAR: &str = "org.eclipse.lemminx-uber.jar";
const MAVEN_EXT_DIR: &str = "maven-ext";
const MAVEN_EXT_JAR_HINT: &str = "lemminx-maven";

const GITHUB_REPO: &str = "weirdo-adam/zed-spring-boot";

struct SpringBootExtension {
    cached_server_path: Option<String>,
}

impl zed::Extension for SpringBootExtension {
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let java = find_java(worktree)?;
        let base = self.server_base_dir(language_server_id)?;
        let path_sep = match current_platform().0 {
            Os::Windows => ";",
            _ => ":",
        };
        Ok(zed::Command {
            command: java,
            args: vec![
                "-Xmx512m".into(),
                "-cp".into(),
                format!("{base}/{LEMMINX_JAR}{path_sep}{base}/{MAVEN_EXT_DIR}/*"),
                "org.eclipse.lemminx.XMLServerLauncher".into(),
            ],
            env: vec![],
        })
    }
}

impl SpringBootExtension {
    /// Returns the directory that contains `org.eclipse.lemminx-uber.jar`
    /// and the `maven-ext/` dependency directory.
    ///
    /// Resolution order:
    /// 1. `ZED_LEMMINX_HOME` environment variable (local development)
    /// 2. previously downloaded copy under the extension's data dir
    /// 3. download from this repository's GitHub releases
    fn server_base_dir(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_server_path {
            if server_files_present(path) {
                return Ok(path.clone());
            }
        }

        // 1. Local override (development / self-managed installs)
        if let Ok(home) = std::env::var("ZED_LEMMINX_HOME") {
            if server_files_present(&home) {
                self.cached_server_path = Some(home.clone());
                return Ok(home);
            }
            return Err(format!(
                "ZED_LEMMINX_HOME is set to {home}, but it does not contain \
                 {LEMMINX_JAR} and {MAVEN_EXT_DIR}/"
            ));
        }

        // 2/3. Managed install under the extension's working directory
        let managed = SERVER_DIR.to_string();
        if !server_files_present(&managed) {
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::CheckingForUpdate,
            );
            let release = latest_github_release(
                GITHUB_REPO,
                GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )
            .map_err(|e| {
                format!(
                    "failed to look up LemMinX release at {GITHUB_REPO}: {e}; \
                     you can also set ZED_LEMMINX_HOME to a directory with \
                     {LEMMINX_JAR} + {MAVEN_EXT_DIR}/ (see README)"
                )
            })?;

            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );

            // A previous buggy download (pre-0.0.2) wrote a FILE named
            // SERVER_DIR; clean it up so the directory can be created.
            if fs::metadata(SERVER_DIR).map_or(false, |s| !s.is_dir()) {
                let _ = fs::remove_file(SERVER_DIR);
            }

            let uber = release
                .assets
                .iter()
                .find(|a| a.name == "lemminx-uber.jar")
                .ok_or("release is missing the `lemminx-uber.jar` asset")?;
            // Uncompressed: `file_path` is the destination file path.
            download_file(
                &uber.download_url,
                &format!("{SERVER_DIR}/{LEMMINX_JAR}"),
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download LemMinX: {e}"))?;

            let maven = release
                .assets
                .iter()
                .find(|a| a.name == "lemminx-maven-deps.zip")
                .ok_or("release is missing the `lemminx-maven-deps.zip` asset")?;
            // Zip: `file_path` is the directory the archive is extracted into.
            // The zip holds lemminx-maven + its dependency jars at its root.
            download_file(
                &maven.download_url,
                &format!("{SERVER_DIR}/{MAVEN_EXT_DIR}"),
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download lemminx-maven: {e}"))?
        }

        if !server_files_present(&managed) {
            return Err(format!(
                "server files incomplete under {managed}: need {LEMMINX_JAR} and \
                 {MAVEN_EXT_DIR}/ containing {MAVEN_EXT_JAR_HINT}*.jar"
            ));
        }
        self.cached_server_path = Some(managed.clone());
        Ok(managed)
    }
}

fn server_files_present(base: &str) -> bool {
    fs::metadata(format!("{base}/{LEMMINX_JAR}")).map_or(false, |s| s.is_file())
        && fs::metadata(format!("{base}/{MAVEN_EXT_DIR}")).map_or(false, |s| s.is_dir())
        && jars_in(&format!("{base}/{MAVEN_EXT_DIR}"))
}

fn jars_in(dir: &str) -> bool {
    match fs::read_dir(dir) {
        Ok(entries) => entries.flatten().any(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.ends_with(".jar")
        }),
        Err(_) => false,
    }
}

/// Locate a JVM for running the language servers.
///
/// Resolution order:
/// 1. `worktree.which("java")` — PATH visible to the worktree
/// 2. `ZED_JAVA_HOME` environment variable
/// 3. `JAVA_HOME` of the Zed process
/// 4. well-known Homebrew / system JVM locations
fn find_java(worktree: &Worktree) -> Result<String> {
    if let Some(path) = worktree.which("java") {
        return Ok(path);
    }
    if let Ok(home) = std::env::var("ZED_JAVA_HOME") {
        let candidate = format!("{home}/bin/java");
        if fs::metadata(&candidate).map_or(false, |s| s.is_file()) {
            return Ok(candidate);
        }
    }
    if let Ok(home) = std::env::var("JAVA_HOME") {
        let candidate = format!("{home}/bin/java");
        if fs::metadata(&candidate).map_or(false, |s| s.is_file()) {
            return Ok(candidate);
        }
    }

    let candidates: Vec<String> = match current_platform().0 {
        Os::Mac => {
            let mut v = vec![
                "/opt/homebrew/opt/openjdk@21/bin/java".into(),
                "/opt/homebrew/opt/openjdk@17/bin/java".into(),
                "/opt/homebrew/opt/openjdk/bin/java".into(),
                "/usr/local/opt/openjdk@21/bin/java".into(),
                "/usr/local/opt/openjdk@17/bin/java".into(),
                "/usr/local/opt/openjdk/bin/java".into(),
            ];
            v.extend(jvms_from("/Library/Java/JavaVirtualMachines"));
            v
        }
        _ => {
            let mut v = vec![
                "/usr/lib/jvm/java-21-openjdk-amd64/bin/java".into(),
                "/usr/lib/jvm/java-17-openjdk-amd64/bin/java".into(),
            ];
            v.extend(jvms_from("/usr/lib/jvm"));
            v
        }
    };

    candidates
        .into_iter()
        .find(|p| fs::metadata(p).map_or(false, |s| s.is_file()))
        .ok_or_else(|| {
            "no JVM found: install a JDK 17+ (e.g. `brew install openjdk@21`), \
             launch Zed from a shell with `java` on PATH, or set JAVA_HOME / ZED_JAVA_HOME"
                .to_string()
        })
}

fn jvms_from(root: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let home = format!("{root}/{name}/Contents/Home/bin/java");
            if fs::metadata(&home).map_or(false, |s| s.is_file()) {
                out.push(home);
            }
        }
    }
    out
}

zed::register_extension!(SpringBootExtension);
