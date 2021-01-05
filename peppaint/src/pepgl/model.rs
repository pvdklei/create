use crate::{
    VertexArrayObject,
    Texture,
};

use std::path::Path;

pub struct Model {
    meshes: Vec<ModelMesh>,
    materials: Vec<Material>
}

impl Model {

    pub fn dot_obj(path: &Path) -> Result<Self, String> {

        if !path.exists() {
            return Err("Incorrect path to .obj file".to_string())
        }

        let obj = tobj::load_obj(&path);
        if !obj.is_ok() {
            return Err("There is something wrong with the .obj file".to_string())
        }
        let (objmodels, _objmaterials) = obj.unwrap();

        let mut meshes = Vec::new();
        let materials = Vec::new();

        for model in objmodels {
            let mut vao = VertexArrayObject::new_static();
            let n_indices = model.mesh.indices.len();
            vao.bind();
            vao.buffer_indices(&model.mesh.indices);
            vao.buffer_to_new_vbo(&model.mesh.positions);
            vao.set_attrib_layout(0, 3, 12, 0, crate::Type::Float);
            meshes.push(ModelMesh { vao, n_indices })
        }

        Ok(Self { meshes, materials })
    }

    pub fn show(&self) {
        for mesh in self.meshes.iter() {
            mesh.vao.bind();
            crate::gl_draw_tris(mesh.n_indices);
        }
    }

}

struct ModelMesh {
    vao: VertexArrayObject,
    n_indices: usize
}

struct Material {

}