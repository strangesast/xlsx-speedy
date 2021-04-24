import os
from pathlib import Path
from openpyxl import load_workbook


def fn(filepath, sheet_name):
    # fucking slow without read_only
    wb = load_workbook(filename=filepath, read_only=True)
    ws = wb[sheet_name]

    for row in ws.rows:
        for cell in row:
            pass

    wb.close()


def test():
    FILEPATH = os.environ.get("FILEPATH") or Path.cwd() / "../schedule.xlsm"
    SHEET_NAME = "PRODUCTION SCHEDULE"

    fn(FILEPATH, SHEET_NAME)


if __name__ == "__main__":
    import timeit

    print(timeit.timeit("test()", setup="from __main__ import test", number=1))
