# cactus_evaluator

Implementation of the Cactus Kev Perfect Hash algorithm.

# Installation

## From source

Requires [Rust](https://www.rust-lang.org/fr)

<pre>git clone https://github.com/Tijoxa/cactus_evaluator.git
cd cactus_evaluator
uv sync</pre>

## From PyPI

<pre>uv add cactus_evaluator</pre>

# Usage

- Usage in Python:

```Python
import cactus_evaluator
cactus_evaluator.evaluate('AC', '4C', '5C', 'TC', 'AH')
>>> 3484
cactus_evaluator.evaluate_7('AC', '4C', '5C', 'TC', 'AH', '9H', '7S')
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
- [PyO3](https://github.com/PyO3/pyo3)
