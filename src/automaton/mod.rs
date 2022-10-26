use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt::Write,
};

use crate::kbdwriter::{KbdMap, KbdWriter};

#[derive(Default, Debug)]
pub struct Automaton {
    pub states: BTreeMap<u32, State>,
    pub transition: BTreeMap<(u32, char), u32>,
    pub start_state: u32,
}
#[derive(Debug)]
pub struct State {
    pub state_num: u32,
    pub next: BTreeMap<u32, char>,
    pub label: String,
    pub accepting: Option<String>,
}

impl Automaton {
    pub fn json_codegen(&self, o: &mut String) -> std::fmt::Result {
        writeln!(o, "let automaton = [")?;
        for (i, (_, state)) in self.states.iter().enumerate() {
            let accepting = if let Some(mat) = &state.accepting {
                format!("\"{}\"", mat.clone().replace('\"', "\\\""))
            } else {
                "null".to_string()
            };
            write!(
                o,
                "    {{l: \"{}\", a: {}, ",
                state.label.replace('\"', "\\\""),
                accepting
            )?;
            // transitions
            write!(o, "t: ")?;
            let mut transitions = self
                .transition
                .iter()
                .filter(|((start, _), _)| *start == state.state_num)
                .peekable();
            if transitions.peek().is_some() {
                let mut add_comma = false;
                write!(o, "new Map([")?;
                for ((_, char), end) in transitions {
                    if add_comma {
                        write!(
                            o,
                            ",[\"{}\", {}]",
                            char.to_string().replace('\"', "\\\""),
                            end
                        )?;
                    } else {
                        write!(
                            o,
                            "[\"{}\", {}]",
                            char.to_string().replace('\"', "\\\""),
                            end
                        )?;
                        add_comma = true;
                    }
                }
                write!(o, "])")?;
            } else {
                write!(o, "null")?;
            }
            if i != self.states.len() - 1 {
                writeln!(o, "}},")?;
            } else {
                writeln!(o, "}}")?;
            }
        }
        writeln!(o, "];")?;
        Ok(())
    }

    pub fn run(&self, str: &str) -> Option<String> {
        let mut cur_state = self.start_state;
        for c in str.chars() {
            if let Some(next_state) = self.transition.get(&(cur_state, c)) {
                cur_state = *next_state;
            } else {
                return None;
            }
        }

        let state = self.states.get(&cur_state).expect("Should be in map");
        if let Some(mapped) = &state.accepting {
            return Some(mapped.clone());
        }

        None
    }

    pub fn from_writer(writer: &KbdWriter) -> (Self, Vec<u32>) {
        let mut ranges = Vec::new();
        let mut iter = writer.sections.iter().peekable();
        let (mut a, mut state_cnt) = iter.peek().map_or(Default::default(), |(_, first_keymap)| {
            Automaton::from_section(first_keymap)
        });
        ranges.push(state_cnt);
        iter.next();
        for (_, keymap) in iter {
            state_cnt = Automaton::join_section(&mut a, keymap, state_cnt);
            ranges.push(state_cnt);
        }
        (a, ranges)
    }

    pub fn join_section(automaton: &mut Automaton, keymap: &KbdMap, start_cnt: u32) -> u32 {
        let start_state = 0;
        let mut state_cnt: u32 = start_cnt;
        for (seq, mapped) in keymap.iter() {
            // initialize state
            let (mut prev_state, mut str) = (start_state, String::new());
            // for each transition letter, number the state if new and insert a transition
            for char in seq.chars() {
                str.push(char);
                let cur_state =
                    if let Entry::Vacant(e) = automaton.transition.entry((prev_state, char)) {
                        // transition doesn't exist, hence no state
                        state_cnt += 1;
                        automaton.states.insert(
                            state_cnt,
                            State {
                                state_num: state_cnt,
                                next: BTreeMap::new(),
                                label: str.to_string(),
                                accepting: None,
                            },
                        );
                        e.insert(state_cnt);
                        state_cnt
                    } else {
                        // transition already exists
                        *automaton
                            .transition
                            .get(&(prev_state, char))
                            .expect("Should be in the map 1")
                    };
                let prev = automaton
                    .states
                    .get_mut(&prev_state)
                    .expect("Should be in map 2");
                if let Entry::Vacant(e) = prev.next.entry(cur_state) {
                    e.insert(char);
                }
                prev_state = cur_state;
            }

            let previous = automaton
                .states
                .get_mut(&prev_state)
                .expect("Should be in map 3");
            if let Some(mapped2) = &previous.accepting {
                eprintln!(
                    "Two different mappings after the same sequence: {} {}",
                    mapped, mapped2
                );
            } else {
                previous.accepting = Some(mapped.clone());
            }
        }
        state_cnt
    }

    pub fn from_section(keymap: &KbdMap) -> (Self, u32) {
        // create empty automaton
        let start_state = 0;
        let mut a = Automaton {
            states: BTreeMap::new(),
            transition: BTreeMap::new(),
            start_state,
        };

        // initialize the dfa with a start state
        a.states.insert(
            start_state,
            State {
                state_num: start_state,
                next: BTreeMap::new(),
                label: "start".to_string(),
                accepting: None,
            },
        );

        let new_count = Automaton::join_section(&mut a, keymap, 0);
        (a, new_count)
    }

    pub fn write(
        &self,
        o: &mut String,
        ranges: Vec<u32>,
        kbdwriter: &KbdWriter,
        style: AutomatonStyle,
    ) -> std::fmt::Result {
        writeln!(o, "digraph G {{")?;
        let mut lower_range = 1;
        for (cluster_num, (str, range_end)) in kbdwriter
            .sections
            .iter()
            .map(|(str, _)| str)
            .zip(ranges.iter())
            .enumerate()
        {
            // cluster header
            writeln!(o, "\tsubgraph cluster_{} {{", cluster_num)?;
            writeln!(o, "\t\tstyle={};", style.cluster_style)?;
            writeln!(o, "\t\tbgcolor={};", style.cluster_bgcolor)?;
            writeln!(
                o,
                "\t\tnode [style=filled,shape={},fillcolor={},fontcolor={}];",
                style.cluster_node_shape, style.cluster_node_bgcolor, style.cluster_node_fontcolor,
            )?;
            writeln!(o, "\t\tlabel=\"{}\";", str)?;

            // cluster nodes
            for cur in lower_range..=*range_end {
                let state = self.states.get(&cur).expect("Map should have value!");
                if let Some(accepting_str) = &state.accepting {
                    writeln!(
                        o,
                        "\t\t{:3} [label=\"{}\"];",
                        cur,
                        accepting_str.replace('\"', "\\\"")
                    )?;
                }
            }
            lower_range = range_end + 1;
            writeln!(o, "\t}}")?;
        }
        // transitions
        writeln!(
            o,
            "\tnode [style=filled,fillcolor={},fontcolor={},shape={}];",
            style.intermediate_node_bgcolor, style.intermediate_fontcolor, style.intermediate_shape
        )?;
        for (cur, state) in self.states.iter() {
            if *cur == 0 {
                writeln!(
                    o,
                    "\t{:3} [style=filled,fillcolor={},fontcolor={},shape={},label=\"{}\"]",
                    cur,
                    style.start_bgcolor,
                    style.start_fontcolor,
                    style.start_shape,
                    state.label.replace('\"', "\\\"")
                )?;
            } else if state.accepting.is_none() {
                writeln!(
                    o,
                    "\t{:3}     \t[label=\"{}\"];",
                    cur,
                    state.label.replace('\"', "\\\"")
                )?;
            }
            for (next, edge) in state.next.iter() {
                if edge == &'\"' {
                    writeln!(o, "\t{:3} -> {:3}\t[label=\"\\\"\"];", cur, next)?;
                } else if edge == &' ' {
                    writeln!(o, "\t{:3} -> {:3}\t[label=\"space\"];", cur, next)?;
                } else {
                    writeln!(o, "\t{:3} -> {:3}\t[label=\"{}\"];", cur, next, edge)?;
                }
            }
        }
        writeln!(o, "}}")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AutomatonStyle {
    cluster_style: String,
    cluster_bgcolor: String,
    cluster_node_shape: String,
    cluster_node_bgcolor: String,
    cluster_node_fontcolor: String,
    intermediate_shape: String,
    intermediate_node_bgcolor: String,
    intermediate_fontcolor: String,
    start_bgcolor: String,
    start_fontcolor: String,
    start_shape: String,
}

impl Default for AutomatonStyle {
    fn default() -> Self {
        Self {
            cluster_style: "filled".to_string(),
            cluster_bgcolor: "aliceblue".to_string(),
            cluster_node_shape: "circle".to_string(),
            cluster_node_bgcolor: "white".to_string(),
            cluster_node_fontcolor: "black".to_string(),
            intermediate_shape: "circle".to_string(),
            intermediate_node_bgcolor: "black".to_string(),
            intermediate_fontcolor: "white".to_string(),
            start_bgcolor: "white".to_string(),
            start_fontcolor: "black".to_string(),
            start_shape: "ellipse".to_string(),
        }
    }
}
