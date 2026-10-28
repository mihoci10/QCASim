//! Registry of the built-in simulation models and construction of the model
//! a design selects. Shared by the `qca-sim` CLI and the QCAForge desktop
//! application.

use crate::design::file::QCADesign;
use crate::objects::cell::CellType;
use crate::simulation::bistable::BistableModel;
use crate::simulation::get_num_inputs;
use crate::simulation::icha::ICHAModel;
use crate::simulation::model::SimulationModelTrait;

/// A fresh instance of every built-in simulation model.
pub fn available_models() -> Vec<Box<dyn SimulationModelTrait>> {
    vec![Box::new(BistableModel::new()), Box::new(ICHAModel::new())]
}

/// Creates the built-in model with the given unique id.
pub fn create_sim_model(model_id: &str) -> Option<Box<dyn SimulationModelTrait>> {
    available_models()
        .into_iter()
        .find(|model| model.get_unique_id() == model_id)
}

/// Builds the simulation model selected in `design` (with its model and clock
/// generator settings applied) and validates the custom input sequence, if
/// one is enabled.
pub fn prepare_simulation(
    design: &QCADesign,
) -> Result<(Box<dyn SimulationModelTrait>, Option<Vec<Vec<usize>>>), String> {
    let model_id = design
        .simulation_settings
        .selected_simulation_model_id
        .clone()
        .ok_or("No simulation model is selected")?;
    let settings = design
        .simulation_settings
        .simulation_model_settings
        .get(&model_id)
        .ok_or(format!("Design has no settings for model '{}'", model_id))?;

    let mut model = create_sim_model(&model_id).ok_or("No model with such id exists")?;
    // The bistable approximation describes two-state (4-dot) cells only; on
    // other cells it would write data the .qcs reader can't parse.
    if model_id == BistableModel::new().get_unique_id()
        && design.layers.iter().any(|layer| {
            design
                .cell_architectures
                .get(&layer.cell_architecture_id)
                .is_some_and(|architecture| architecture.dot_count != 4)
        })
    {
        return Err("The bistable model only supports 4-dot cells".into());
    }
    model
        .deserialize_model_settings(&settings.model_settings.to_string())
        .map_err(|e| format!("Error parsing model settings: {}", e))?;
    model
        .deserialize_clock_generator_settings(&settings.clock_generator_settings.to_string())
        .map_err(|e| format!("Error parsing clock generator settings: {}", e))?;

    let custom_input_sequence = if design.simulation_settings.use_custom_input_sequence {
        let sequence = design.simulation_settings.custom_input_sequence.clone();
        if sequence.is_empty() {
            return Err("Custom input sequence is enabled but has no vectors".into());
        }
        let num_inputs = get_num_inputs(&design.layers);
        if sequence.iter().any(|vector| vector.len() != num_inputs) {
            return Err(format!(
                "Every vector in the custom input sequence must have exactly {} value(s), one per input",
                num_inputs
            ));
        }
        // Each input takes 2 states per polarization of its layer's architecture.
        let num_states: Vec<usize> = design
            .layers
            .iter()
            .flat_map(|layer| {
                let states = design
                    .cell_architectures
                    .get(&layer.cell_architecture_id)
                    .map_or(0, |architecture| architecture.dot_count as usize / 2);
                layer
                    .cells
                    .iter()
                    .filter(|cell| cell.typ == CellType::Input)
                    .map(move |_| states)
            })
            .collect();
        if sequence
            .iter()
            .any(|vector| vector.iter().zip(&num_states).any(|(state, n)| state >= n))
        {
            return Err(
                "The custom input sequence contains a state its input cell cannot take".into(),
            );
        }
        Some(sequence)
    } else {
        None
    };

    Ok((model, custom_input_sequence))
}
