// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pyo3::{prelude::*};
use serde::Serialize;
// use tensorflow::{
//     Graph, Operation, SavedModelBundle, SessionOptions, DEFAULT_SERVING_SIGNATURE_DEF_KEY,
//     PREDICT_INPUTS, PREDICT_OUTPUTS,
// };

// const MODEL_PATH: &str = "./model/input.h5";
// const OUTPUT_MODEL_PATH: &str = "./model/output.h5";
// const MODEL_TAG: &str = "serve";

// pub struct MLModel {
//     bundle: SavedModelBundle,
//     input_op: Operation,
//     input_index: i32,
//     output_op: Operation,
//     output_index: i32,
// }

// impl MLModel {
//     pub fn init_model() -> Self {
//         let mut graph = Graph::new();
//         let mut session_options = SessionOptions::new();
//         session_options.set_target(&MODEL_PATH)?;
//         let bundle = SavedModelBundle::load(
//             &session_options,
//             &[MODEL_TAG],
//             &mut graph,
//             &OUTPUT_MODEL_PATH,
//         )?;

//         let sig = bundle
//             .meta_graph_def()
//             .get_signature(DEFAULT_SERVING_SIGNATURE_DEF_KEY)?;
//         let input_info = sig.get_input(PREDICT_INPUTS)?;
//         let output_info = sig.get_output(PREDICT_OUTPUTS)?;
//         let input_op = graph.operation_by_name_required(&input_info.name().name)?;
//         let output_op = graph.operation_by_name_required(&output_info.name().name)?;
//         let input_index = input_info.name().index;
//         let output_index = output_info.name().index;

//         MLModel {
//             bundle,
//             input_op,
//             input_index,
//             output_op,
//             output_index,
//         }
//     }
// }

#[derive(Debug)]
pub enum CommandError {
    PythonError{msg: String},
}

impl Serialize for CommandError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CommandError::PythonError{msg } => serializer.serialize_str(&format!("PythonError {}", msg)),
        }
    }
}

impl From<pyo3::PyErr> for CommandError {
    fn from(err: pyo3::PyErr) -> Self {
        CommandError::PythonError{msg: err.to_string()}
    }
}

#[tauri::command]
fn record(_blob_url: &str) -> Result<String, CommandError> {
    // println!("{blob_url}");
    // let model = MLModel::init_model();
    pyo3::prepare_freethreaded_python();
    let py_module = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/python/test.py"));
    let k: Result<String, CommandError> = Python::with_gil(|py| {
        let app = PyModule::from_code(py, py_module, "test.py", "test")?;
        let result = app.getattr("test_func")?.call0()?;
        
        println!("{:?}", result.to_string());
        Ok(result.to_string())
    });

    match k {
        Ok(val) => {
            Ok(val)
        },
        Err(e) => {
            Err(e)
        },
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![record])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
