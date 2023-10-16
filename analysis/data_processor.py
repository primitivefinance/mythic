import pandas as pd
from decimal import Decimal
import glob
import os

def to_wad(x):
    if isinstance(x, str) and x.startswith("0x"):
        return Decimal(int(x, 16)).scaleb(-18)
    else: 
        return Decimal(x).scaleb(-18)

class DataProcessor:
    def __init__(self, dir, schema):
        self.dir = dir
        self.schema = schema

    def process_dataframe(self, df):
        # Apply the to_wad function only to columns in the schema with type Decimal
        for column, dtype in self.schema.items():
            if dtype == Decimal:
                df[column] = df[column].apply(to_wad)
        return df

    def import_csv(self, filename):
        filepath = os.path.join(self.dir, filename)
        try:
            df = pd.read_csv(filepath)
            return self.process_dataframe(df)
        except FileNotFoundError:
            print(f"File {filepath} not found.")
            return None
        except Exception as e:
            print(f"Error processing file {filepath}: {e}")
            return None

    def import_csvs(self):
        all_files = glob.glob(os.path.join(self.dir, "*.csv"))
        print(all_files)
        dfs = [self.process_dataframe(pd.read_csv(filename)) for filename in all_files]
        return dfs
