use async_trait::async_trait;
use fuzzy_cognitive_model_common::adjustment::{
    AdjustmentModel, Chromosome, Generation, SaveResult,
};
use wasm_bindgen::prelude::*;

struct SaveResultClient {
    pub adjustment_run_id: i32,
    pub base_url: String,
    pub locale: String,
}

#[async_trait]
impl SaveResult<(), JsValue> for SaveResultClient {
    async fn save_result(&mut self, result_chromosome: &Chromosome) -> Result<(), JsValue> {
        Ok(())
    }
    async fn save_generation(
        &mut self,
        generation: &mut Generation,
        number: i32,
    ) -> Result<(), JsValue> {
        Ok(())
    }
}

#[wasm_bindgen]
pub async fn adjust(
    adjustment_model: JsValue,
    adjustment_run_id: i32,
    base_url: String,
    locale: String,
) -> Result<(), JsValue> {
    let adjustment_model =
        serde_wasm_bindgen::from_value::<AdjustmentModel>(adjustment_model).unwrap();
    if let Err(error) = adjustment_model
        .run(SaveResultClient {
            adjustment_run_id,
            base_url,
            locale,
        })
        .await
    {
        return Err(error);
    }
    Ok(())
}
