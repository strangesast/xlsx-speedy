const fs = require('fs');
const path = require('path');
const XLSX = require('xlsx');

function fn(filepath, sheetName, cb) {
  fs.readFile(filepath, (err, buffer) => {
    if (err) {
      cb(err, null);
    } else {
      const wb = XLSX.read(buffer, { type: 'buffer' });
      const ws = wb.Sheets[sheetName];
      
      const range = XLSX.utils.decode_range(ws['!ref']);
      for(let j = range.s.r; j <= range.e.r; j++){
        for(let i=range.s.c; i<=range.e.c; i++){
          const pos = XLSX.utils.encode_cell({r: j, c: i});
          const cell = ws[pos];
          if( typeof cell !== 'undefined' ){
            // pass
          }
        }
      }
      cb(null, null);
    }
  });
}

function test(cb) {
  const FILEPATH = process.env.FILEPATH || path.join(__dirname, '../schedule.xlsm');
  const SHEET_NAME = 'PRODUCTION SCHEDULE'

  fn(FILEPATH, SHEET_NAME, cb);
}


function timeit(fn, cb) {
  const hrstart = process.hrtime();
  fn(function (err, _) {
    if (err) {
      cb(err, null);
    } else {
      const hrend = process.hrtime(hrstart)
      cb(null, hrend[0] + hrend[1] / 1e9);
    }
  });
}

if (require.main === module) {
  timeit(test, (err, res) => {
    if (err != null) {
      throw err;
    } else {
      process.stdout.write(res);
    }
  });
}
