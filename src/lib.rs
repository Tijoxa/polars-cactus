use pyo3::prelude::*;
use pyo3_polars::PolarsAllocator;

pub mod tools;
mod expressions;

#[global_allocator]
static ALLOC: PolarsAllocator = PolarsAllocator::new();

#[pymodule]
mod _core {
    use std::cmp;

    use pyo3::prelude::*;

    use crate::tools::{arrays, card_and_repr::card_to_repr, eval_5cards};

    #[pyfunction]
    fn __version__() -> PyResult<String> {
        Ok(env!("CARGO_PKG_VERSION").to_string())
    }

    #[pyfunction]
    pub fn evaluate(
        card1: Option<&str>,
        card2: Option<&str>,
        card3: Option<&str>,
        card4: Option<&str>,
        card5: Option<&str>,
    ) -> PyResult<u32> {
        let c1: u32 = card_to_repr(card1).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c2: u32 = card_to_repr(card2).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c3: u32 = card_to_repr(card3).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c4: u32 = card_to_repr(card4).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c5: u32 = card_to_repr(card5).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;

        let result: u32 = eval_5cards(c1, c2, c3, c4, c5);

        Ok(result)
    }

    #[pyfunction]
    pub fn evaluate_7(
        card1: Option<&str>,
        card2: Option<&str>,
        card3: Option<&str>,
        card4: Option<&str>,
        card5: Option<&str>,
        card6: Option<&str>,
        card7: Option<&str>,
    ) -> PyResult<u32> {
        let c1: u32 = card_to_repr(card1).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c2: u32 = card_to_repr(card2).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c3: u32 = card_to_repr(card3).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c4: u32 = card_to_repr(card4).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c5: u32 = card_to_repr(card5).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c6: u32 = card_to_repr(card6).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;
        let c7: u32 = card_to_repr(card7).ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid card"))?;

        let cards: [u32; 7] = [c1, c2, c3, c4, c5, c6, c7];

        let mut result: u32 = 7462;

        for [id1, id2, id3, id4, id5] in arrays::PERM7 {
            let tmp_result: u32 = eval_5cards(cards[*id1], cards[*id2], cards[*id3], cards[*id4], cards[*id5]);
            result = cmp::min(tmp_result, result);
        }

        Ok(result)
    }
}
