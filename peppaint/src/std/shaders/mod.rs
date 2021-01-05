use crate::ShaderProgram;

pub fn pos_color() -> ShaderProgram {
    ShaderProgram::from_frag_and_vert_src(
        include_str!("./pos_color.frag"),
        include_str!("./pos_color.vert"),
    ).expect("Could not load PosColor Shader ShaderProgram")
}