use crate::config::Color;

pub trait Creator {
    fn builder(vec: Vec<Vec<Color>>) -> String;
}
