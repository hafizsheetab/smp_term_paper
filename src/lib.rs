use std::cell::RefCell;
use std::fs;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::Ref;
pub mod types;
pub mod htmltopdf;
use types::ReactComponent;
pub fn run(args: &Vec<String>) {
    let path = args[1].clone();
    let name = args[0].as_str();
    let mut path = path.replace("\\", "/");
    println!("This is the Root path of App.js: {}", &path);
    let component =build_component("App", Path::new(&path), "App").unwrap();
    let ref_component = RefCell::new(component);
    htmltopdf::build_html(ref_component.borrow(), name)
}

// pub fn build_component_tree( path: &String) {
//     let mut child_path = path.clone();
//     child_path.push_str("App.js");
//     let content = fs::read_to_string(&child_path).expect("Should Read File");
//     let lines_with_imports = search("import,from", &content.lines().collect(), false);
//     let lines_with_curly_braces = search("{, }", &lines_with_imports, false);
//     let lines_without_curly_braces = search("{, }", &lines_with_imports, true);ln!

//     if let Some(parent_component) = build_component(&lines_with_curly_braces, &lines_without_curly_braces, "App", path){
//         dbg!(parent_component);
//     }
// }

pub fn build_component(
    name: &str,
    mut dir_path: &Path,
    module_name: &str
) -> Option<ReactComponent> {
    //println!("=-=-=-=-=-=- First {} {} {}", name, dir_path.display(), module_name);
    let mut component = ReactComponent::new(name, dir_path.canonicalize().unwrap().as_os_str().to_str().unwrap());
    let (mut file_name, found) = get_file_name(name, dir_path);
    let mut corrected_relative_file_path = String::from("./");
    if(!found){
        let (relative_file_path_temp, relative_file_dir_temp, mut file_name_temp) = get_module_path_from_directory(&dir_path.join(&name), module_name).unwrap_or_else(|| return ("".to_owned(), "".to_owned(), "".to_owned()));
        if(relative_file_dir_temp == ""){
            return Some(component);
        }
        corrected_relative_file_path = String::from(name);
        corrected_relative_file_path.push_str(&relative_file_dir_temp);
        file_name_temp = get_file_name(&file_name_temp, &dir_path.join(&corrected_relative_file_path)).0;
        file_name = file_name_temp

    }
    let content = fs::read_to_string(Path::new(&dir_path).join(&corrected_relative_file_path).join(&file_name)).expect("Should Read File");
    let number_of_state_variables: Vec<&str> = content.matches("useState").collect();
    let number_of_effect_dependency: Vec<&str> = content.matches("useEffect").collect();
    let number_of_ref_dependency: Vec<&str> = content.matches("useRef").collect();
    let number_of_memos: Vec<&str> = content.matches("useMemo").collect();
    let number_of_context_usages: Vec<&str> = content.matches("useContext").collect();
    let number_of_reducers: Vec<&str> = content.matches("useReducer").collect();
    component.incr_component_internal_dependency(calculate_component_internal_dependency((&number_of_state_variables, &number_of_effect_dependency, &number_of_ref_dependency, &number_of_memos, &number_of_context_usages, &number_of_reducers)));
    let lines_with_imports = search("import,from", &content.lines().collect(), false);
    let lines_with_curly_braces = search("{, }", &lines_with_imports, false);
    for line in & lines_with_curly_braces {
        let  (module_names,  file_name,  relative_file_path,  relative_file_dir) = get_module_name_and_path_from_line(line, true);
        let module_names: Vec<&str> = module_names.split(",").collect();
        if(is_module(&relative_file_dir)){
            component.incr_external_module_dependency(1)
        }
        for module_name in module_names {
           if is_component(module_name.trim(), &file_name, &relative_file_path, &relative_file_dir, &content){
            component.incr_external_component_dependency(1);
            let child_path = dir_path.join(&corrected_relative_file_path).join(&relative_file_dir);
            if let Some(child_component) = build_component(
                &file_name,
                &child_path,
                &module_name
            ) {

                component.add_child(Rc::new(RefCell::new(child_component)))
            }
           }
           else if !is_module(&relative_file_dir) && !path_contains_extension(&relative_file_path){
            component.incr_external_interface_dependency(1)
        }  
        }
    }
    let lines_without_curly_braces = search("{, }", &lines_with_imports, true);
    for line in &lines_without_curly_braces {   
        let  (module_name,  file_name,  relative_file_path,  relative_file_dir) = get_module_name_and_path_from_line(line, false);
        if(is_module(&relative_file_dir)){
            component.incr_external_module_dependency(1);
        }
        if is_component(&module_name, &file_name, &relative_file_path, &relative_file_dir, &content){
            component.incr_external_component_dependency(1);
            let child_path = dir_path.join(&corrected_relative_file_path).join(&relative_file_dir);
            if let Some(child_component) = build_component(
                &file_name,
                &child_path,
                &module_name
            ) {

                component.add_child(Rc::new(RefCell::new(child_component)))
            }
        }
        else if !is_module(&relative_file_dir)  && !path_contains_extension(&relative_file_path){
            component.incr_external_interface_dependency(1)
        }      
        
    }
    Some(component)
}

fn calculate_component_internal_dependency(dependencies: (&Vec<&str>,&Vec<&str>,&Vec<&str>,&Vec<&str>,&Vec<&str>,&Vec<&str>)) -> usize {
    let mut dependency:usize = 0;
    if dependencies.0.len() > 0 {
        dependency += dependencies.0.len() - 1;
    }
    if dependencies.1.len() > 0 {
        dependency += dependencies.1.len() - 1;
    }
    if dependencies.2.len() > 0 {
        dependency += dependencies.2.len() - 1;
    }
    if dependencies.3.len() > 0 {
        dependency += dependencies.3.len() -1;
    }
    if dependencies.4.len() > 0 {
        dependency += dependencies.4.len() - 1;
    }
    if dependencies.5.len() > 0 {
        dependency += dependencies.5.len() - 1;
    }
    dependency
}
fn is_module (relative_file_dir: &str) -> bool {
    if(relative_file_dir.contains("./") || relative_file_dir.contains("../")){
        return  false;
    }
    return true;
}
fn is_component (mut module_name: &str, file_name: &str, relative_file_path: &str, relative_file_dir: &str, content: &str) -> bool {
    if(module_name.contains("as")){
        module_name = module_name.split("as").nth(0).unwrap().trim();
    }
    let print_status = module_name == "ProfileOptions";
    let mut return_value = true;
    // let component_usage_identifier = Regex::new(format!(r".*<{module_name}").as_str()).unwrap();
    if module_name == "" || relative_file_path == "" {
        return_value = false
    }
    else if !relative_file_dir.contains("./") && !relative_file_dir.contains("../") {
        return_value = false
    }
    else if file_name.contains(".") {
        if !file_name.contains(".js") && !file_name.contains(".jsx") && !file_name.contains(".ts") && !file_name.contains(".tsx"){
            return_value = false
        }
    }
    // else if  !component_usage_identifier.is_match(content) {
    //     return_value = false
    // }
    else if(!starts_with_capital_letter(module_name)){
        return_value = false
    }
    
    return_value
}
fn path_contains_extension(relative_file_path: &str) -> bool {
    let last_5_chars = &relative_file_path[relative_file_path.len() - 5 .. relative_file_path.len()];
    if(last_5_chars.contains(".")){
        return true;
    }
    false
}
fn get_module_path_from_directory(path: &Path, module_name: &str) -> Option<(String, String, String)>{
    let (file_name, found) = get_file_name("index", path);
    let read_path = path.join(file_name);
    let contents = fs::read_to_string(read_path).unwrap();
    let get_exported_lines = search("export, from", &contents.lines().collect(), false);
    for line in get_exported_lines {
        if(line.contains(module_name)){
            return Some(get_file_paths(line));
        }
    }
    None
}
fn get_file_name(name: &str, dir_path: &Path) -> (String, bool){
    let files: Vec<String> = fs::read_dir(&dir_path)
    .ok()
    .unwrap()
    .into_iter()
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.metadata().unwrap().is_file())
    .map(|entry| entry.file_name().to_str().unwrap().to_owned())
    .filter(|file_name| ( file_name.ends_with(".js") || file_name.ends_with(".jsx") || file_name.ends_with(".ts") || file_name.ends_with(".tsx")))
    .collect();
    for file in files {
        if file.contains(name){
            return (file, true);
        }
    };
    
    return (String::new(), false);
    
}

fn get_module_name_and_path_from_line(line: &str, curly_braces: bool) -> (String, String, String, String) {
    let (relative_file_path, relative_file_dir, file_name) = get_file_paths(line);
    if curly_braces {
        let module_names = find_string_in_between(line, "{", "}").unwrap_or("").trim();
        (module_names.to_owned(), file_name, relative_file_path, relative_file_dir)
    }
    else{
        let module_name = find_string_in_between(line, "import", "from").unwrap_or("").trim();
        (module_name.to_owned(), file_name, relative_file_path, relative_file_dir)
    }
    
}
pub fn get_file_paths(line: &str) -> (String, String, String){
    let  relative_file_path = find_string_in_between(line, "from", ";").unwrap_or("");
    let  relative_file_path =relative_file_path.replace(&['(', ')', ',', '\"', ';', ':', '\''][..], "");
    let  relative_file_path = relative_file_path.trim().to_owned();
    let mut str_slc: Vec<&str> = relative_file_path.split("/").collect();
    let file_name = str_slc.get(str_slc.len() -1).unwrap().to_owned().to_owned();
    str_slc.remove(str_slc.len() -1);
    let mut relative_file_dir = str_slc.join("/");
    relative_file_dir.push_str("/");
    (relative_file_path, relative_file_dir, file_name)
}
pub fn starts_with_capital_letter(word: &str) -> bool {
    let ascii = word.bytes().nth(0).unwrap_or(0);
    if ascii == 0 {
        return false;
    }
    if ascii >= 65 && ascii <= 90 {
        return true;
    }
    false
}
pub fn find_string_in_between<'a>(
    line: &'a str,
    pattern1: &'a str,
    pattern2: &'a str,
) -> Result<&'a str, &'a str> {
    let start_bytes = match line.find(pattern1) {
        Some(index) => index + pattern1.len(),
        None => 0,
    };
    let end_bytes = match line.find(pattern2) {
        Some(index) => index,
        None => line.len(),
    };
    if start_bytes > end_bytes {
        return Err("Patterns Sent is inverse");
    }
    if start_bytes == 0 && end_bytes == line.len() {
        return Err("Not Found");
    }

    Ok(&line[start_bytes..end_bytes])
}

pub fn search<'a>(query: &str, contents: &Vec<&'a str>, not: bool) -> Vec<&'a str> {
    let queries: Vec<&str> = query
        .split(",")
        .into_iter()
        .map(|query| query.trim())
        .collect();
    let mut results: Vec<&'a str> = vec![];
    for line in contents {
        let mut contains = true;
        for query in &queries {
            if !line.contains(query) {
                contains = false;
                break;
            }
        }
        if not && !contains {
            results.push(line);
        } else if !not && contains {
            results.push(line)
        }
    }
    results
}
