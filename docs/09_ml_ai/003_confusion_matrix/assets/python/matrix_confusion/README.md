## Setup

```powershell
uv init matrix_confusion --python 3.12
cd matrix_confusion
uv add --dev ipykernel
code .
```
## In VSCode

### Jupyter Notebook

```powershell
uv add pandas
uv add scikit-learn
uv add matplotlib
```

* Edit the jupyter notebook
* Run cells etc.


### Write Python code

Open a terminal inside VSCode

```powershell
uv run titanic.py
```

## Running `titanic.py` outside VSCode

Open a terminal in the directory containing `titanic.py`.

```powershell
Set-Env .
python .\titanic.py
```

## Fix CVE-2026-49855
19/06/26

```powershell
cd docs\09_ml_ai\003_confusion_matrix\assets\python\matrix_confusion
uv lock --upgrade-package tornado
```