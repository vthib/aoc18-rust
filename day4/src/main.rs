use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;
use std::iter::Iterator;
use std::str::FromStr;

use scan_fmt::scan_fmt;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut logs: Vec<Log> = Vec::new();

    /* parse logs */
    for line in input.lines() {
        logs.push(line.parse()?);
    }
    /* sort all logs chronologically */
    logs.sort_unstable();

    let map = build_guards_map(&logs);
    part1(&map);
    part2(&map);
    Ok(())
}

/* {{{ Part1 */

fn part1(map: &HashMap<u32, GuardSleeping>) {
    let (guard_id, guard) = map
        .iter()
        .max_by_key(|&(_, guard)| guard.total_minutes)
        .unwrap();
    let minute_max = guard
        .sleep_records
        .iter()
        .enumerate()
        .max_by_key(|&(_, m)| m)
        .unwrap();

    println!(
        "day4, part1: guard_id: {}, minute: {} => {}",
        guard_id,
        minute_max.0,
        guard_id * (minute_max.0 as u32)
    );
}

/* }}} */
/* {{{ Part2 */

fn part2(map: &HashMap<u32, GuardSleeping>) {
    /* guard with current max */
    let mut max_guard_id = 0;
    /* minute of guard with current max */
    let mut max_minute = 0;
    /* current max */
    let mut max = 0;

    /* across all guards and all minutes, find max */
    for (guard_id, guard) in map {
        for (minute_nb, val) in guard.sleep_records.iter().enumerate() {
            if *val > max {
                max_guard_id = *guard_id;
                max_minute = minute_nb;
                max = *val;
            }
        }
    }

    println!(
        "day4, part2: guard_id: {}, minute: {}, value: {} => {}",
        max_guard_id,
        max_minute,
        max,
        max_guard_id * (max_minute as u32)
    );
}

/* }}} */
/* {{{ Guard map */

struct GuardSleeping {
    /* number of times a guard was sleeping, per minutes */
    sleep_records: [u32; 60],
    /* total number of minutes sleeping */
    total_minutes: u32,
}

impl GuardSleeping {
    fn add_sleepy_time(&mut self, start: u8, end: u8) {
        self.total_minutes += (end - start) as u32;
        for m in start..end {
            self.sleep_records[m as usize] += 1;
        }
    }
}

fn build_guards_map(logs: &Vec<Log>) -> HashMap<u32, GuardSleeping> {
    let mut map = HashMap::new();
    let mut cur_guard_id = 0;
    let mut asleep_minute = None;

    for log in logs {
        match log.typ {
            LogType::BeginShift(guard_id) => {
                if !map.contains_key(&guard_id) {
                    let guard = GuardSleeping {
                        sleep_records: [0; 60],
                        total_minutes: 0,
                    };
                    map.insert(guard_id, guard);
                }
                cur_guard_id = guard_id;
            }
            LogType::FallsAsleep => {
                asleep_minute = Some(log.minute);
            }
            LogType::WakesUp => {
                let guard = map.get_mut(&cur_guard_id).unwrap();
                assert!(asleep_minute.is_some());
                guard.add_sleepy_time(asleep_minute.unwrap(), log.minute);
            }
        }
    }
    map
}

/* }}} */
/* {{{ Date */

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

/* }}} */
/* {{{ Log */

#[derive(Debug)]
enum LogType {
    BeginShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct Log {
    date: Date,
    minute: u8,
    typ: LogType,
}

/* {{{ Ordering */

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.date.partial_cmp(&other.date) {
            None => None,
            Some(ordering) => match ordering {
                cmp::Ordering::Equal => self.minute.partial_cmp(&other.minute),
                _ => Some(ordering),
            },
        }
    }
}

impl Ord for Log {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let date_cmp = self.date.cmp(&other.date);
        match date_cmp {
            cmp::Ordering::Equal => self.minute.cmp(&other.minute),
            _ => date_cmp,
        }
    }
}

impl PartialEq for Log {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.minute == other.minute
    }
}
impl Eq for Log {}

/* }}} */
/* {{{ FromStr */

impl FromStr for Log {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (year, month, day, h, m, text) = scan_fmt!(
            s,
            "[{d}-{d}-{d} {d}:{d}] {[a-zA-Z#0-9 ]}",
            u32,
            u32,
            u32,
            u8,
            u8,
            String
        );
        let year = year.unwrap();
        let month = month.unwrap();
        let mut day = day.unwrap();
        let mut minute = m.unwrap();
        if h.unwrap() == 23 {
            day += 1;
            minute = 0;
        }
        let text = text.unwrap();

        match text.as_ref() {
            "falls asleep" => Ok(Log {
                date: Date { year, month, day },
                minute,
                typ: LogType::FallsAsleep,
            }),
            "wakes up" => Ok(Log {
                date: Date { year, month, day },
                minute,
                typ: LogType::WakesUp,
            }),
            _ => {
                let id = scan_fmt!(&text, "Guard #{d} begins shift", u32);
                Ok(Log {
                    date: Date { year, month, day },
                    minute,
                    typ: LogType::BeginShift(id.unwrap()),
                })
            }
        }
    }
}

/* }}} */
/* }}} */
