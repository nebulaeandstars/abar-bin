use crate::utils::run;

pub fn keyboard() -> String {
    run(r"setxkbmap -query | grep -oP 'layout:\s*\K\w+'")
}
