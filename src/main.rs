use std::{env, fs, u8};

/// Serves to simulate the io stream constraint presented in the requirements.
struct FakeStreamer {
    next_read_idx: usize,
    data: Vec<char>,
}

impl FakeStreamer {
    /// Reads the next index provided in the structure and updates the counter.
    fn read_next_idx(&mut self) -> Option<char> {
        let read_idx = self.next_read_idx.clone();
        if self.data.len() > read_idx {
            self.next_read_idx = read_idx + 1;
            return Some(self.data[read_idx]);
        }
        return None;
    }
}

struct ProgramState {
    /// Provides the current pointer index from `data_ptrs`.
    cell_idx: usize,
    /// Holds the pointers created within the programs lifetime.
    data_ptrs: Vec<u8>,
    /// Emulates the stream mechanism constraint we require.
    stream: FakeStreamer,
    /// Captures standard output from the program.
    output: Vec<u8>,
}

impl ProgramState {
    /// Creates a new instance of the Brainfuck program.
    fn new(file_path: String) -> Result<Self, anyhow::Error> {
        Ok(ProgramState {
            stream: FakeStreamer {
                next_read_idx: 0,
                data: fs::read(file_path)?.iter().map(|&i| i as char).collect(),
            },
            cell_idx: 0,
            data_ptrs: vec![0],
            output: Vec::new(),
        })
    }

    /// Interpretes the Brainfuck program serially, save for the looping operation that has to re-evaluate previous streamed data.
    /// The stack condition for looping is achieved through recursion at '['
    fn execute(&mut self, loop_start_idx: usize, loop_counter_idx: usize) {
        self.stream.next_read_idx = loop_start_idx;
        self.cell_idx = loop_counter_idx;
        loop {
            match self.stream.read_next_idx() {
                Some(cmd) => match cmd {
                    '>' => {
                        self.data_ptrs.push(0);
                        self.cell_idx += 1;
                    }
                    '<' => self.cell_idx -= 1,
                    '+' => {
                        if self.data_ptrs[self.cell_idx] == u8::MAX {
                            self.data_ptrs[self.cell_idx] = u8::MIN;
                        } else {
                            self.data_ptrs[self.cell_idx] += 1;
                        }
                    }
                    '-' => {
                        if self.data_ptrs[self.cell_idx] == u8::MIN {
                            self.data_ptrs[self.cell_idx] = u8::MAX;
                        } else {
                            self.data_ptrs[self.cell_idx] -= 1;
                        }
                    }

                    '.' => self.output.push(self.data_ptrs[self.cell_idx]),
                    '[' => self.execute(self.stream.next_read_idx, self.cell_idx),
                    ']' => {
                        if self.data_ptrs[loop_counter_idx] == 0 {
                            return;
                        }
                        self.stream.next_read_idx = loop_start_idx;
                        self.cell_idx = loop_counter_idx;
                    }
                    _ => {}
                },
                None => return,
            }
        }
    }
}

#[test]
fn test_hello_world() {
    let mut hello_world = ProgramState::new("./bf_samples/hello_world.bf".to_string()).unwrap();
    hello_world.execute(0, 0);

    assert_eq!(
        "Hello World!",
        hello_world
            .output
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
    );
}

#[test]
fn test_add_2_5() {
    let mut add_2_5 = ProgramState::new("./bf_samples/add_2_5.bf".to_string()).unwrap();
    add_2_5.execute(0, 0);

    assert_eq!(
        "7",
        add_2_5
            .output
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
    );
}

fn main() {
    // takes filenames as arg and runs them as output
    let args: Vec<String> = env::args().collect();

    for i in args.iter().skip(1) {
        let program_res = ProgramState::new(i.clone());
        match program_res {
            Ok(mut program) => {
                program.execute(0, 0);
                println!(
                    "Program: {:?} output: {:?}",
                    i,
                    program
                        .output
                        .iter()
                        .map(|&b| b as char)
                        .collect::<String>()
                );
            }
            Err(err) => {
                println!("Error occured for program: {:?} \n Error: {:?}", i, err);
            }
        }
    }
}
