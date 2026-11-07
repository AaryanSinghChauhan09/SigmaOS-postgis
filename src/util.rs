//! postgis model — GIS/geospatial.

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub path: String,
    pub crs: String,
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub projects: Vec<Project>,
}

impl Default for Model {
    fn default() -> Self { Self { projects: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, name: &str) -> usize {
        let id = self.projects.len();
        self.projects.push(Project { name: name.into(), layers: vec![] });
        id
    }

    pub fn add_layer(&mut self, project_id: usize, name: &str, path: &str, crs: &str) -> bool {
        if let Some(p) = self.projects.get_mut(project_id) {
            p.layers.push(Layer { name: name.into(), path: path.into(), crs: crs.into(), visible: true });
            true
        } else { false }
    }

    pub fn toggle_layer(&mut self, project_id: usize, layer_name: &str) -> bool {
        if let Some(p) = self.projects.get_mut(project_id) {
            if let Some(l) = p.layers.iter_mut().find(|l| l.name == layer_name) {
                l.visible = !l.visible;
                return true;
            }
        }
        false
    }

    pub fn reproject(&self, project_id: usize, from: &str, to: &str) -> Option<String> {
        self.projects.get(project_id).map(|_| format!("reproject {} -> {}", from, to))
    }

    pub fn list(&self) -> &[Project] { &self.projects }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn project_layers() {
        let mut m = Model::new();
        let id = m.create("map1");
        assert!(m.add_layer(id, "roads", "/data/roads.shp", "EPSG:4326"));
        assert!(m.toggle_layer(id, "roads"));
        assert!(m.reproject(id, "EPSG:4326", "EPSG:3857").is_some());
    }
}
