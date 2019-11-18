../model: dataset.json train.py
	rm -rf ../model/ && python train.py

dataset.json: $(wildcard ../dataset/entities/*.yml) $(wildcard ../dataset/intents/*.yml)
	snips-nlu generate-dataset en ../dataset/entities/*.yml ../dataset/intents/*.yml > dataset.json
