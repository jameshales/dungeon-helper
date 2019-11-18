# Training a Snips NLU model

## Requirements

1.  [pyenv](https://github.com/pyenv/pyenv)
2.  Python 3.6.9 (Snips NLU dependencies require `libffi` to be installed locally)

    ```
    sudo dnf install bzip2-devel libffi-devel openssl-devel readline-devel sqlite-devel
    pyenv install 3.6.9
    pyenv local 3.6.9
    ```
3.  [pipenv](https://github.com/pypa/pipenv)
    ```
    pip install pipenv
    ```
4.  Snips NLU English model
    ```
    pipenv run snips-nlu download en
    ```

## Usage

To generate the trained model:

```
pipenv run make
```

The model is produced in `../model`
