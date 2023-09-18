import fs from 'node:fs'
import readline from 'node:readline'
import events from 'node:events';

export const readLines = async (listener) => {
  try {
    const rl = readline.createInterface({
      input: fs.createReadStream('input')
    })

    rl.on('line', (line: string) => {
      listener(line)
    })

    await events.once(rl, 'close')

    console.log('Reading file line by line with readline done.');
    const used = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`The script uses approximately ${Math.round(used * 100) / 100} MB`);
  } catch (err) {
    console.error(err);
  }
}
