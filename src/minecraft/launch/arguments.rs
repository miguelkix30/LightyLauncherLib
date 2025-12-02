use crate::minecraft::version::version_metadata::VersionBuilder;
use crate::minecraft::version::version::Version;
use std::borrow::Cow;
use std::collections::HashMap;

pub trait Arguments<'a> {
    fn build_arguments(
        &self,
        builder: &VersionBuilder,
        username: &str,
        uuid: &str,
    ) -> Vec<String>;
}

impl<'a> Arguments<'a> for Version<'a> {
    fn build_arguments(
        &self,
        builder: &VersionBuilder,
        username: &str,
        uuid: &str,
    ) -> Vec<String> {
        // Créer la HashMap avec toutes les variables
        let variables = self.create_variable_map(builder, username, uuid);

        // Remplacer les variables dans les arguments
        let game_args = replace_variables_in_vec(&builder.arguments.game, &variables);

        let mut jvm_args = builder.arguments.jvm
            .as_ref()
            .map(|jvm| replace_variables_in_vec(jvm, &variables))
            .unwrap_or_else(|| self.build_default_jvm_args(&variables));

        // S'assurer que les arguments JVM critiques sont toujours présents

        // 1. java.library.path (pour les natives LWJGL)
        if !jvm_args.iter().any(|arg| arg.starts_with("-Djava.library.path=")) {
            let natives_dir = variables.get("natives_directory").cloned().unwrap_or_default();
            jvm_args.insert(0, format!("-Djava.library.path={}", natives_dir));
        }

        // 2. Launcher brand et version
        if !jvm_args.iter().any(|arg| arg.starts_with("-Dminecraft.launcher.brand=")) {
            let launcher_name = variables.get("launcher_name").cloned().unwrap_or_default();
            jvm_args.insert(0, format!("-Dminecraft.launcher.brand={}", launcher_name));
        }

        if !jvm_args.iter().any(|arg| arg.starts_with("-Dminecraft.launcher.version=")) {
            let launcher_version = variables.get("launcher_version").cloned().unwrap_or_default();
            jvm_args.insert(0, format!("-Dminecraft.launcher.version={}", launcher_version));
        }

        // 3. Classpath (doit être en dernier avant la mainClass)
        if !jvm_args.contains(&"-cp".to_string()) {
            let classpath = variables.get("classpath").cloned().unwrap_or_default();
            jvm_args.push("-cp".to_string());
            jvm_args.push(classpath);
        }

        // Construire le Vec complet : JVM + MainClass + Game
        let mut full_args = jvm_args;
        full_args.push(builder.main_class.main_class.clone());
        full_args.extend(game_args);
        tracing::debug!(args = ?full_args, "Launch arguments built");

        full_args
    }
}

impl<'a> Version<'a> {
    /// Crée la HashMap avec toutes les variables de lancement
    fn create_variable_map(
        &self,
        builder: &VersionBuilder,
        username: &str,
        uuid: &str,
    ) -> HashMap<String, String> {
        let mut map = HashMap::new();

        #[cfg(target_os = "windows")]
        let classpath_separator = ";";
        #[cfg(not(target_os = "windows"))]
        let classpath_separator = ":";

        // Authentification
        map.insert("auth_player_name".to_string(), username.to_string());
        map.insert("auth_uuid".to_string(), uuid.to_string());
        map.insert("auth_access_token".to_string(), "0".to_string());
        map.insert("auth_xuid".to_string(), "0".to_string());
        map.insert("clientid".to_string(), "{client-id}".to_string());
        map.insert("user_type".to_string(), "legacy".to_string());
        map.insert("user_properties".to_string(), "{}".to_string());

        // Version
        map.insert("version_name".to_string(), self.name.to_string());
        map.insert("version_type".to_string(), "release".to_string());

        // Directories
        map.insert("game_directory".to_string(), self.game_dirs.display().to_string());
        map.insert("assets_root".to_string(), self.game_dirs.join("assets").display().to_string());
        map.insert("natives_directory".to_string(), self.game_dirs.join("natives").display().to_string());
        map.insert("library_directory".to_string(), self.game_dirs.join("libraries").display().to_string());

        // Assets index
        let assets_index_name = builder.assets_index
            .as_ref()
            .map(|idx| idx.id.clone())
            .unwrap_or_else(|| self.minecraft_version.to_string());
        map.insert("assets_index_name".to_string(), assets_index_name);

        // Launcher
        map.insert("launcher_name".to_string(), "LightyLauncher".to_string());
        map.insert("launcher_version".to_string(), "1.0.0".to_string());

        // Classpath
        let classpath = self.build_classpath(&builder.libraries);
        map.insert("classpath".to_string(), classpath);
        map.insert("classpath_separator".to_string(), classpath_separator.to_string());

        map
    }

    /// Construit le classpath à partir des libraries
    fn build_classpath(&self, libraries: &[crate::minecraft::version::version_metadata::Library]) -> String {
        #[cfg(target_os = "windows")]
        let separator = ";";
        #[cfg(not(target_os = "windows"))]
        let separator = ":";

        let lib_dir = self.game_dirs.join("libraries");

        let mut classpath_entries: Vec<String> = libraries
            .iter()
            .filter_map(|lib| {
                lib.path.as_ref().map(|path| {
                    lib_dir.join(path).display().to_string()
                })
            })
            .collect();

        // Ajouter le client.jar à la fin
        classpath_entries.push(
            self.game_dirs.join(format!("{}.jar", self.name)).display().to_string()
        );

        classpath_entries.join(separator)
    }

    /// Arguments JVM par défaut (pour anciennes versions sans JVM args)
    fn build_default_jvm_args(&self, variables: &HashMap<String, String>) -> Vec<String> {
        let natives_dir = variables.get("natives_directory").cloned().unwrap_or_default();
        let launcher_name = variables.get("launcher_name").cloned().unwrap_or_default();
        let launcher_version = variables.get("launcher_version").cloned().unwrap_or_default();
        let classpath = variables.get("classpath").cloned().unwrap_or_default();

        vec![
            "-Xms1024M".to_string(),
            "-Xmx2048M".to_string(),
            format!("-Djava.library.path={}", natives_dir),
            format!("-Dminecraft.launcher.brand={}", launcher_name),
            format!("-Dminecraft.launcher.version={}", launcher_version),
            "-cp".to_string(),
            classpath,
        ]
    }
}


/// Replaces variables in a vector of arguments efficiently
fn replace_variables_in_vec(args: &[String], variables: &HashMap<String, String>) -> Vec<String> {
    args.iter()
        .map(|arg| replace_variables_cow(arg, variables).into_owned())
        .collect()
}

/// Efficient variable replacement using Cow (Copy-on-Write)
/// Only allocates when replacements are actually needed
fn replace_variables_cow<'a>(
    input: &'a str,
    variables: &HashMap<String, String>
) -> Cow<'a, str> {
    // Fast path: no variables to replace
    if !input.contains("${") {
        return Cow::Borrowed(input); // Zero allocation!
    }

    // Pre-allocate with extra capacity for replacements
    let mut result = String::with_capacity(input.len() + 128);
    let mut last_end = 0;

    // Find all ${...} patterns
    for (start, _) in input.match_indices("${") {
        if let Some(end_offset) = input[start..].find('}') {
            let end = start + end_offset;
            let key = &input[start + 2..end];

            // Append text before the variable
            result.push_str(&input[last_end..start]);

            // Replace with value or keep original if not found
            if let Some(value) = variables.get(key) {
                result.push_str(value);
            } else {
                result.push_str(&input[start..=end]);
            }

            last_end = end + 1;
        }
    }

    // Append remaining text
    result.push_str(&input[last_end..]);
    Cow::Owned(result)
}

// Note: For regex-based replacement (if needed in the future):
// 1. Add `regex = "1"` to Cargo.toml dependencies
// 2. Use this implementation:
//
// use once_cell::sync::Lazy;
// use regex::Regex;
//
// static VAR_PATTERN: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r"\$\{([^}]+)\}").expect("Invalid regex pattern")
// });
//
// fn replace_variables_regex(input: &str, variables: &HashMap<String, String>) -> String {
//     VAR_PATTERN.replace_all(input, |caps: &regex::Captures| {
//         variables.get(&caps[1]).map(|s| s.as_str()).unwrap_or(&caps[0])
//     }).into_owned()
// }
//
// However, the Cow-based approach above is faster and doesn't require regex.