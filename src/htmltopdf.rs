use std::{cell::Ref, fs};
use std::sync::atomic::{AtomicU32, Ordering};
use crate::types::ReactComponent;

fn get_html_header() -> String {
    String::from(r#"<head><meta charset="UTF-8"><meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Metrics</title>
    <style>
        body {
            background-color: whitesmoke;
            margin-top: 1%;
        }
        h1{
            text-align: center;
        }
        div{
            display: flex;
            justify-content: left;
            margin: 10px;
        }
        
    </style>
</head>"#)
}
static print_num_1: AtomicU32 = AtomicU32::new(1);
static print_num_2: AtomicU32 = AtomicU32::new(1);
fn get_print_num_1() -> u32 {
    print_num_1.load(Ordering::Relaxed)
}
fn set_print_num_1(num_1: u32){
    print_num_1.store(num_1, Ordering::Relaxed);
}
fn get_print_num_2() -> u32 {
    print_num_2.load(Ordering::Relaxed)
}
fn set_print_num_2(num_2: u32){
    print_num_2.store(num_2, Ordering::Relaxed);
}
fn build_component_tree(component: Ref<ReactComponent>, root: bool, used_components: &mut Vec<String>) -> String {
    let mut component_tree_str = format!(r"<div><b>{}. {}</b></div><div><b>Children: </b>",get_print_num_1(), component.name());
    used_components.push(component.name().to_owned());
    for child in component.children(){
        let child_component = child.borrow();
        component_tree_str.push_str(format!("{}, ", child_component.name()).as_str());
    };
    component_tree_str.push_str("</div> <br />");
    for child in component.children(){
        let child_component = child.borrow();        
        if(!used_components.contains(&child_component.name().to_owned())){
            set_print_num_1(get_print_num_1() + 1);
            component_tree_str.push_str(build_component_tree(child_component, false, used_components).as_str(), );
        }
        
    };
    if(root){
        let mut used_components_2: Vec<String> = Vec::new();
        component_tree_str.push_str("<h1>Metrics</h1>");
        component_tree_str.push_str(build_component_metrics(component, &mut used_components_2).as_str());
    };
    component_tree_str
}
fn build_component_metrics(component: Ref<ReactComponent>,  used_components: &mut Vec<String>) -> String{
    let mut component_metrics_str = format!(r"<div><u><b>Name:{}. {}</b></u></div>
    <div>
        <b>Component Internal Dependency:</b>{}
    </div>
    <div>
        <b>External Component Dependency:</b>{}
    </div>
    <div>
        <b>Module Dependency:</b>{}
    </div>
    <div>
        <b>Interface Dependency:</b>{}
    </div>
    <br />
    ",get_print_num_2(), component.name(), component.component_internal_dependency(), component.external_component_dependency(), component.external_module_dependency(), component.external_interface_dependency());
    used_components.push(component.name().to_owned());
    for child in component.children(){
        let child_component = child.borrow();
        if !used_components.contains(&child_component.name().to_owned()){
            set_print_num_2(get_print_num_2() + 1);
            component_metrics_str.push_str(build_component_metrics(child_component, used_components).as_str());
        }
    };
    component_metrics_str
}

pub fn build_html(root_component: Ref<ReactComponent>, name: &str){
    let mut used_components: Vec<String> = Vec::new();
    let html_str = format!(r#"<!DOCTYPE html><html lang="en"> {} <body><h1>{}</h1> {} </body> </html>"#, get_html_header(), name, build_component_tree(root_component, true, &mut used_components));
    fs::write("output.html", html_str).expect("Could Not Write");
}