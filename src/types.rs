use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct component {
    name: String,
    path: String,
    children: Vec<Rc<RefCell<component>>>,
    external_module_dependency: u32,
    component_internal_dependency: u32,
    module_dependency: u32
}

impl component {
    pub fn new(name: &str, path: &str) -> component {
        component { name: String::from(name), path: String::from(path), children: Vec::new(), external_module_dependency: 0, component_internal_dependency: 0, module_dependency: 0 }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn path(&self) -> &String {
        &self.path
    }
    pub fn children(&self) -> &Vec<Rc<RefCell<component>>>{
        &self.children
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    pub fn set_path(&mut self, path: &str) {
        self.path = String::from(path);
    }
    pub fn add_child(&mut self, component: Rc<RefCell<component>>){
        self.children.push(component);
    }
    pub fn incr_external_module_dependency(&mut self, increment_value: u32) {
        self.external_module_dependency += increment_value;
    }
    pub fn incr_component_internal_dependency(&mut self, increment_value: u32) {
        self.component_internal_dependency += increment_value
    }   
    pub fn incr_module_dependency(&mut self, increment_value: u32) {
        self.module_dependency += increment_value
    }

}

