from __future__ import annotations

from pathlib import Path
from typing import Iterable, Protocol, cast

import polars as pl
from polars._typing import PolarsDataType
from polars.plugins import register_plugin_function

from polars_cactus._core import __version__, evaluate, evaluate_7


@pl.api.register_expr_namespace("cactus")
class CactusEvaluator:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def translate_card_to_repr(self) -> pl.Expr:
        """Takes a string as input and returns a u32 that represents the card."""
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            function_name="translate_card_to_repr",
            args=self._expr,
            is_elementwise=True,
        )

    def translate_repr_to_card(self) -> pl.Expr:
        """Takes a u32 as input and returns a string that represents the card."""
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            function_name="translate_repr_to_card",
            args=self._expr,
            is_elementwise=True,
        )

    def evaluate_5cards_repr(self) -> pl.Expr:
        """Takes a list of u32 as input and returns a u32 that represents the hand."""
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            function_name="evaluate_5cards_repr",
            args=self._expr,
            is_elementwise=True,
        )

    def evaluate_5cards(self) -> pl.Expr:
        """Takes a list of str as input and returns a u32 that represents the hand."""
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            function_name="evaluate_5cards",
            args=self._expr,
            is_elementwise=True,
        )


class CExpr(pl.Expr):
    @property
    def cactus(self) -> CactusEvaluator:
        return CactusEvaluator(self)


class CactusColumn(Protocol):
    def __call__(
        self,
        name: str | PolarsDataType | Iterable[str] | Iterable[PolarsDataType],
        *more_names: str | PolarsDataType,
    ) -> CExpr: ...

    def __getattr__(self, name: str) -> pl.Expr: ...

    @property
    def translate_card_to_repr(self) -> CactusColumn: ...

    @property
    def translate_repr_to_card(self) -> CactusColumn: ...

    @property
    def evaluate_5cards_repr(self) -> CactusColumn: ...

    @property
    def evaluate_5cards(self) -> CactusColumn: ...


col = cast(CactusColumn, pl.col)


__all__ = ["col", "__version__", "evaluate", "evaluate_7"]
