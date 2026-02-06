# polars-cactus

Implementation of the Cactus Kev Perfect Hash algorithm.

# Installation

## From source

Requires [Rust](https://www.rust-lang.org/fr)

<pre>git clone https://github.com/Tijoxa/polars-cactus.git
cd polars-cactus
uv sync</pre>

## From PyPI (not yet)

<pre>uv add polars-cactus</pre>

# Usage

- Usage in Python:

```Python
import polars_cactus as plc

plc.evaluate("AC", "4C", "5C", "TC", "AH")
>>> 3484
plc.evaluate_7("AC", "4C", "5C", "TC", "AH", "9H", "7S")
>>> 3463
```

```Python
import polars as pl
import polars_cactus as plc

df = pl.DataFrame(
    {
        "cards": pl.Series(
            [
                ["9H", "8H", "7H", "6H", "TH"],
                ["AC", "4C", "5C", "TC", "AH"],
            ]
        )
    }
)
df.select(plc.col("cards").cactus.evaluate_5cards_str())
>>> pl.Series([5, 3484], dtype=pl.UInt32)
```

# Source

- [Cactus Kev Algorithm](https://suffe.cool/poker/evaluator.html)
- [Hash Tables](http://suffe.cool/poker/7462.html)
