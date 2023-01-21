use crate::game_state::{GameState, GameStateUpdateResult, Player, Stage};
use crate::world_definition::{Spell, SpellEffectType, WorldDefinition};

// Draw functions

pub fn draw_ui() {}

pub fn draw_scene() {}

// Update functions

pub fn sanitize_input(input: &str) -> &str {
    input.trim()
}

pub fn validate_input(definition: &WorldDefinition, input: &str) -> bool {
    definition.get_spell_types().keys().any(|k| k == input)
}

pub fn execute_input(
    definition: &WorldDefinition,
    game_state: &mut GameState,
    input: &str,
) -> GameStateUpdateResult {
    let spell = definition.get_spell_types().get(input);

    game_state.get_scene_mut().update_with(spell)
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
        let input = "  Wo lo lo   \n";
        let expected = "Wo lo lo";
        assert_eq!(super::sanitize_input(input), expected);
    }

    #[test]
    fn validate_input_asserts() {
        let mut definitions = WorldDefinition::new();
        // let mut game_state = GameState::new();
        definitions.initialize_spell_types();

        assert_eq!(super::validate_input(&definitions, "Wololo"), true);
        assert_eq!(super::validate_input(&definitions, "Awo you you"), true);
        assert_eq!(super::validate_input(&definitions, "Avada Kedavra"), false);
    }

    #[test]
    fn execute_input_asserts() {
        let mut definitions = WorldDefinition::new();
        let mut game_state = GameState::new();
        definitions.initialize_spell_types();
        game_state.initialize();

        assert_eq!(
            super::execute_input(&definitions, &mut game_state, "Wololo"),
            GameStateUpdateResult::NextTurn
        );

        assert_eq!(game_state.is_over(), false);
        assert_eq!(game_state.is_won(), false);
        assert_eq!(
            super::execute_input(&definitions, &mut game_state, "Awo you you"),
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
            super::execute_input(&definitions, &mut game_state, "Awo you you"),
            GameStateUpdateResult::NextStage
        );

        super::maybe_mark_game_as_over_or_won(&mut game_state);

        assert_eq!(game_state.is_over(), true);
        assert_eq!(game_state.is_won(), true);
    }
}
