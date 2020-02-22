import altair as alt
import pandas as pd

data = pd.read_csv("./src/data.csv")
alt.Chart(data).mark_bar().encode(x="index", y="val").serve()
