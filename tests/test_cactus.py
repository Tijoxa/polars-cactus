import polars_cactus as plc


def test_polars_cactus_evaluate():
    assert plc.evaluate("AC", "4C", "5C", "TC", "AH") == 3484


def test_polars_cactus_evaluate_7():
    assert plc.evaluate_7("AC", "4C", "5C", "TC", "AH", "9H", "7S") == 3463
