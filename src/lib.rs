use common_game::components::planet::{PlanetAI, PlanetState};
use common_game::components::resource::BasicResourceType::*;
use common_game::components::resource::ComplexResourceRequest::{
    AIPartner, Diamond, Dolphin, Life, Robot, Water as WaterRequest,
};
use common_game::components::resource::{
    BasicResource, Combinator, ComplexResource, ComplexResourceType, Generator, GenericResource,
};
use common_game::components::rocket::Rocket;
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use std::collections::HashSet;

pub struct AI;
impl PlanetAI for AI {
    fn handle_asteroid(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> Option<Rocket> {
        None
    }
    fn handle_orchestrator_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        match msg {
            OrchestratorToPlanet::Asteroid(_) => Some(PlanetToOrchestrator::AsteroidAck {
                planet_id: state.id(),
                destroyed: false,
            }),
            OrchestratorToPlanet::Sunray(sunray) => Some(PlanetToOrchestrator::SunrayAck {
                planet_id: state.id(),
            }),
            OrchestratorToPlanet::InternalStateRequest => {
                Some(PlanetToOrchestrator::InternalStateResponse {
                    planet_id: state.id(),
                    planet_state: state.to_dummy(),
                })
            }
            _ => None,
        }
    }
    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        match msg {
            ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id } => {
                Some(PlanetToExplorer::AvailableEnergyCellResponse {
                    available_cells: state.cells_count() as u32,
                })
            }
            ExplorerToPlanet::SupportedCombinationRequest { explorer_id } => {
                Some(PlanetToExplorer::SupportedCombinationResponse {
                    combination_list: HashSet::from([ComplexResourceType::Water]),
                })
            }
            ExplorerToPlanet::SupportedResourceRequest { explorer_id } => {
                Some(PlanetToExplorer::SupportedResourceResponse {
                    resource_list: generator.all_available_recipes(),
                })
            }
            ExplorerToPlanet::GenerateResourceRequest {
                explorer_id,
                resource,
            } => {
                if let Some((cell, _)) = state.full_cell() {
                    //Useless cell matching
                    match resource {
                        Oxygen => Some(PlanetToExplorer::GenerateResourceResponse {
                            resource: {
                                match generator.make_oxygen(cell) {
                                    Ok(oxygen) => Some(BasicResource::Oxygen(oxygen)),
                                    Err(_) => None,
                                }
                            },
                        }),
                        Hydrogen => Some(PlanetToExplorer::GenerateResourceResponse {
                            resource: {
                                match generator.make_hydrogen(cell) {
                                    Ok(hydrogen) => Some(BasicResource::Hydrogen(hydrogen)),
                                    Err(_) => None,
                                }
                            },
                        }),
                        Carbon => Some(PlanetToExplorer::GenerateResourceResponse {
                            resource: {
                                match generator.make_carbon(cell) {
                                    Ok(carbon) => Some(BasicResource::Carbon(carbon)),
                                    Err(_) => None,
                                }
                            },
                        }),
                        Silicon => Some(PlanetToExplorer::GenerateResourceResponse {
                            resource: {
                                match generator.make_silicon(cell) {
                                    Ok(silicon) => Some(BasicResource::Silicon(silicon)),
                                    Err(_) => None,
                                }
                            },
                        }),
                    }
                } else {
                    Some(PlanetToExplorer::GenerateResourceResponse { resource: None })
                }
            }
            ExplorerToPlanet::CombineResourceRequest { explorer_id, msg } => match msg {
                WaterRequest(hydrogen, oxygen) => Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response: {
                        match combinator.make_water(hydrogen, oxygen, state.cell_mut(0)) {
                            Ok(water) => Ok(ComplexResource::Water(water)),
                            Err((s, hydrogen, oxygen)) => Err((
                                s,
                                GenericResource::BasicResources(BasicResource::Hydrogen(hydrogen)),
                                GenericResource::BasicResources(BasicResource::Oxygen(oxygen)),
                            )),
                        }
                    },
                }),
                Diamond(carbon1, carbon2) => Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response: Err((
                        String::from("Diamond receipe not available on this planet"),
                        GenericResource::BasicResources(BasicResource::Carbon(carbon1)),
                        GenericResource::BasicResources(BasicResource::Carbon(carbon2)),
                    )),
                }),
                Life(water, carbon) => Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response: Err((
                        String::from("Life receipe not available on this planet"),
                        GenericResource::ComplexResources(ComplexResource::Water(water)),
                        GenericResource::BasicResources(BasicResource::Carbon(carbon)),
                    )),
                }),
                Robot(silicon, life) => Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response: Err((
                        String::from("Robot receipe not available on this planet"),
                        GenericResource::BasicResources(BasicResource::Silicon(silicon)),
                        GenericResource::ComplexResources(ComplexResource::Life(life)),
                    )),
                }),
                Dolphin(water, life) => Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response: Err((
                        String::from("Dolphin receipe not available on this planet"),
                        GenericResource::ComplexResources(ComplexResource::Water(water)),
                        GenericResource::ComplexResources(ComplexResource::Life(life)),
                    )),
                }),
                AIPartner(robot, diamond) => Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response: Err((
                        String::from("AIPartner receipe not available on this planet"),
                        GenericResource::ComplexResources(ComplexResource::Robot(robot)),
                        GenericResource::ComplexResources(ComplexResource::Diamond(diamond)),
                    )),
                }),
            },
        }
    }
    fn start(&mut self, state: &PlanetState) {}
    fn stop(&mut self, state: &PlanetState) {}
}
