import pandas as pd

data_dic = {
    "a": [1, 2, 3],
    "b": [3, 4, 5],
}

df = pd.DataFrame(data_dic)
df.to_csv("../test.csv", index=False)