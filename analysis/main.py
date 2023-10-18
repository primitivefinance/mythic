import argparse
import pandas as pd
from data_processor import DataProcessor
from visualization import Visualizer
from decimal import Decimal
import os

def main():

    parser = argparse.ArgumentParser(description='Run different types of visualizations.')
    parser.add_argument('--type', type=str, choices=['test'], required=True,
                        help='Type of visualization to run')

    args = parser.parse_args()
    if args.type == 'test':
        data = DataProcessor("test_data", {"price": Decimal, "new_price": Decimal})
        dfs = data.import_csvs()

        x_data = [dfs[os.path.join("lex", "PriceChangeFilter")].index.to_list(), dfs[os.path.join("g3m", "SwapFilter")].index.to_list()]
        y_data = [dfs[os.path.join("lex", "PriceChangeFilter")]["price"], dfs[os.path.join("g3m", "SwapFilter")]["new_price"]]
        labels = ["PriceChangeFilter", "SwapFilter"]

        viz = Visualizer(nrows=1, ncols=1, figsize=(16, 10))
        viz.plot(row=0, col=0, x_data=x_data, y_data=y_data, labels=labels)

        viz.save("test_data/test_output.png")
        print("Saved test output to `analysis/test_data/test_output.png`")
    else:
        print("Invalid type")


if __name__ == "__main__":
    main()
