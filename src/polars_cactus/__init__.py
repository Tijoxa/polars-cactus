from __future__ import annotations

from pathlib import Path
from typing import Iterable, Protocol, cast

import polars as pl
from polars._typing import IntoExpr, PolarsDataType
from polars.plugins import register_plugin_function

from polars_cactus._core import __version__, evaluate, evaluate_7


@pl.api.register_expr_namespace("chash")
class CryptographicHashingNameSpace:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def sha2_256(self) -> pl.Expr:
        """Takes Utf8 as input and returns utf8 hash with sha256 from SHA-2 family."""
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            function_name="sha2_256",
            args=self._expr,
            is_elementwise=True,
        )


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
    def chash(self) -> CryptographicHashingNameSpace:
        return CryptographicHashingNameSpace(self)

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
    def chash(self) -> CactusColumn: ...

    @property
    def translate_card_to_repr(self) -> CactusColumn: ...

    @property
    def translate_repr_to_card(self) -> CactusColumn: ...

    @property
    def evaluate_5cards_repr(self) -> CactusColumn: ...

    @property
    def evaluate_5cards(self) -> CactusColumn: ...


class CactusConcatStr(Protocol):
    def __call__(
        self,
        exprs: IntoExpr | Iterable[IntoExpr],
        *more_exprs: IntoExpr,
        separator: str = "",
        ignore_nulls: bool = False,
    ) -> CExpr: ...

    def __getattr__(self, name: str) -> pl.Expr: ...

    @property
    def chash(self) -> CactusEvaluator: ...


col = cast(CactusColumn, pl.col)
concat_str = cast(CactusConcatStr, pl.concat_str)


__all__ = ["col", "concat_str", "__version__", "evaluate", "evaluate_7"]
