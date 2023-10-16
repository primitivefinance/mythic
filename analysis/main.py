import argparse
import pandas as pd
from data_processor import DataProcessor
from visualization import Visualizer
from decimal import Decimal

def main():

    parser = argparse.ArgumentParser(description='Run different types of visualizations.')
    parser.add_argument('--type', type=str, choices=['test'], required=True,
                        help='Type of visualization to run')

    args = parser.parse_args()
    if args.type == 'test':
        data = DataProcessor("test_data/lex", {"price": Decimal})
        df = data.import_csv("PriceChangeFilter.csv")
        print(df)

        viz = Visualizer(nrows=1, ncols=1, figsize=(16, 10))
        viz.plot(row=0, col=0, x_data=df.index.to_list(), y_data=df["price"], label="Price")

        viz.save("test_data/test_output.png")
        print("Saved test output to `analysis/test_data/test_output.png`")
    else:
        print("Invalid type")


if __name__ == "__main__":
    main()
