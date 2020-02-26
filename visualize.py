import altair as alt
import pandas as pd

data = pd.read_csv("./src/data.csv", header=None, index_col=False).rename(columns={0: "x", 1: "y"})
alt.Chart(data).mark_point(opacity=0.3).encode(x="x", y="y").serve()
