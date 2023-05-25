use pyo3::prelude::*;
use std::str::FromStr;
use std::fmt;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

enum Move {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "Rock" => Ok(Move::Rock),
            "Paper" => Ok(Move::Paper),
            "Scissors" => Ok(Move::Scissors),
            _ => Err(())
        }
    }
}

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        match rng.gen_range(0..=2) {
            0 => Move::Rock,
            1 => Move::Scissors,
            _ => Move::Paper
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Rock => write!(f, "Rock"),
            Move::Paper => write!(f, "Paper"),
            Move::Scissors => write!(f, "Scissors")
        }
    }
}

#[pyfunction]
fn rps_round(input: &str) -> PyResult<bool>{
    let player_move = Move::from_str(input).unwrap();
    let cpu_move: Move = rand::random();
    println!("CPU chose {}", cpu_move.to_string());
    match player_move {
        Move::Rock => match cpu_move {Move::Scissors => Ok(true), _ => Ok(false)},
        Move::Paper => match cpu_move {Move::Rock => Ok(true), _ => Ok(false)},
        Move::Scissors => match cpu_move {Move::Paper => Ok(true), _ => Ok(false)},
    }
}

// State Struct
#[pyclass]
struct Record {
    wins: u8,
    #[pyo3(get, set)]
    data: PyObject
}

#[pyfunction]
fn save_score(wins: u8, data: PyObject) -> PyResult<Record> {
    println!();
    Ok(Record {
        wins,
        data
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyrt(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Record>()?;
    m.add_function(wrap_pyfunction!(rps_round, m)?)?;
    m.add_function(wrap_pyfunction!(save_score, m)?)?;

    Ok(())
}