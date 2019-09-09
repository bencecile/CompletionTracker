use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("bundle.js"), all_js::bundle_components())
        .expect("Failed to write the bundle js file");
}

/// This module will handle the building of Vue files and making them into something useable
mod all_js {
    use std::fs;
    use std::io::{Write};
    use std::path::{Path};

    const COMPONENT_DIR: &'static str = "assets/htmlComponents";
    const JS_DIR: &'static str = "assets/js";

    // Some constants for the tags that we will be finding
    const TEMPLATE_TAG: &'static str = "<template>";
    const TEMPLATE_TAG_END: &'static str = "</template>";
    const SCRIPT_TAG: &'static str = "<script>";
    const SCRIPT_TAG_END: &'static str = "</script>";
    const EXPORT_START: &'static str = "export default {";
    const EXPORT_END: &'static str = "}";
    const STYLE_TAG: &'static str = "<style>";
    const STYLE_TAG_END: &'static str = "</style>";


    /// Finds the content within the tag in the search data
    fn find_tag_content<'s>(search_data: &'s str, tag: &str, tag_end: &str) -> &'s str {
        let start_index = search_data.find(tag)
            .expect(&format!("Failed to find the '{}' tag", tag))
            // Go to the index just after the end of the tag
            + tag.len();
        // Look from the end to find the matching end tag
        let end_index = search_data.rfind(tag_end)
            .expect(&format!("Failed to find the '{}' end tag", tag_end));
        // Return the slice of the data that will go right in-between the tags
        // Also trim it so we lose any unneeded whitespace
        search_data[start_index..end_index].trim()
    }

    /// Bundles all of the Vue components together into a single file
    pub fn bundle_components() -> Vec<u8> {
        let mut bundled_js = Vec::new();

        let js_dir = Path::new(JS_DIR);
        let mut write_js_data = |file_path| {
            let js_data = fs::read_to_string(js_dir.join(file_path))
                .expect("Failed to read a JS file");
            write!(bundled_js, r#"
(function() {{
{js_data}
}})();"#, js_data=js_data)
                .expect("Failed to write the JS data");
        };

        if cfg!(debug_assertions) {
            write_js_data("vue@2.6.10.js");
            write_js_data("vue-router@3.1.3.js");
            write_js_data("vue-i18n@8.14.0.js");
        } else {
            // Write the minfied JS instead for when we're in release
            write_js_data("vue@2.6.10.min.js");
            write_js_data("vue-router@3.1.3.min.js");
            write_js_data("vue-i18n@8.14.0.min.js");
        }
        write_js_data("custom.js");
        write_js_data("langStrings.js");
        write_js_data("settings.js");

        // Go through each vue component, appending them to the data
        for file_path in super::read_dir(COMPONENT_DIR, true) {
            // Just in case we have other types of files in there (hopefully temporarily)
            if file_path.extension().unwrap().to_str().unwrap() != "vue" {
                continue;
            }

            let component_data = fs::read_to_string(&file_path)
                .expect("Failed to read a component");

            let template_data = find_tag_content(&component_data, TEMPLATE_TAG, TEMPLATE_TAG_END);
            let script_data = find_tag_content(&component_data, SCRIPT_TAG, SCRIPT_TAG_END);
            // Modify the script data to not include some stuff
            let script_data = find_tag_content(&script_data, EXPORT_START, EXPORT_END);
            let style_data = find_tag_content(&component_data, STYLE_TAG, STYLE_TAG_END);

            // Only include the template entry if we get something
            let template_entry = if template_data.is_empty() {
                String::new()
            } else {
                format!("template: `{}`,", template_data)
            };
            // Only add the function for the style if there is some style to use
            let style_function = if style_data.is_empty() {
                String::new()
            } else {
                format!(r#"
(function() {{
    const style = document.createElement("style");
    style.innerHTML = `{}`;
    document.head.appendChild(style);
}})();"#, style_data)
            };

            // Write a slightly elaborate JS string into our component data
            write!(bundled_js, r#"
Vue.component("{file_name}", {{
    {template_entry}
    {script_data}
}});{style_function}"#, file_name=file_path.file_stem().unwrap().to_str().unwrap(),
            template_entry=template_entry,
            script_data=script_data,
            style_function=style_function)
                .expect("Failed to write the component data");
        }

        bundled_js
    }
}

/// Gets all of the *files* from the directory.
/// If recursive, it will read everything from all the sub-directories
/// Panics if it's not a directory.
fn read_dir(dir: impl AsRef<Path>, recursive: bool) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(dir).expect("Tried reading something that's not a directory")
        .filter_map(move |dir_entry| {
            // Get rid of everything that isn't a file (or dir if recursive)
            let dir_entry = dir_entry.expect("Failed to read a dir entry");
            let file_type = dir_entry.file_type().expect("Failed to read the file type");
            if file_type.is_file() || (recursive && file_type.is_dir()) {
                Some((dir_entry.path(), file_type))
            } else {
                None
            }
        }).flat_map(move |(file_path, file_type)| {
            // TODO This may be able to be cleaned up
            if file_type.is_file() {
                // Just use this single file
                vec![file_path]
            } else {
                // Read this directory now too
                // If it's a directory, we know that we are recursive since that's the
                //  only way we can get a directory here
                read_dir(file_path, recursive).collect::<Vec<PathBuf>>()
            }
        })
}
