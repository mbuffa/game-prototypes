use crate::game_state::{GameState, GameStateUpdateResult, Player, Stage};
use crate::world_definition::{Spell, SpellEffectType, WorldDefinition};

pub fn sanitize_input(input: String) -> String {
    input.trim().to_owned()
}

pub fn validate_input(definition: &WorldDefinition, input: &String) -> bool {
    crate::debug!("{:?}", definition.get_spell_types().keys());

    definition.get_spell_types().keys().any(|k| k == input)
}

pub fn execute_input(
    definition: &WorldDefinition,
    game_state: &mut GameState,
    input: &String,
) -> GameStateUpdateResult {
    let spell = definition.get_spell_types().get(&input[..]);

    game_state.get_scene_mut().update_with(spell)
}

pub fn maybe_go_to_next_stage(game_state: &mut GameState) {
    let scene = game_state.get_scene_mut();

    match scene.get_current_stage() {
        Some(stage) => {
            if stage.are_all_dead() {
                scene.go_to_next_stage();
            }
        }
        None => {}
    }
}

pub fn maybe_mark_game_as_over_or_won(game_state: &mut GameState) {
    if game_state.get_scene().is_game_won() {
        game_state.set_is_won(true);
        game_state.set_is_over(true);
    }

    if game_state.get_scene().is_game_over() {
        game_state.set_is_over(true);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game_state::{GameState, GameStateUpdateResult},
        world_definition::WorldDefinition,
    };

    #[test]
    fn init_asserts() {
        let mut definitions = WorldDefinition::new();
        let mut game_state = GameState::new();
        definitions.initialize_spell_types();
        game_state.initialize();

        match game_state.get_scene().get_current_stage() {
            Some(_) => assert!(true),
            None => assert!(false, "Invalid initial current stage"),
        }

        // Add spell assertions
    }

    #[test]
    fn sanitize_input_asserts() {
        let input = "  Wo lo lo   \n".to_owned();
        let expected = "Wo lo lo".to_owned();
        assert_eq!(super::sanitize_input(input), expected);
    }

    #[test]
    fn validate_input_asserts() {
        let mut definitions = WorldDefinition::new();
        definitions.initialize_spell_types();

        assert_eq!(
            super::validate_input(&definitions, &"wololo".to_owned()),
            true
        );
        assert_eq!(
            super::validate_input(&definitions, &"awo you you".to_owned()),
            true
        );
        assert_eq!(
            super::validate_input(&definitions, &"avada kedavra".to_owned()),
            false
        );
    }

    #[test]
    fn execute_input_asserts() {
        let mut definitions = WorldDefinition::new();
        let mut game_state = GameState::new();
        definitions.initialize_spell_types();
        game_state.initialize();

        assert_eq!(
            super::execute_input(&definitions, &mut game_state, &"wololo".to_owned()),
            GameStateUpdateResult::NextTurn
        );

        assert_eq!(game_state.is_over(), false);
        assert_eq!(game_state.is_won(), false);
        assert_eq!(
            super::execute_input(&definitions, &mut game_state, &"awo you you".to_owned()),
            GameStateUpdateResult::NextStage
        );
    }

    #[test]
    fn is_game_over_asserts() {
        let mut definitions = WorldDefinition::new();
        let mut game_state = GameState::new();
        definitions.initialize_spell_types();
        game_state.initialize();

        assert_eq!(
            super::execute_input(&definitions, &mut game_state, &"awo you you".to_owned()),
            GameStateUpdateResult::NextStage
        );

        super::maybe_mark_game_as_over_or_won(&mut game_state);

        // FIXME: This is failing because I added stages.
        // To fix this, we should be able to specify stages at startup, or load
        // a file containing stages.
        assert_eq!(game_state.is_over(), true);
        assert_eq!(game_state.is_won(), true);
    }
}
