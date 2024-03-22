use convert_case::{Case, Casing};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub struct CategoryError;

impl fmt::Display for CategoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Category must be one of: pokemon, item, berry, egg-group, ability, move, machine"
        )
    }
}

#[derive(Debug)]
pub enum Category {
    Pokemon,
    Item,
    Berry,
    EggGroup,
    Ability,
    Move,
    Machine,
}

impl FromStr for Category {
    type Err = CategoryError;
    fn from_str(category: &str) -> Result<Self, Self::Err> {
        match category {
            "pokemon" => Ok(Category::Pokemon),
            "item" => Ok(Category::Item),
            "berry" => Ok(Category::Berry),
            "egg-group" => Ok(Category::EggGroup),
            "ability" => Ok(Category::Ability),
            "move" => Ok(Category::Move),
            "machine" => Ok(Category::Machine),
            _ => Err(CategoryError),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut value = format!("{:?}", self);
        value = value.as_str().from_case(Case::Pascal).to_case(Case::Kebab);
        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::Category;
    use pretty_assertions::assert_eq;
    use std::matches;
    use std::str::FromStr;

    #[test]
    fn test_category_from_str() {
        assert!(matches!(
            Category::from_str("pokemon"),
            Ok(Category::Pokemon)
        ));
        assert!(matches!(Category::from_str("item"), Ok(Category::Item)));
        assert!(matches!(Category::from_str("berry"), Ok(Category::Berry)));
        assert!(matches!(
            Category::from_str("egg-group"),
            Ok(Category::EggGroup)
        ));
        assert!(matches!(
            Category::from_str("ability"),
            Ok(Category::Ability)
        ));
        assert!(matches!(Category::from_str("move"), Ok(Category::Move)));
        assert!(matches!(
            Category::from_str("machine"),
            Ok(Category::Machine)
        ));
    }

    #[test]
    fn test_category_display() {
        assert_eq!(Category::Pokemon.to_string(), "pokemon");
        assert_eq!(Category::Item.to_string(), "item");
        assert_eq!(Category::Berry.to_string(), "berry");
        assert_eq!(Category::EggGroup.to_string(), "egg-group");
        assert_eq!(Category::Ability.to_string(), "ability");
        assert_eq!(Category::Move.to_string(), "move");
        assert_eq!(Category::Machine.to_string(), "machine");
    }

    #[test]
    fn test_category_from_str_error() {
        assert!(Category::from_str("error").is_err());
    }

    #[test]
    fn test_category_debug() {
        assert_eq!(format!("{:?}", Category::Pokemon), "Pokemon");
        assert_eq!(format!("{:?}", Category::Item), "Item");
        assert_eq!(format!("{:?}", Category::Berry), "Berry");
        assert_eq!(format!("{:?}", Category::EggGroup), "EggGroup");
        assert_eq!(format!("{:?}", Category::Ability), "Ability");
        assert_eq!(format!("{:?}", Category::Move), "Move");
        assert_eq!(format!("{:?}", Category::Machine), "Machine");
    }

    #[test]
    fn test_category_error_display() {
        assert_eq!(
            format!("{}", super::CategoryError),
            "Category must be one of: pokemon, item, berry, egg-group, ability, move, machine"
        );
    }

    #[test]
    fn test_category_error_debug() {
        assert_eq!(
            format!("{:?}", super::CategoryError),
            "CategoryError"
        );
    }

    #[test]
    fn test_category_error_clone() {
        let error = super::CategoryError;
        let _cloned_error = error.clone();
    }
}
