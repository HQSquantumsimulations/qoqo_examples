# Copyright © 2021 HQS Quantum Simulations GmbH.

"""Unit test Intro_to_qoqo."""

import pytest
import sys
import nbformat
from nbconvert.preprocessors import ExecutePreprocessor
from pathlib import Path


@pytest.mark.parametrize("name", [
    "1_Intro_to_qoqo",
    "2_Measurement_example",
    "3_Teleportation_example",
    "4_Half_adder_example",
    "5_Deutsch-Josza_example",
    "6_Simple_VHA_with_qoqo",
])
def test_running_notebook(name):
    src_path = Path(__file__)
    root_path = src_path.parents[2]

    with open(root_path / "qoqo/{}.ipynb".format(name)) as f:
        notebook = nbformat.read(f, as_version=4)

    preprocessor = ExecutePreprocessor(timeout=60)

    _ = preprocessor.preprocess(notebook)


# makes your pytest run
if __name__ == '__main__':
    pytest.main(sys.argv)
