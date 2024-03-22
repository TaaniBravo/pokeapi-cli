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
        value = value.to_lowercase();
        write!(f, "{}", value)
    }
}
