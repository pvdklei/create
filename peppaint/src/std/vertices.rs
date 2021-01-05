use crate::{
    Vec3,
    Vec2,
    AttributeLayout,
    Vertex,
    Float,
    Color,
    VertexLayout,
    Type
};

#[repr(C, packed)]
#[derive(Clone, Debug, Copy)]
pub struct DepricPaintingVertex { 
    pub pos: Vec3, 
    pub col: Color,
    pub model: Float
}

#[repr(C, packed)]
#[derive(Clone, Debug, Copy)]
pub struct PaintingVertex { 
    pub pos: (f32, f32), 
    pub color: (f32, f32, f32, f32),
    pub model: f32
}

#[repr(C, packed)]
#[derive(Clone, Debug, Copy)]
pub struct Pos(pub f32, pub f32, pub f32);

#[repr(C, packed)]
#[derive(Clone, Debug, Copy)]
pub struct PosColor {
    pub pos: Vec3,
    pub color: Color
}

#[repr(C, packed)]
#[derive(Clone, Debug, Copy)]
pub struct PosColorTex {
    pub pos: Vec3,
    pub color: Color,
    pub texcoords: Vec2
}

impl Vertex for PaintingVertex {
    fn get_layout() -> VertexLayout {
        let stride = 28;
        let al1 = AttributeLayout {
            location: 0,
            n_elements: 2, 
            byte_offset: 0,
            type_: Type::Float
        };
        let al2 = AttributeLayout {
            location: 1,
            n_elements: 4,
            byte_offset: 8,
            type_: Type::Float
        };
        let al3 = AttributeLayout {
            location: 2,
            n_elements: 1,
            byte_offset: 24,
            type_: Type::Float
        };
        let als = vec![al1, al2, al3];
        VertexLayout {
            stride,
            attrib_layouts: als
        }
    }
}

impl Vertex for DepricPaintingVertex {
    fn get_layout() -> VertexLayout {
        let stride = 32;
        let al1 = AttributeLayout {
            location: 0,
            n_elements: 3, 
            byte_offset: 0,
            type_: Type::Float
        };
        let al2 = AttributeLayout {
            location: 1,
            n_elements: 4,
            byte_offset: 12,
            type_: Type::Float
        };
        let al3 = AttributeLayout {
            location: 2,
            n_elements: 1,
            byte_offset: 28,
            type_: Type::Float
        };
        let als = vec![al1, al2, al3];
        VertexLayout {
            stride,
            attrib_layouts: als
        }
    }
}

impl Vertex for Pos {
    fn get_layout() -> VertexLayout {
        let stride = 12;

        let al1 = AttributeLayout {
            location: 0,
            n_elements: 3,
            type_: Type::Float,
            byte_offset: 0
        };

        let attrib_layouts = vec![al1];
        VertexLayout {
            stride,
            attrib_layouts
        }
    }
}

impl Vertex for PosColor {
    fn get_layout() -> VertexLayout {
        let stride = 28;

        let al1 = AttributeLayout {
            location: 0,
            n_elements: 3,
            type_: Type::Float,
            byte_offset: 0
        };

        let al2 = AttributeLayout {
            location: 1,
            n_elements: 4,
            type_: Type::Float,
            byte_offset: 12
        };

        let attrib_layouts = vec![al1, al2];
        VertexLayout {
            stride,
            attrib_layouts
        }
    }
}