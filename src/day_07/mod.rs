use util;

pub fn run_part_1(args: &[String]) {
    util::run_part_n("1", args, determine_steps);
}

fn determine_steps(filename: &String) -> util::AppResult {
    let graph = read_steps(filename)?;



    Ok(())
}

fn read_steps(filename: &String) -> Result<Vec<Process>, util::AppError> {
    let input = util::read_file_input(filename)?;

    let mut map = std::collections::HashMap::new();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        map.insert(c, Process::new(c));
    }

    for line in input.lines() {
        let d = line.split(' ').collect::<Vec<&str>>();
        match d.as_slice() {
            ["Step", x, "must", "be", "finished", "before", "step", y, "can", "begin."] => {
                let key = x.chars().next().unwrap();
                let dep = y.chars().next().unwrap();
                map.get_mut(&key).unwrap().add_dependency(dep);
            },
            _ => panic!("Invalid input!")
        };
    }

    let mut results = Vec::new();

    for value in map.values() {
        results.push(value.clone());
    }

    Ok(results)
}

#[derive(Clone)]
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
