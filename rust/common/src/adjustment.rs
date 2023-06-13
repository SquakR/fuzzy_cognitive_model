use async_trait::async_trait;
use ordered_float::OrderedFloat;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait SaveResult<T, E> {
    async fn save_result(&mut self, result_chromosome: &Chromosome) -> Result<T, E>;
    async fn save_generation(&mut self, generation: &mut Generation, number: i32) -> Result<T, E>;
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DynamicModel {
    DeltaDelta,
    DeltaValue,
    ValueDelta,
    ValueValue,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopCondition {
    pub max_generations: i32,
    pub max_without_improvements: i32,
    pub error: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentInput {
    pub name: String,
    pub description: String,
    pub max_model_time: i32,
    pub dynamic_model: DynamicModel,
    pub generation_size: i32,
    pub generation_save_interval: i32,
    pub stop_condition: StopCondition,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentModel {
    pub adjustment_input: AdjustmentInput,
    pub concepts_map: HashMap<i32, Arc<Concept>>,
    pub control_concepts: Vec<Arc<Concept>>,
    pub target_concepts: Vec<Arc<Concept>>,
    pub regular_concepts: Vec<Arc<Concept>>,
    pub connections_map: HashMap<i32, Arc<Connection>>,
    pub control_connections: Vec<Arc<Connection>>,
    without_improvements: i32,
    current_generation: Option<Generation>,
    generation_number: i32,
    is_generation_saved: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Concept {
    pub id: i32,
    pub value: f64,
    pub is_control: bool,
    pub is_target: bool,
    pub target_value: Option<f64>,
    pub constraint: Option<Constraint>,
    pub dynamic_model: Option<DynamicModel>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub id: i32,
    pub value: f64,
    pub source_id: i32,
    pub target_id: i32,
    pub is_control: bool,
    pub constraint: Option<Constraint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constraint {
    pub min_value: f64,
    pub include_min_value: bool,
    pub max_value: f64,
    pub include_max_value: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chromosome {
    pub id: Option<i32>,
    pub concepts: HashMap<i32, f64>,
    pub connections: HashMap<i32, f64>,
    pub error: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Generation {
    pub chromosomes: Vec<Chromosome>,
    pub error: f64,
}

const ALPHA: f64 = 0.5;
const ERROR_DIFF: f64 = 0.001;

impl AdjustmentModel {
    pub fn new(
        adjustment_input: AdjustmentInput,
        concepts_map: HashMap<i32, Arc<Concept>>,
        control_concepts: Vec<Arc<Concept>>,
        target_concepts: Vec<Arc<Concept>>,
        regular_concepts: Vec<Arc<Concept>>,
        connections_map: HashMap<i32, Arc<Connection>>,
        control_connections: Vec<Arc<Connection>>,
    ) -> Self {
        Self {
            adjustment_input,
            concepts_map,
            control_concepts,
            target_concepts,
            regular_concepts,
            connections_map,
            control_connections,
            without_improvements: 0,
            current_generation: None,
            generation_number: 0,
            is_generation_saved: false,
        }
    }
    pub fn start(&mut self) -> () {
        self.without_improvements = 0;
        self.current_generation = Some(self.create_first_generation());
        self.generation_number = 0;
        self.is_generation_saved = false;
    }
    pub async fn next<S, T, E>(&mut self, save_result: &mut S) -> Result<bool, E>
    where
        S: SaveResult<T, E>,
    {
        if self.generation_number >= self.adjustment_input.stop_condition.max_generations
            || self.without_improvements
                >= self
                    .adjustment_input
                    .stop_condition
                    .max_without_improvements
        {
            return Ok(false);
        }
        if self.generation_number % self.adjustment_input.generation_save_interval == 0 {
            save_result
                .save_generation(
                    self.current_generation.as_mut().unwrap(),
                    self.generation_number + 1,
                )
                .await?;
            self.is_generation_saved = true;
        }
        let best_chromosome_error = self.get_best_chromosome().error;
        if best_chromosome_error < self.adjustment_input.stop_condition.error {
            return Ok(false);
        }
        let next_generation = self.create_next_generation();
        self.generation_number += 1;
        self.is_generation_saved = false;
        if (next_generation.error - self.current_generation.as_ref().unwrap().error).abs()
            < ERROR_DIFF
        {
            self.without_improvements += 1;
        } else {
            self.without_improvements = 0;
        }
        self.current_generation = Some(next_generation);
        Ok(
            self.generation_number < self.adjustment_input.stop_condition.max_generations
                && self.without_improvements
                    < self
                        .adjustment_input
                        .stop_condition
                        .max_without_improvements,
        )
    }
    pub async fn finish<S, T, E>(&mut self, save_result: &mut S) -> Result<Chromosome, E>
    where
        S: SaveResult<T, E>,
    {
        if !self.is_generation_saved {
            save_result
                .save_generation(
                    self.current_generation.as_mut().unwrap(),
                    self.generation_number + 1,
                )
                .await?;
        }
        let best_chromosome = self.get_best_chromosome();
        save_result.save_result(best_chromosome).await?;
        return Ok(best_chromosome.clone());
    }
    fn get_chromosome_error(
        &self,
        concepts: &HashMap<i32, f64>,
        connections: &HashMap<i32, f64>,
    ) -> f64 {
        let concepts = self.get_initial_state(concepts);
        let time_simulation = TimeSimulation::new(
            self.adjustment_input.max_model_time,
            self.concepts_map.clone(),
            self.connections_map.clone(),
            self.target_concepts.clone(),
            self.adjustment_input.dynamic_model.clone(),
            concepts,
            connections.clone(),
        );
        let mut error = time_simulation.get_error();
        for data in time_simulation {
            error = data.error;
        }
        error
    }
    fn get_generation_error(chromosomes: &[Chromosome]) -> f64 {
        chromosomes
            .iter()
            .map(|chromosome| chromosome.error)
            .sum::<f64>()
            / chromosomes.len() as f64
    }
    fn select_parent_candidates(&self, rng: &mut ThreadRng) -> Vec<&Chromosome> {
        let generation = self.current_generation.as_ref().unwrap();
        let mut parents = Vec::new();
        for _ in 0..self.adjustment_input.generation_size {
            let candidate1 =
                &generation.chromosomes[rng.gen_range(0..generation.chromosomes.len())];
            let candidate2 =
                &generation.chromosomes[rng.gen_range(0..generation.chromosomes.len())];
            if candidate1.error <= candidate2.error {
                parents.push(candidate1);
            } else {
                parents.push(candidate2);
            }
        }
        parents
    }
    fn cross_chromosomes(
        &self,
        parent1: &Chromosome,
        parent2: &Chromosome,
        rng: &mut ThreadRng,
    ) -> Vec<Chromosome> {
        if rng.gen::<f64>() < 0.05 {
            return vec![parent1.clone(), parent2.clone()];
        }
        return vec![
            self.create_child_chromosome(parent1, parent2, rng),
            self.create_child_chromosome(parent1, parent2, rng),
        ];
    }
    fn mutate_chromosome(&self, mut chromosome: Chromosome, rng: &mut ThreadRng) -> Chromosome {
        let (concept_probability, connection_probability) = if rng.gen::<f64>() < 0.5 {
            (0.9, 0.5)
        } else {
            (0.5, 0.9)
        };
        if chromosome.concepts.len() > 0 && rng.gen::<f64>() < concept_probability {
            let concept = &self.control_concepts[rng.gen_range(0..self.control_concepts.len())];
            *chromosome.concepts.get_mut(&concept.id).unwrap() = concept.generate_value(rng);
        }
        if chromosome.connections.len() > 0 && rng.gen::<f64>() < connection_probability {
            let connection =
                &self.control_connections[rng.gen_range(0..self.control_connections.len())];
            *chromosome.connections.get_mut(&connection.id).unwrap() =
                connection.generate_value(rng);
        }
        let error = self.get_chromosome_error(&chromosome.concepts, &chromosome.connections);
        chromosome.error = error;
        chromosome
    }
    fn create_first_generation(&self) -> Generation {
        let mut rng = rand::thread_rng();
        let mut chromosomes = Vec::new();
        for _ in 0..self.adjustment_input.generation_size {
            chromosomes.push(Self::create_random_chromosome(self, &mut rng));
        }
        let error = Self::get_generation_error(&chromosomes);
        Generation { chromosomes, error }
    }
    fn create_next_generation(&self) -> Generation {
        let mut rng = rand::thread_rng();
        let mut rng_clone = rng.clone();
        let chromosomes = self
            .select_parent_candidates(&mut rng)
            .chunks(2)
            .flat_map(|chunk| match chunk {
                &[p1, p2] => self.cross_chromosomes(p1, p2, &mut rng),
                &[p] => vec![p.clone()],
                _ => unreachable!(),
            })
            .map(|chromosome| self.mutate_chromosome(chromosome, &mut rng_clone))
            .collect::<Vec<_>>();
        let error = Self::get_generation_error(&chromosomes);
        Generation { chromosomes, error }
    }
    fn create_random_chromosome(&self, rng: &mut ThreadRng) -> Chromosome {
        let mut concepts = HashMap::new();
        for concept in &self.control_concepts {
            concepts.insert(concept.id, concept.generate_value(rng));
        }
        let mut connections = HashMap::new();
        for connection in &self.control_connections {
            connections.insert(connection.id, connection.generate_value(rng));
        }
        let error = self.get_chromosome_error(&concepts, &connections);
        Chromosome {
            id: None,
            concepts,
            connections,
            error,
        }
    }
    fn create_child_chromosome(
        &self,
        parent1: &Chromosome,
        parent2: &Chromosome,
        rng: &mut ThreadRng,
    ) -> Chromosome {
        let mut concepts = HashMap::new();
        for (id, p1) in &parent1.concepts {
            let mut p1 = *p1;
            let mut p2 = parent2.concepts[&id];
            if p1 > p2 {
                (p1, p2) = (p2, p1);
            }
            let mut min = f64::max(p1 - ALPHA * (p2 - p1), 0.0);
            let mut max = f64::min(p2 + ALPHA * (p2 - p1), 1.0);
            let concept = &self.concepts_map[id];
            match &concept.constraint {
                Some(constraint) => {
                    min = constraint.get_min(min);
                    max = constraint.get_max(max);
                }
                None => {}
            }
            concepts.insert(*id, rng.gen_range(min..=max));
        }
        let mut connections = HashMap::new();
        for (id, p1) in &parent1.connections {
            let mut p1 = *p1;
            let mut p2 = parent2.connections[&id];
            if p1 > p2 {
                (p1, p2) = (p2, p1);
            }
            let mut min = f64::max(p1 - ALPHA * (p2 - p1), -1.0);
            let mut max = f64::min(p2 + ALPHA * (p2 - p1), 1.0);
            let connection = &self.connections_map[id];
            match &connection.constraint {
                Some(constraint) => {
                    min = constraint.get_min(min);
                    max = constraint.get_max(max);
                }
                None => {}
            }
            connections.insert(*id, rng.gen_range(min..=max));
        }
        Chromosome {
            id: None,
            concepts,
            connections,
            error: 0.0,
        }
    }
    fn get_best_chromosome(&self) -> &Chromosome {
        self.current_generation
            .as_ref()
            .unwrap()
            .chromosomes
            .iter()
            .min_by_key(|chromosome| OrderedFloat(chromosome.error))
            .unwrap()
    }
    fn get_initial_state(&self, concepts: &HashMap<i32, f64>) -> State {
        let mut state = concepts.clone();
        for concept in &self.regular_concepts {
            state.insert(concept.id, concept.value);
        }
        for concept in &self.target_concepts {
            state.insert(concept.id, concept.value);
        }
        state
    }
}

pub struct TimeSimulation {
    max_model_time: i32,
    current_time: i32,
    error: f64,
    concepts_map: HashMap<i32, Arc<Concept>>,
    connections_map: HashMap<i32, Arc<Connection>>,
    target_concepts: Vec<Arc<Concept>>,
    dynamic_model: DynamicModel,
    previous_state: HashMap<i32, f64>,
    delta_state: HashMap<i32, f64>,
    connections: HashMap<i32, f64>,
}

#[derive(Serialize)]
pub struct TimeSimulationData {
    pub time: i32,
    pub error: f64,
    pub state: HashMap<i32, f64>,
}

impl TimeSimulation {
    pub fn new(
        max_model_time: i32,
        concepts_map: HashMap<i32, Arc<Concept>>,
        connections_map: HashMap<i32, Arc<Connection>>,
        target_concepts: Vec<Arc<Concept>>,
        dynamic_model: DynamicModel,
        concepts: HashMap<i32, f64>,
        connections: HashMap<i32, f64>,
    ) -> Self {
        let previous_state = concepts;
        let delta_state = previous_state.clone();
        Self {
            max_model_time,
            current_time: 1,
            concepts_map,
            error: Self::calculate_error(&previous_state, &target_concepts),
            connections_map,
            target_concepts,
            dynamic_model,
            previous_state,
            delta_state,
            connections,
        }
    }
    pub fn get_max_model_time(&self) -> i32 {
        self.max_model_time
    }
    pub fn get_current_time(&self) -> i32 {
        self.current_time
    }
    pub fn get_error(&self) -> f64 {
        self.error
    }
    pub fn get_state(&self) -> HashMap<i32, f64> {
        self.previous_state.clone()
    }
    fn execute_next_value(
        &self,
        current_state: &mut HashMap<i32, f64>,
        concept_id: i32,
        dynamic_model: &DynamicModel,
        to_connections: &[(i32, f64)],
    ) -> () {
        let current_value = current_state.get_mut(&concept_id).unwrap();
        match dynamic_model {
            DynamicModel::DeltaDelta => {
                *current_value += Self::normalize_value(
                    to_connections
                        .iter()
                        .map(|(source_id, value)| value * self.delta_state[&source_id])
                        .sum::<f64>(),
                );
            }
            DynamicModel::DeltaValue => {
                *current_value += Self::normalize_value(
                    to_connections
                        .iter()
                        .map(|(source_id, value)| value * self.previous_state[&source_id])
                        .sum::<f64>(),
                );
            }
            DynamicModel::ValueDelta => {
                *current_value = Self::normalize_value(
                    to_connections
                        .iter()
                        .map(|(source_id, value)| value * self.delta_state[&source_id])
                        .sum::<f64>(),
                );
            }
            DynamicModel::ValueValue => {
                *current_value = Self::normalize_value(
                    to_connections
                        .iter()
                        .map(|(source_id, value)| value * self.previous_state[&source_id])
                        .sum::<f64>(),
                );
            }
        };
    }
    fn calculate_delta_state(&self, current_state: &State) -> State {
        State::from_iter(
            current_state
                .iter()
                .map(|(k, v)| (*k, v - self.previous_state[k])),
        )
    }
    fn calculate_error(state: &HashMap<i32, f64>, target_concepts: &[Arc<Concept>]) -> f64 {
        target_concepts
            .iter()
            .map(|concept| (state[&concept.id] - concept.target_value.unwrap()).powf(2.0))
            .sum::<f64>()
    }
    fn normalize_value(value: f64) -> f64 {
        if value > 1.0 {
            return 1.0;
        }
        if value <= 0.0 {
            return 0.0;
        }
        value
    }
}

impl Iterator for TimeSimulation {
    type Item = TimeSimulationData;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_time > self.max_model_time {
            panic!("The current time must be less or equal to the max model time");
        }
        let mut current_state = self.previous_state.clone();
        for concept in self.concepts_map.values() {
            let dynamic_model = concept
                .dynamic_model
                .as_ref()
                .unwrap_or(&self.dynamic_model);
            let to_connections = self
                .connections_map
                .values()
                .filter(|connection| connection.target_id == concept.id)
                .map(|connection| match self.connections.get(&connection.id) {
                    Some(value) => (connection.source_id, *value),
                    None => (connection.source_id, connection.value),
                })
                .collect::<Vec<_>>();
            if to_connections.len() == 0 {
                continue;
            }
            self.execute_next_value(
                &mut current_state,
                concept.id,
                dynamic_model,
                &to_connections,
            )
        }
        self.delta_state = self.calculate_delta_state(&current_state);
        self.previous_state = current_state;
        self.error = Self::calculate_error(&self.previous_state, &self.target_concepts);
        self.current_time += 1;
        if self.current_time <= self.max_model_time {
            Some(TimeSimulationData {
                time: self.current_time,
                error: self.error,
                state: self.previous_state.clone(),
            })
        } else {
            None
        }
    }
}

impl Concept {
    fn generate_value(&self, rng: &mut ThreadRng) -> f64 {
        match &self.constraint {
            Some(constraint) => constraint.generate_value(rng),
            None => rng.gen_range(0.0..=1.0),
        }
    }
}

impl Connection {
    fn generate_value(&self, rng: &mut ThreadRng) -> f64 {
        match &self.constraint {
            Some(constraint) => constraint.generate_value(rng),
            None => {
                if self.value >= 0.0 {
                    rng.gen_range(0.0..=1.0)
                } else {
                    rng.gen_range(-1.0..=0.0)
                }
            }
        }
    }
}

const SIGNIFICANT_DIFF: f64 = 0.0000001;

impl Constraint {
    fn generate_value(&self, rng: &mut ThreadRng) -> f64 {
        let mut number = rng.gen_range(self.min_value..=self.max_value);
        let mut attempts = 0;
        while !self.include_min_value && number == self.min_value
            || !self.include_max_value && number == self.max_value
        {
            number = rng.gen_range(self.min_value..=self.max_value);
            attempts += 1;
            if attempts >= 1000 {
                return (self.min_value + self.max_value) / 2.0;
            }
        }
        number
    }
    fn get_min(&self, min: f64) -> f64 {
        f64::max(
            min,
            if self.include_min_value {
                self.min_value
            } else {
                self.min_value + SIGNIFICANT_DIFF
            },
        )
    }
    fn get_max(&self, max: f64) -> f64 {
        f64::min(
            max,
            if self.include_max_value {
                self.max_value
            } else {
                self.max_value - SIGNIFICANT_DIFF
            },
        )
    }
}

type State = HashMap<i32, f64>;
