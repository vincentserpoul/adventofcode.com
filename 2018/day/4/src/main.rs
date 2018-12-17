use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
extern crate chrono;
use chrono::{Duration, NaiveDateTime, Timelike};
extern crate regex;
#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() -> io::Result<()> {
    let f = File::open("./input.txt").unwrap();
    // let f = File::open("./src/test_input.txt").unwrap();
    let mut tt = TimeTable::new();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .for_each(|l| tt.parse_line(&l));
    tt.parse_state();

    let lazy_guard = tt.most_asleep_guard();
    let laziness = tt.most_slept_minute_for_a_specific_guard(lazy_guard);
    println!(
        "the laziest guard is {} who slept {} times at minute {}, the answer is then: {}",
        lazy_guard,
        laziness.1,
        laziness.0,
        lazy_guard * laziness.0
    );

    let laziest = tt.most_slept_minute();
    println!("the laziest overall is guard {} who slept for {} times at minute {}, the answer to part 2 is then {}",
        laziest.0, laziest.2, laziest.1, laziest.0 * laziest.1,
    );

    Ok(())
}

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Awake,
    Asleep,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Awake => write!(f, "awake"),
            Status::Asleep => write!(f, "asleep"),
        }
    }
}

enum Action {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::BeginsShift => write!(f, "BeginsShift"),
            Action::FallsAsleep => write!(f, "FallsAsleep"),
            Action::WakesUp => write!(f, "WakesUp"),
        }
    }
}

struct TimeTable {
    datetimes: Vec<NaiveDateTime>,
    guards_shift: HashMap<NaiveDateTime, u32>,
    actions: HashMap<NaiveDateTime, Action>,
    state: Vec<(NaiveDateTime, u32, Status)>,
    asleep_minutes: HashMap<u32, HashMap<u32, u32>>,
}

impl TimeTable {
    fn new() -> TimeTable {
        TimeTable {
            datetimes: Vec::new(),
            guards_shift: HashMap::new(),
            actions: HashMap::new(),
            state: Vec::new(),
            asleep_minutes: HashMap::new(),
        }
    }

    fn parse_line(&mut self, l: &str) {
        // take 18 first chars and turn them into a date
        let datetime = NaiveDateTime::parse_from_str(&l[1..17], "%Y-%m-%d %H:%M").unwrap();
        self.datetimes.push(datetime);
        if l.contains("begins shift") {
            self.guards_shift.insert(datetime, extract_guard_id(l));
        }

        let mut act: Action = Action::BeginsShift;
        if l.contains("falls asleep") {
            act = Action::FallsAsleep;
        } else if l.contains("wakes up") {
            act = Action::WakesUp;
        }

        self.actions.insert(datetime, act);
    }

    fn parse_state(&mut self) {
        self.datetimes.sort();
        let end_datetime = *self.datetimes.last().unwrap();
        let mut curr_datetime = *self.datetimes.first().unwrap();
        let mut curr_guard_id = self.guards_shift[&curr_datetime];
        let mut curr_status = Status::Awake;
        self.state.push((curr_datetime, curr_guard_id, curr_status));
        // loop every minute from start date to end date
        loop {
            if self.guards_shift.contains_key(&curr_datetime) {
                curr_guard_id = self.guards_shift[&curr_datetime];
            }
            if self.actions.contains_key(&curr_datetime) {
                curr_status = match self.actions[&curr_datetime] {
                    Action::BeginsShift => Status::Awake,
                    Action::FallsAsleep => Status::Asleep,
                    Action::WakesUp => Status::Awake,
                }
            }
            self.state.push((curr_datetime, curr_guard_id, curr_status));

            if curr_status == Status::Asleep {
                self.asleep_minutes
                    .entry(curr_guard_id)
                    .and_modify(|m| {
                        m.entry(curr_datetime.minute())
                            .and_modify(|v| {
                                *v += 1;
                            })
                            .or_insert(1);
                    })
                    .or_insert({
                        let mut h = HashMap::new();
                        h.insert(curr_datetime.minute(), 1);
                        h
                    });
            }

            if curr_datetime == end_datetime {
                break;
            }

            curr_datetime = curr_datetime
                .checked_add_signed(Duration::minutes(1))
                .unwrap();
        }
    }

    fn most_asleep_guard(&self) -> u32 {
        let mut guard_asleep_time: HashMap<u32, u32> = HashMap::new();
        self.state
            .iter()
            .filter(|(_t, _gid, st)| *st == Status::Asleep)
            .for_each(|(_t, gid, _st)| {
                let s = guard_asleep_time.entry(*gid).or_insert(0);
                *s += 1;
            });
        let mut max_sleep_time: u32 = 0;
        let mut max_guard_id: u32 = 0;
        for (gid, mn) in guard_asleep_time.iter() {
            if *mn > max_sleep_time {
                max_guard_id = *gid;
                max_sleep_time = *mn;
            }
        }
        max_guard_id
    }

    fn most_slept_minute(&self) -> (u32, u32, u32) {
        let mut guard_id: u32 = 0;
        let mut most_slept_minute_time: u32 = 0;
        let mut most_slept_minute: u32 = 0;
        self.asleep_minutes.iter().for_each(|(gid, min_time)| {
            min_time.iter().for_each(|(m, t)| {
                if *t > most_slept_minute_time {
                    most_slept_minute_time = *t;
                    most_slept_minute = *m;
                    guard_id = *gid;
                }
            });
        });
        (guard_id, most_slept_minute, most_slept_minute_time)
    }

    fn most_slept_minute_for_a_specific_guard(&self, guard_id: u32) -> (u32, u32) {
        let mut most_slept_minute_time: u32 = 0;
        let mut most_slept_minute: u32 = 0;
        self.asleep_minutes[&guard_id].iter().for_each(|(m, t)| {
            if *t > most_slept_minute_time {
                most_slept_minute_time = *t;
                most_slept_minute = *m;
            }
        });
        (most_slept_minute, most_slept_minute_time)
    }
}

fn extract_guard_id(text: &str) -> u32 {
    lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(r"\#\d{0,9}").unwrap();
    }
    let match_str = HASHTAG_REGEX.find(text).unwrap();
    text[match_str.start() + 1..match_str.end()]
        .parse()
        .unwrap()
}

#[test]
fn test_extract_guard_id() {
    assert_eq!(extract_guard_id("  #123"), 123);
    assert_eq!(
        extract_guard_id("[1518-06-17 00:04] Guard #1951 begins shift"),
        1951
    );
}
