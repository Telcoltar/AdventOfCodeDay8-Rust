mod main_test;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};
use std::ops::{Add, AddAssign};
use std::fmt;

#[derive(Debug)]
enum Command {
    NOP,
    ACC,
    JMP
}

impl Command {
    pub fn parse(identifier: &str) -> Command{
        return match identifier {
            "nop" => Command::NOP,
            "acc" => Command::ACC,
            "jmp" => Command::JMP,
            _ => Command::NOP
        }
    }
}

#[derive(Debug)]
struct State {
    acc: i32,
    index: i32
}

impl State {
    pub fn new(index: i32, acc: i32) -> State {
        State {
            acc,
            index
        }
    }
}

impl Add for State {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            acc: self.acc + other.acc,
            index: self.index + other.index,
        }
    }
}

impl AddAssign for State {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            acc: self.acc + other.acc,
            index: self.index + other.index,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.acc, self.index)
    }
}

fn read_input_data(file_name: &str) -> Vec<(String, i32)> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let mut commands:Vec<(String, i32)> = Vec::new();

    for line in f.lines() {
        let line = line.unwrap();
        let mut command_number_split = line.split_whitespace();
        let command = command_number_split.next().unwrap().to_owned();
        let number = command_number_split.next().unwrap().parse::<i32>().unwrap();
        debug!("Command: {:?}, Number: {:?}", command, number);
        commands.push((command, number));
    }
    return commands;
}

fn read_input_data_enum(file_name: &str) -> Vec<(Command, i32)> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let mut commands:Vec<(Command, i32)> = Vec::new();

    for line in f.lines() {
        let line = line.unwrap();
        let mut command_number_split = line.split_whitespace();
        let command = Command::parse(command_number_split.next().unwrap());
        let number = command_number_split.next().unwrap().parse::<i32>().unwrap();
        debug!("Command: {:?}, Number: {:?}", command, number);
        commands.push((command, number));
    }
    return commands;
}

fn solution_part_1(file_name: &str) -> i32 {
    let commands = read_input_data(file_name);
    let mut visited_index: Vec<i32> = vec![0];
    let mut acc: i32 = 0;
    let mut index: i32 = 0;
    let mut next = &commands[0];
    match &next.0[..] {
        "nop" => index = 1,
        "acc" => {index = 1; acc+= next.1}
        "jmp" => index = next.1,
        _ => {}
    }
    debug!("Index: {}, {:?}", index, next);
    while !visited_index.contains(&index) {
        next = &commands[index as usize];
        debug!("Index: {}, {:?}", index, next);
        visited_index.push(index);
        match &next.0[..] {
            "nop" => index += 1,
            "acc" => {index += 1; acc+= next.1},
            "jmp" => index += next.1,
            _ => {}
        }
    }
    return acc
}

fn process_command_enum(command: &(Command, i32), replace: bool) -> State {
    if replace {
        return match &command.0 {
            Command::NOP => State::new(command.1, 0),
            Command::ACC => State::new(1, command.1),
            Command::JMP => State::new(1, 0),
        }
    }
    return match &command.0 {
        Command::NOP => State::new(1, 0),
        Command::ACC => State::new(1, command.1),
        Command::JMP => State::new(command.1, 0),
    }

}

fn solution_part_1_enum_command(file_name: &str) -> i32 {
    let commands = read_input_data_enum(file_name);
    let mut visited_index: Vec<i32> = vec![0];
    let mut state = State::new(0,0);
    let mut command = &commands[0];
    let mut result_command = process_command_enum(command, false);
    debug!("Index: {:?}, {:?}", state, result_command);
    state += result_command;
    while !visited_index.contains(&state.index) {
        command = &commands[state.index as usize];
        visited_index.push(state.index);
        result_command = process_command_enum(command, false);
        debug!("Index: {:?}, {:?}", state, result_command);
        state += result_command;
    }
    return state.acc
}

fn process_command(command: &(String, i32), replace: bool) -> State {
    if replace {
        return match &command.0[..] {
            "nop" => State::new(command.1, 0),
            "acc" => State::new(1, command.1),
            "jmp" => State::new(1, 0),
            _ => State::new(0, 0)
        }
    }
    return match &command.0[..] {
        "nop" => State::new(1, 0),
        "acc" => State::new(1, command.1),
        "jmp" => State::new(command.1, 0),
        _ => State::new(0, 0)
    }

}

fn loop_through_commands_with_replace(commands: &Vec<(String, i32)>, index: i32) -> (bool, i32) {
    let mut visited_index: Vec<i32> = vec![0];
    let mut command = &commands[0];
    let mut state: State = State {
        acc: 0, index: 0
    };
    let mut command_result = process_command(command, index == 0);
    debug!("State: {}, {:?}, {:?}", state, command, command_result);
    state += command_result;
    let commands_len: i32 = commands.len() as i32;
    let mut success_run = false;
    while !visited_index.contains(&state.index) {
        visited_index.push(state.index);
        command = &commands[state.index as usize];
        command_result = process_command(command, index == state.index);
        debug!("State: {}, {:?}, {:?}", state, command, command_result);
        state += command_result;
        if state.index >= commands_len {
            success_run = true;
            break;
        }
    }
    return (success_run, state.acc)
}

fn solution_part_2(file_name: &str) -> i32 {
    let commands = read_input_data(file_name);
    for i in 0..commands.len() {
        debug!("{}", i);
        let run_result = loop_through_commands_with_replace(&commands, i as i32);
        if run_result.0 {
            return run_result.1;
        }
    }
    return 0
}

fn main() {
    env_logger::init();
    info!("{}", solution_part_1("inputData.txt"));
    info!("{}", solution_part_1_enum_command("inputData.txt"));
    info!("{}", solution_part_2("inputData.txt"))
}
