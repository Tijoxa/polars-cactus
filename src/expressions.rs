use std::borrow::Cow;

use crate::tools::{
    card_and_repr::{card_to_repr, repr_to_card},
    eval_5cards,
};
use polars::{
    datatypes::{StringChunked, UInt32Chunked},
    error::PolarsResult,
    prelude::{ChunkApply, IntoSeries, arity::unary_elementwise},
    series::Series,
};
use pyo3_polars::derive::polars_expr;

#[polars_expr(output_type=String)]
fn sha2_256(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_values(|v| Cow::Owned(v.to_string()));
    Ok(out.into_series())
}

#[polars_expr(output_type=UInt32)]
fn translate_card_to_repr(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: UInt32Chunked = unary_elementwise(ca, card_to_repr);
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn translate_repr_to_card(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].u32()?;
    let out: StringChunked = unary_elementwise(ca, repr_to_card);
    Ok(out.into_series())
}

#[polars_expr(output_type=UInt32)]
fn evaluate_5cards_repr(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].list()?;

    let out: UInt32Chunked = ca
        .amortized_iter()
        .map(|opt_hand_series| {
            let s = opt_hand_series?;
            let ca_inner = s.as_ref().u32().ok()?;

            if ca_inner.len() != 5 {
                return None;
            }

            let c1 = ca_inner.get(0)?;
            let c2 = ca_inner.get(1)?;
            let c3 = ca_inner.get(2)?;
            let c4 = ca_inner.get(3)?;
            let c5 = ca_inner.get(4)?;

            Some(eval_5cards(c1, c2, c3, c4, c5))
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=UInt32)]
fn evaluate_5cards(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].list()?;

    let out: UInt32Chunked = ca
        .amortized_iter()
        .map(|opt_hand_series| {
            let s = opt_hand_series?;
            let ca_inner = s.as_ref().str().ok()?;

            if ca_inner.len() != 5 {
                return None;
            }

            let c1 = card_to_repr(Some(ca_inner.get(0)?))?;
            let c2 = card_to_repr(Some(ca_inner.get(1)?))?;
            let c3 = card_to_repr(Some(ca_inner.get(2)?))?;
            let c4 = card_to_repr(Some(ca_inner.get(3)?))?;
            let c5 = card_to_repr(Some(ca_inner.get(4)?))?;

            Some(eval_5cards(c1, c2, c3, c4, c5))
        })
        .collect();

    Ok(out.into_series())
}
