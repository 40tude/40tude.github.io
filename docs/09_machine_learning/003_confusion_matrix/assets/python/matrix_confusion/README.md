## Setup

```powershell
uv init matrix_confusion --python 3.12
cd matrix_confusion
uv add --dev ipykernel
code .

uv add pandas
uv add scikit-learn
uv add matplotlib
```