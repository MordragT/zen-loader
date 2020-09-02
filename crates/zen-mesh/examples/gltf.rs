use std::fs::File;
use std::io::Cursor;
use zen_archive::Vdfs;
use zen_mesh::{gltf, object::ObjectMesh, GeneralMesh};
use zen_parser::prelude::*;

fn main() {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Meshes.vdf").unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    //vdf.list();
    let mesh_entry = vdf.get_by_name("ORC_MASTERTHRONE.MRM").unwrap().unwrap();
    println!("Data length: {}", mesh_entry.data.len());
    let cursor = Cursor::new(mesh_entry.data);
    let object_mesh = ObjectMesh::new(cursor, "ORC_MASTERTHRONE").unwrap();
    let mesh = GeneralMesh::from(object_mesh);
    mesh.to_obj("ORC_MASTERTHRONE.obj");
    // let gltf = gltf::to_gltf(mesh, gltf::Output::Standard);
    //println!("{:?}", gltf);
}