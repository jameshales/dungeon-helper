import io
import json
from snips_nlu import SnipsNLUEngine

with io.open("dataset.json") as f:
    dataset = json.load(f)

engine = SnipsNLUEngine()

engine.fit(dataset)

engine.persist("../model")
