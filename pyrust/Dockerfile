from rust as builder

run apt-get update && apt-get install -y --no-install-recommends \
  build-essential \
  gcc \
  python3 \
  python3-pip \
  python3-dev \
  python3-setuptools

workdir /usr/src/app

copy requirements.txt .
run python3 -m pip install -r requirements.txt

copy . .
run python3 setup.py install

workdir /
copy test.py .

cmd ["python3", "/test.py"]
