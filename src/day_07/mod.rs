use crate::util;

pub fn run_part_1(args: &[String]) {
    util::run_part_n("1", args, determine_steps);
}

pub fn run_part_2(args: &[String]) {
    util::run_part_n("2", args, calculate_time);
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

fn calculate_time(filename: &String) -> util::AppResult {
    let processes = read_steps(filename)?;
    let available_workers = 5;
    let base_action_time = 60;

    let result = simulate_processes(&processes, available_workers, base_action_time);

    println!("Time taken: {}", result);

    Ok(())
}

fn simulate_processes(steps: &Vec<Process>, workers: i32, base_action_time: i32) -> i32 {
    let mut current_time = 0;

    let mut steps_done = Vec::new();
    let mut available_actions = find_available_steps(steps, &steps_done);
    let mut workers: Vec<(Option<char>, i32)> = vec![(None, 0); workers as usize];

    for i in 0..std::cmp::min(available_actions.len(), workers.len()) {
        workers[i].0 = Some(available_actions[i]);
        workers[i].1 = get_process(available_actions[i], steps).unwrap().duration(base_action_time);
    }

    available_actions.clear();

    while steps.len() != steps_done.len() {
        // println!("current_time: {:?}", current_time);
        // println!("steps_done: {:?}", steps_done);
        // println!("available_actions: {:?}", available_actions);
        // println!("workers: {:?}", workers);
        // println!("");

        for worker in workers.iter_mut() {
            if let Some(c) = worker.0 {
                worker.1 -= 1;

                if worker.1 <= 0 {
                    steps_done.push(c);
                    worker.0 = None;
                }
            }
        }

        let in_progress = &workers.iter().filter_map(|w| w.0).collect();
        let doable = find_available_unstarted_steps(steps, &steps_done, in_progress);
        for maybe_new in &doable {
            if !available_actions.contains(maybe_new) {
                available_actions.push(maybe_new.clone());
            }
        }

        for worker in workers.iter_mut() {
            if worker.0.is_none() && available_actions.len() > 0 {
                let next = determine_next_step_alphabetically(&available_actions);
                let process = get_process(next, steps).unwrap();
                let index = available_actions.iter().position(|x| *x == next).unwrap();
                available_actions.remove(index);
                worker.0 = Some(next);
                worker.1 = process.duration(base_action_time);
            }
        }

        current_time += 1;
    }

    current_time
}

fn get_process(id: char, processes: &Vec<Process>) -> Option<&Process> {
    processes.iter().filter(|p| p.id == id).next()
}

fn find_available_steps(steps: &Vec<Process>, done: &Vec<char>) -> Vec<char> {
    find_available_unstarted_steps_internal(steps, done, None)
}

fn find_available_unstarted_steps(steps: &Vec<Process>, done: &Vec<char>, started: &Vec<char>) -> Vec<char> {
    find_available_unstarted_steps_internal(steps, done, Some(started))
}

fn find_available_unstarted_steps_internal(steps: &Vec<Process>, done: &Vec<char>, started: Option<&Vec<char>>) -> Vec<char> {
    steps.iter()
        .filter(|p| !done.contains(&p.id) && !started.map(|s| s.contains(&p.id)).unwrap_or(true))
        .filter(|p| dependencies_fulfilled(&p.dependencies, done))
        .map(|p| p.id)
        .collect::<Vec<char>>()
}

fn dependencies_fulfilled(deps: &Vec<char>, done: &Vec<char>) -> bool {
    deps.iter().filter(|d| !done.contains(d)).any(|_| true)
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

    fn duration(&self, base_action_time: i32) -> i32 {
        base_action_time + (self.id as i32 - 'A' as i32) + 1
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

    #[test]
    fn test_simulate_processes() {
        let input = get_input();
        let workers = 2;
        let time = 0;

        assert_eq!(15, simulate_processes(&input, workers, time));
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
