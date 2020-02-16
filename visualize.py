import altair as alt
import pandas as pd

data = pd.read_csv("./data.csv")
alt.Chart(data).mark_bar().encode(x=alt.X("val", bin=alt.Bin(maxbins=5000)), y="count()").serve()
