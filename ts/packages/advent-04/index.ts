import { readLines } from '../helpers'

const challenge_1 = async () => {
  let one, two
  await readLines((line: string) => {
    const [section_1, section_2] = line.split(',')
    const section_1_range = { from: Number(section_1.split('-')[0]), to: Number(section_1.split('-')[1])}
    const section_2_range = { from: Number(section_2.split('-')[0]), to: Number(section_2.split('-')[1])}

    one = build_range(section_1_range.from, section_1_range.to)
    two = build_range(section_2_range.from, section_2_range.to)
  })
  console.log(one, two)
}

const build_range = (from: number, to: number): number[] => {
  const res = []
  for (let i = from; i <= to; i++) {
    res.push(i)
  }
  return res
}

challenge_1()
