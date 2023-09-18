import { readLines } from '../helpers';

const ASCII_LOWER: string[] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

const challenge_1 = async (): Promise<number> => {
  const duplicate_letters: string[] = []
  await readLines((line: string) => {
    for (const letter of line.substring(0, line.length / 2).split('')) {
      if (line.substring(line.length / 2, line.length).split('').indexOf(letter) > -1) {
        duplicate_letters.push(letter)
        break;
      }
    }
  });
  return get_item_score(duplicate_letters);
}

const get_item_score = (items: string[]): number => {
  return items.map(letter => ASCII_LOWER.indexOf(letter.toLowerCase()) + (/[a-z]/.test(letter) ? 1 :  27)).reduce((p, v) => p + v)
}

const challenge_2 = async (): Promise<number> => {
  let groups: [string[]?] = [];
  let index = 0
  let badges = []
  await readLines((line: string) => {
    if (!groups[index]) {
      groups.push([line])
    } else {
      groups[index].push(line)
    }
    if (groups[index].length === 3) index++

  })
  groups.forEach(group => {
    for (const letter of group[0].split('')) {
        if (group[1].split('').indexOf(letter) > -1 && group[2].split('').indexOf(letter) > -1) {
          badges.push(letter)
          break
        }
      }
  })
  const score = badges.map(letter => ASCII_LOWER.indexOf(letter.toLowerCase()) + (/[a-z]/.test(letter) ? 1 :  27)).reduce((p, v) => p + v)
  return get_item_score(badges)
}

challenge_2()
