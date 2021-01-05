// TODO: make the type a peppaint::Type

pub struct VertexLayout {
    pub stride: usize,
    pub attrib_layouts: Vec<AttributeLayout>
}

pub struct AttributeLayout {
    pub location: usize, 
    pub n_elements: usize,
    pub byte_offset: usize,
    pub type_: crate::Type
}

pub trait Vertex { 
    fn get_layout() -> VertexLayout;
}