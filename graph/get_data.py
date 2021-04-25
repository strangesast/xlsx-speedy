import csv
from pathlib import Path
from subprocess import run, PIPE


def run_trials(name: str, trials=1):
    args = [str(cmdpath), name]
    for i in range(trials):
        p = run(args, stdout=PIPE)
        yield float(p.stdout.decode().strip())

if __name__ == '__main__':
    TRIALS = 10
    HEADER = ('folder', 'duration')
    data = []

    wd = Path(__file__).resolve().parent
    root = wd.parent
    cmdpath = root / 'run.sh'


    for path in root.iterdir():
        if path.is_dir() and (path / 'Dockerfile').is_file():
            trials = run_trials(path.name, TRIALS)
            data.extend(((path.name, f) for f in trials))
    
    with open(wd / 'data.csv', 'w') as f:
        writer = csv.writer(f)
        writer.writerow(HEADER)
        writer.writerows(data)
