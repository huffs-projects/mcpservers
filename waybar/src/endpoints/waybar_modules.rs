use crate::models::WaybarModuleOption;
use crate::utils::WaybarSchema;

pub fn query_modules(filter_module: Option<String>) -> Vec<WaybarModuleOption> {
    let all_modules = WaybarSchema::get_all_modules();
    let mut results = Vec::new();

    if let Some(filter) = filter_module {
        if let Some(module_options) = all_modules.get(&filter) {
            results.extend_from_slice(module_options);
        }
    } else {
        for module_options in all_modules.values() {
            results.extend_from_slice(module_options);
        }
    }

    results
}

pub fn list_all_module_names() -> Vec<String> {
    WaybarSchema::get_all_modules()
        .keys()
        .cloned()
        .collect()
}

