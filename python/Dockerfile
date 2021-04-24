ARG tag=latest
from python:${tag}

workdir /app
copy requirements.txt .
run pip install openpyxl
#run pip install -r requirements.txt
copy main.py .

cmd ["python", "main.py"]
