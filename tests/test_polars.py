import polars as pl
from polars.testing import assert_frame_equal

import polars_cactus as plc


def test_sha2_256_expression():
    # Test basic string hashing
    df = pl.DataFrame({"text": ["hello", "world"]})
    result = df.select(plc.col("text").chash.sha2_256())
    assert result.shape == (2, 1)
    assert result.dtypes == [pl.Utf8]

    # Test with concat_str
    df = pl.DataFrame({"a": ["foo"], "b": ["bar"]})
    result = df.select(plc.concat_str(["a", "b"]).chash.sha2_256())
    assert result.shape == (1, 1)
    assert result.dtypes == [pl.Utf8]

    # Test with null values
    df = pl.DataFrame({"text": ["hello", None, "world"]})
    result = df.select(plc.col("text").chash.sha2_256())
    assert result.shape == (3, 1)
    assert result.dtypes == [pl.Utf8]
    assert result["text"].is_null()[1]  # Second row should be null


def test_translate_card_to_repr():
    df = pl.DataFrame({"card": ["TH", "Z", "TT"]})
    result = df.select(plc.col("card").cactus.translate_card_to_repr())
    expected = pl.Series([16787479, None, None], dtype=pl.UInt32)
    assert_frame_equal(result, pl.DataFrame({"card": expected}))


def test_translate_repr_to_card():
    df = pl.DataFrame({"repr": pl.Series([16787479, 0, 1], dtype=pl.UInt32)})
    result = df.select(plc.col("repr").cactus.translate_repr_to_card())
    expected = pl.Series(["TH", None, None])
    assert_frame_equal(result, pl.DataFrame({"repr": expected}))


def test_evaluate_5cards():
    df = pl.DataFrame(
        {
            "cards": pl.Series(
                [
                    [8398611, 4204049, 2106637, 1057803, 16787479],
                    [268471337, 295429, 557831, 16812055, 268446761],
                ],
                dtype=pl.List(pl.UInt32),
            )
        }
    )
    result = df.select(plc.col("cards").cactus.evaluate_5cards())
    expected = pl.Series([5, 3484], dtype=pl.UInt32)
    assert_frame_equal(result, pl.DataFrame({"cards": expected}))


def test_evaluate_5cards_str():
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
    result = df.select(plc.col("cards").cactus.evaluate_5cards_str())
    expected = pl.Series([5, 3484], dtype=pl.UInt32)
    assert_frame_equal(result, pl.DataFrame({"cards": expected}))
