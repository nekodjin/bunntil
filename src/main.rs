use std::io;

use chrono::Local;
use chrono::LocalResult;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::TimeZone as _;

use clap::Parser as _;

use crossterm::execute;
use crossterm::style as term;

#[allow(non_camel_case_types)]
use term::Attribute as attr;
#[allow(non_camel_case_types)]
use term::Color as color;

fn main() -> anyhow::Result<()> {
    let _args = Args::parse();

    let today = Local::now();
    let target = match Local.from_local_datetime(&NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2023, 06, 30)
            .ok_or(anyhow::anyhow!("failed to create target date"))?,
        NaiveTime::from_hms_opt(00, 00, 00)
            .ok_or(anyhow::anyhow!("failed to create target time"))?,
    )) {
        LocalResult::Single(time) => time,
        _ => anyhow::bail!("failed to create target datetime"),
    };

    let time_remaining = target - today;
    let days_remaining = time_remaining.num_days() + 1;

    execute!(
        io::stdout(),
        term::Print("Days Left: "),
        term::SetAttribute(attr::Bold),
        term::SetForegroundColor(color::Blue),
        term::Print(days_remaining.to_string()),
        term::SetAttribute(attr::NormalIntensity),
        term::ResetColor,
    )?;

    println!();

    let factors = factorize(days_remaining);

    execute!(
        io::stdout(),
        term::Print("Factors: "),
        term::SetAttribute(attr::Bold),
        term::SetForegroundColor(color::Blue),
        term::Print(factors[0].to_string()),
        term::SetAttribute(attr::NormalIntensity),
        term::ResetColor,
    )?;

    for factor in &factors[1..] {
        execute!(
            io::stdout(),
            term::Print(if factors.len() == 1 { "" } else { ", " }),
            term::SetAttribute(attr::Bold),
            term::SetForegroundColor(color::Blue),
            term::Print(factor.to_string()),
            term::SetAttribute(attr::NormalIntensity),
            term::ResetColor,
        )?;
    }

    println!();

    Ok(())
}

#[derive(clap::Parser)]
struct Args;

fn factorize(mut n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![0];
    }

    if n < 0 {
        n = -n;
    }

    let mut factors = Vec::new();
    let mut candidate = 2;

    while n > 1 {
        if n % candidate == 0 {
            n /= candidate;
            factors.push(candidate);
        }
        else {
            candidate += 1;
        }
    }

    match &factors[..] {
        [] => vec![1],
        [x] => vec![1, *x],
        _ => factors,
    }
}
