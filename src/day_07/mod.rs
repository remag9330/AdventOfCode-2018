use util;

pub fn run_part_1(args: &[String]) {
    util::run_part_n("1", args, determine_steps);
}

fn determine_steps(filename: &String) -> util::AppResult {
    let processes = read_steps(filename)?;
    let result = create_steps_list(&processes);

    println!("Steps: {}", result);


    Ok(())
}

fn create_steps_list(steps: &Vec<Process>) -> String {
    let mut result = String::new();

    let mut steps_done = Vec::new();
    while steps.len() != steps_done.len() {
        let doable = find_available_steps(steps, &steps_done);
        let next_step = determine_next_step_alphabetically(&doable);
        result.push(next_step);
        steps_done.push(next_step);
    }

    result
}

fn find_available_steps(steps: &Vec<Process>, done: &Vec<char>) -> Vec<char> {
    steps.iter()
        .filter(|p| !done.contains(&p.id))
        .filter(|p| dependencies_fulfilled(&p.dependencies, done))
        .map(|p| p.id)
        .collect::<Vec<char>>()
}

fn dependencies_fulfilled(deps: &Vec<char>, done: &Vec<char>) -> bool {
    for dep in deps.iter() {
        if !done.contains(dep) {
            return false;
        }
    }

    true
}

fn determine_next_step_alphabetically(options: &Vec<char>) -> char {
    let mut result = 'Z';

    for c in options.iter() {
        result = std::cmp::min(*c, result);
    }

    result
}

fn read_steps(filename: &String) -> Result<Vec<Process>, util::AppError> {
    let input = util::read_file_input(filename)?;
    Ok(parse_input(&input))
}

fn parse_input(s: &str) -> Vec<Process> {
    let mut map = std::collections::HashMap::new();

    for line in s.lines() {
        let d = line.split(' ').collect::<Vec<&str>>();
        match d.as_slice() {
            ["Step", x, "must", "be", "finished", "before", "step", y, "can", "begin."] => {
                let key = y.chars().next().unwrap();
                let dep = x.chars().next().unwrap();

                map.entry(key).or_insert_with(|| Process::new(key)).add_dependency(dep);
                map.entry(dep).or_insert_with(|| Process::new(dep));
            },
            _ => panic!("Invalid input!")
        };
    }

    let mut results = Vec::new();

    for value in map.values() {
        results.push(value.clone());
    }

    results
}

#[derive(Clone, Debug)]
struct Process {
    id: char,
    dependencies: Vec<char>,
}

impl Process {
    fn new(id: char) -> Process {
        Process{
            id,
            dependencies: Vec::new(),
        }
    }

    fn add_dependency(&mut self, new_dep: char) {
        self.dependencies.push(new_dep);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dependency_input() {
        let input = get_input();

        println!("{:?}", input);

        assert_eq!(6, input.len());

        let c = get_process(&input, 'C').unwrap();
        assert_eq!(0, c.dependencies.len());

        let e = get_process(&input, 'E').unwrap();
        assert_eq!(3, e.dependencies.len());
        assert!(e.dependencies.contains(&'B'));
        assert!(e.dependencies.contains(&'D'));
        assert!(e.dependencies.contains(&'F'));
    }

    #[test]
    fn test_create_steps_list() {
        let input = get_input();
        let result = create_steps_list(&input);
        assert_eq!("CABDFE", result);
    }

    fn get_process(processes: &Vec<Process>, id: char) -> Option<&Process> {
        for p in processes.iter() {
            if p.id == id {
                return Some(p);
            }
        }

        None
    }

    fn get_input() -> Vec<Process> {
        let s: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        parse_input(s)
    }
}
