use std::collections::HashMap;

use pyo3::types::{PyDict, PyType};
use pyo3::{create_exception, prelude::*};
use pyo3::exceptions::PyException;
use qris::node::{Node, Nodes, Value};
use image;
use fast_qr::convert::image::ImageBuilder;
use fast_qr::qr::QRBuilder;
use rqrr;
// use qris::node::Nodes;
#[pyclass]
struct QRIS{
    pub nodes: Nodes
}
create_exception!(pyqris, ErrorToObtainImage, PyException);
create_exception!(pyqris, ParsingError, PyException);

trait NodeMod {
    fn to_dict(&mut self) -> Py<PyDict>;
}

impl NodeMod for Nodes {
    fn to_dict(&mut self) -> Py<PyDict> {
        let py = Python::with_gil(|py|{
            let hashmap = PyDict::new_bound(py);
            self.nodes.iter_mut().for_each(|node| {
                match &mut node.value {
                    Value::Nodes(e) => {
                        let childhashmap = PyDict::new_bound(py);
                        e.iter_mut().for_each(|nested_node| {
                            // print!()
                            let nested_hashmap = nested_node.to_dict();
                            let getitem = nested_hashmap.bind(py).get_item(nested_node.code).unwrap().unwrap();
                            childhashmap.set_item(nested_node.code, getitem).unwrap();
                        });
                        hashmap.set_item(node.code, childhashmap).unwrap();
                    }
                    Value::Value(f) => {
                        hashmap.set_item(node.code, f.clone()).unwrap();
                    }
                }
            });
            hashmap.unbind()
        });
        py

    }
}
#[derive(Debug,FromPyObject)]
enum NodeHashMap{
    String(String),
    Nested(HashMap<u8, NodeHashMap>),
}

impl NodeHashMap{
    pub fn to_nodes(&self) -> Value {
        match self {
            Self::Nested(nodes)=>{
                let mut sorted_keys = nodes.keys().cloned().collect::<Vec<u8>>();
                sorted_keys.sort();
                let mut node_vec: Vec<Node> = Vec::new();
                sorted_keys.iter().for_each(|&k|{
                    if let Some(node) = nodes.get(&k){
                        node_vec.push(Node { code: k, value: node.to_nodes() });
                    }
                });
                Value::Nodes(node_vec)
            },
            Self::String(val)=>{
                Value::Value(val.to_string())
            }
        }
    }
}

impl NodeMod for Node {
    fn to_dict(&mut self) -> Py<PyDict>{
        Python::with_gil(|py|{
            let hashmap = PyDict::new_bound(py);
            match &mut self.value {
                Value::Nodes(ref mut nodes)=>{
                    let childhashmap=PyDict::new_bound(py);
                    nodes.iter_mut().for_each(|node|{
                        childhashmap.set_item(node.code, node.to_dict()).unwrap();
                    });
                    hashmap.set_item(self.code, childhashmap).unwrap();
                },
                Value::Value(val)=>{
                    hashmap.set_item(self.code, val.to_string()).unwrap();
                }
            };
            hashmap.unbind()
        })
    }
}

#[pymethods]
impl QRIS {
    #[new]
    fn from_path(path: &str)-> PyResult<Self>{
        let img = image::open(path);
        match img {
            Ok(img) => {
                let luma8 = img.to_luma8();
                let mut detect = rqrr::PreparedImage::prepare(luma8);
                let grid = detect.detect_grids();
                if grid.len() > 0 {
                    match grid[0].decode() {
                        Ok((_d, content))=>{
                            match Nodes::from_str(&content) {
                                Ok(nodes)=>{
                                    Ok(QRIS{nodes})
                                },
                                Err(e)=>{
                                    Err(ErrorToObtainImage::new_err(e.to_string()))
                                }
                            }
                        },
                        Err(e)=>{
                            Err(ErrorToObtainImage::new_err(e.to_string()))
                        }
                    }
                }else{
                    Err(ErrorToObtainImage::new_err("QR not detected"))
                }
            },
            Err(e)=>{
                Err(ErrorToObtainImage::new_err(e.to_string()))
            }
        }

    }
    #[classmethod]
    fn from_str(_:&Bound<'_, PyType>, code: String) -> PyResult<Self>{
        let nodes = Nodes::from_str(&code);
        match nodes {
            Ok(nodes)=>{
                Ok(Self{nodes})
            },
            Err(e)=>{
                Err(ParsingError::new_err(e.to_string()))
            }
        }
    }
    #[classmethod]
    fn from_dict(_cls:&Bound<'_, PyType>, data: Py<PyAny>) -> PyResult<Self> {
        let result = Python::with_gil(|py|{
            data.extract::<NodeHashMap>(py).unwrap().to_nodes()
        });
        match result {
            Value::Value(_)=>{
                Err(ParsingError::new_err("invalid structure"))
            },
            Value::Nodes(node_vec)=>{
                let nodes = Nodes{nodes:node_vec};
                Ok(Self{nodes})
            }
        }
    }
    fn save(&self, path: &str, width: u32) -> PyResult<()>{
        let qr_builder = QRBuilder::new(self.nodes.dumps()).build().unwrap();
        let result = ImageBuilder::default().fit_width(width).to_file(&qr_builder, path);
        match result {
            Ok(_)=> {
                Ok(())
            },
            Err(e)=>{
                Err(ErrorToObtainImage::new_err(e.to_string()))
            }
        }
    }
    fn to_dict(&mut self)->Py<PyDict>{
        self.nodes.to_dict()
    }
    fn set_merchant_name(&mut self, name: String){
        self.nodes.set_merchant_name(name);
    }
    fn set_merchant_city(&mut self, city: String){
        self.nodes.set_merchant_city(city);
    }
    fn set_postal_code(&mut self, code: String){
        self.nodes.set_postal_code(code);
    }
    fn dumps(&self) -> String{
        self.nodes.dumps()
    }
    fn __repr__(&self)->String{
        format!("{:#?}", self.nodes)
    }

}

/// A Python module implemented in Rust.
#[pymodule]
fn _pyqris(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _=m.add_class::<QRIS>();
    Ok(())
}
