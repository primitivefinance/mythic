import argparse
import pandas as pd
from analysis.data_processor import DataProcessor
from analysis.visualization import Visualizer

def main():
    data = DataProcessor("data", {"timestamp": int, "value": float})
    dfs = data.import_csvs()

    viz = Visualizer(nrows=2, ncols=2, figsize=(16, 10))

    # For a 2x2 layout, you'll specify the row and col index where you want each plot.
    viz.plot_statistical(0, 0, x1, y1, "Label 1")
    viz.plot_statistical(0, 1, x2, y2, "Label 2")
    viz.plot_statistical(1, 0, x3, y3, "Label 3")

    viz.save("output.png")


if __name__ == "__main__":
    main()
