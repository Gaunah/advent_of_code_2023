fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn hash(string: &str) -> u8 {
    string
        .chars()
        .fold(0, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17))
}

fn part1(input: &str) -> u32 {
    input.trim().split(',').map(|step| hash(step) as u32).sum()
}

#[derive(Debug)]
struct Lens {
    label: String,
    focus: u8,
}

fn part2(input: &str) -> usize {
    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());
    input.trim().split(',').for_each(|step| {
        let pos = step
            .chars()
            .position(|ch| ch == '=' || ch == '-')
            .expect("Should contain '=' or '-'!");
        let (label, instruction) = step.split_at(pos);
        let mut instruction = instruction.chars();
        let label = label.to_string();
        let lens_box = boxes.get_mut(hash(&label) as usize).unwrap();
        match instruction.next().expect("Should not be empty!") {
            '=' => {
                let lens = Lens {
                    label,
                    focus: instruction
                        .next()
                        .expect("Should contain focus power!")
                        .to_digit(10)
                        .expect("Should be a ascii digit") as u8,
                };
                insert_lens(lens_box, lens);
            }
            '-' => remove_lens(lens_box, &label),
            _ => unreachable!(),
        }
    });

    get_total_focus_power(&boxes)
}

fn get_total_focus_power(boxes: &[Vec<Lens>; 256]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(i, lens_box)| {
            (i + 1)
                * lens_box
                    .iter()
                    .enumerate()
                    .map(|(slot, lens)| (slot + 1) * lens.focus as usize)
                    .sum::<usize>()
        })
        .sum()
}

fn insert_lens(lens_box: &mut Vec<Lens>, lens: Lens) {
    match lens_box.iter_mut().find(|l| l.label == lens.label) {
        Some(existing_lens) => *existing_lens = lens,
        None => lens_box.push(lens),
    };
}

fn remove_lens(lens_box: &mut Vec<Lens>, label: &str) {
    if let Some(idx) = lens_box.iter().position(|l| l.label == label) {
        lens_box.remove(idx);
    };
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_test() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 145);
    }
}
