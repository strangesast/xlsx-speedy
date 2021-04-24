const fs = require('fs/promises');
const path = require('path');
const XLSX = require('xlsx');

async function fn(filepath, sheetName) {
  const buffer = await fs.readFile(filepath);
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
}

async function test() {
  const FILEPATH = process.env.FILEPATH || path.join(__dirname, '../schedule.xlsm');
  const SHEET_NAME = 'PRODUCTION SCHEDULE'

  await fn(FILEPATH, SHEET_NAME);
}


async function timeit(fn) {
  const hrstart = process.hrtime();
  await fn();
  const hrend = process.hrtime(hrstart)
  return hrend[0] + hrend[1] / 1e9;
}

if (require.main === module) {
  timeit(test).then(t => console.log(t));
}
