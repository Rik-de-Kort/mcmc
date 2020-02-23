import altair as alt
import pandas as pd

data = pd.read_csv("./src/data.csv", index_col=False)
alt.Chart(data).mark_point().encode(x="x", y="y").serve()
