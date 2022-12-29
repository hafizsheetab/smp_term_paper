use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct ReactComponent {
    name: String,
    path: String,
    children: Vec<Rc<RefCell<ReactComponent>>>,
    external_module_dependency: usize,
    component_internal_dependency: usize,
    external_component_dependency: usize,
    external_interface_dependency: usize
}

impl ReactComponent {
    pub fn new(name: &str, path: &str) -> ReactComponent {
        ReactComponent { name: String::from(name), path: String::from(path), children: Vec::new(), external_module_dependency: 0, component_internal_dependency: 0, external_component_dependency: 0, external_interface_dependency: 0 }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn path(&self) -> &String {
        &self.path
    }
    pub fn children(&self) -> &Vec<Rc<RefCell<ReactComponent>>>{
        &self.children
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    pub fn set_path(&mut self, path: &str) {
        self.path = String::from(path);
    }
    pub fn add_child(&mut self, component: Rc<RefCell<ReactComponent>>){
        self.children.push(component);
    }
    pub fn incr_external_module_dependency(&mut self, increment_value: usize) {
        self.external_module_dependency += increment_value;
    }
    pub fn incr_component_internal_dependency(&mut self, increment_value: usize) {
        self.component_internal_dependency += increment_value
    }   
    pub fn incr_external_component_dependency(&mut self, increment_value: usize) {
        self.external_component_dependency += increment_value
    }
    pub fn incr_external_interface_dependency(&mut self, increment_value: usize){
        self.external_interface_dependency += increment_value
    }
    pub fn external_interface_dependency(&self) -> usize{
        self.external_interface_dependency
    }
    pub fn external_module_dependency(&self) -> usize {
        self.external_module_dependency
    }
    pub fn component_internal_dependency(&self) -> usize {
        self.component_internal_dependency
    }
    pub fn external_component_dependency(&self) -> usize {
        self.external_component_dependency
    }

}

