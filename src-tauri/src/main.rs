// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::f32::consts::E;

use tensorflow::{
    Graph, Operation, SavedModelBundle, SessionOptions, DEFAULT_SERVING_SIGNATURE_DEF_KEY,
    PREDICT_INPUTS, PREDICT_OUTPUTS,
};

const MODEL_PATH: &str = "./model/input.h5";
const OUTPUT_MODEL_PATH: &str = "./model/output.h5";
const MODEL_TAG: &str = "serve";

pub struct MLModel {
    bundle: SavedModelBundle,
    input_op: Operation,
    input_index: i32,
    output_op: Operation,
    output_index: i32,
}

impl MLModel {
    pub fn init_model() -> Self {
        let mut graph = Graph::new();
        let mut session_options = SessionOptions::new();
        session_options.set_target(&MODEL_PATH)?;
        let bundle = SavedModelBundle::load(
            &session_options,
            &[MODEL_TAG],
            &mut graph,
            &OUTPUT_MODEL_PATH,
        )?;

        let sig = bundle
            .meta_graph_def()
            .get_signature(DEFAULT_SERVING_SIGNATURE_DEF_KEY)?;
        let input_info = sig.get_input(PREDICT_INPUTS)?;
        let output_info = sig.get_output(PREDICT_OUTPUTS)?;
        let input_op = graph.operation_by_name_required(&input_info.name().name)?;
        let output_op = graph.operation_by_name_required(&output_info.name().name)?;
        let input_index = input_info.name().index;
        let output_index = output_info.name().index;

        MLModel {
            bundle,
            input_op,
            input_index,
            output_op,
            output_index,
        }
    }
}

#[tauri::command]
fn record(blob_url: &str) -> String {
    println!("{blob_url}");
    let model = MLModel::init_model();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![record])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
