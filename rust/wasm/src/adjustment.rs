use async_trait::async_trait;
use fuzzy_cognitive_model_common::adjustment::{
    AdjustmentModel, Generation, Individual, SaveResult, TimeSimulation,
};
use js_sys::Function;
use wasm_bindgen::prelude::*;

struct SaveResultClient {
    pub adjustment_run_id: i32,
}

impl SaveResultClient {
    fn get_function(name: &str) -> Function {
        let window = web_sys::window().unwrap();
        let save_result_client = window.get("saveResultClient").unwrap();
        js_sys::Reflect::get(&save_result_client, &JsValue::from(name))
            .unwrap()
            .dyn_into::<js_sys::Function>()
            .unwrap()
    }
}

#[async_trait]
impl SaveResult<(), JsValue> for SaveResultClient {
    async fn save_result(&mut self, result_chromosome: &Individual) -> Result<(), JsValue> {
        Self::get_function("saveResult")
            .call2(
                &JsValue::undefined(),
                &JsValue::from(self.adjustment_run_id),
                &serde_wasm_bindgen::to_value(result_chromosome).unwrap(),
            )
            .unwrap();
        Ok(())
    }
    async fn save_generation(
        &mut self,
        generation: &mut Generation,
        number: i32,
    ) -> Result<(), JsValue> {
        Self::get_function("saveGeneration")
            .call3(
                &JsValue::undefined(),
                &JsValue::from(self.adjustment_run_id),
                &serde_wasm_bindgen::to_value(generation).unwrap(),
                &JsValue::from(number),
            )
            .unwrap();
        Ok(())
    }
}

#[wasm_bindgen]
pub struct AdjustmentExecutor {
    save_result: SaveResultClient,
    adjustment_model: AdjustmentModel,
}

#[wasm_bindgen]
impl AdjustmentExecutor {
    #[wasm_bindgen(constructor)]
    pub fn new(adjustment_model: JsValue, adjustment_run_id: i32) -> Self {
        Self {
            save_result: SaveResultClient { adjustment_run_id },
            adjustment_model: serde_wasm_bindgen::from_value::<AdjustmentModel>(adjustment_model)
                .unwrap(),
        }
    }
    pub fn start(&mut self) -> () {
        self.adjustment_model.start();
    }
    pub async fn next(&mut self) -> Result<JsValue, JsValue> {
        match self.adjustment_model.next(&mut self.save_result).await {
            Ok(run_next) => Ok(JsValue::from_bool(run_next)),
            Err(error) => Err(error),
        }
    }
    pub async fn finish(&mut self) -> Result<JsValue, JsValue> {
        match self.adjustment_model.finish(&mut self.save_result).await {
            Ok(chromosome) => Ok(serde_wasm_bindgen::to_value(&chromosome).unwrap()),
            Err(error) => Err(error),
        }
    }
}

#[wasm_bindgen]
pub struct TimeSimulationExecutor {
    time_simulation: TimeSimulation,
}

#[wasm_bindgen]
impl TimeSimulationExecutor {
    #[wasm_bindgen(constructor)]
    pub fn new(
        max_model_time: JsValue,
        concepts_map: JsValue,
        connections_map: JsValue,
        target_concepts: JsValue,
        dynamic_model: JsValue,
        concepts: JsValue,
        connections: JsValue,
    ) -> Self {
        Self {
            time_simulation: TimeSimulation::new(
                serde_wasm_bindgen::from_value(max_model_time).unwrap(),
                serde_wasm_bindgen::from_value(concepts_map).unwrap(),
                serde_wasm_bindgen::from_value(connections_map).unwrap(),
                serde_wasm_bindgen::from_value(target_concepts).unwrap(),
                serde_wasm_bindgen::from_value(dynamic_model).unwrap(),
                serde_wasm_bindgen::from_value(concepts).unwrap(),
                serde_wasm_bindgen::from_value(connections).unwrap(),
            ),
        }
    }
    pub fn get_max_model_time(&self) -> i32 {
        self.time_simulation.get_max_model_time()
    }
    pub fn get_current_time(&self) -> i32 {
        self.time_simulation.get_current_time()
    }
    pub fn get_error(&self) -> f64 {
        self.time_simulation.get_error()
    }
    pub fn get_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.time_simulation.get_state()).unwrap()
    }
    pub fn next(&mut self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.time_simulation.next()).unwrap()
    }
}
