fn find_failure_function(pattern: &Vec<char>) -> Vec<usize> {
    let mut i = 1;
    let mut j = 0;
    let pattern_length = pattern.len();
    let end_i = pattern_length - 1;
    let mut failure_function = vec![0usize; pattern_length];
    while i <= end_i {
        if pattern[i] == pattern[j] {
            failure_function[i] = j + 1;
            i = i + 1;
            j = j + 1;
        } else {
            if j == 0 {
                failure_function[i] = 0;
                i = i + 1;
            } else {
                j = failure_function[j - 1];
            }
        }
    }
    failure_function
}

pub fn index_of_any(
    text: &str,
    pattern: &Vec<char>,
    failure_function: Vec<usize>,
) -> Option<usize> {
    let pattern_length = pattern.len();
    let text: Vec<char> = text.chars().collect();
    let mut t_i: usize = 0;
    let mut p_i: usize = 0;
    let text_len = text.len();
    let mut result_idx = 0usize;
    let pattern_len = pattern_length;
    while (t_i <= text_len - 1) && (p_i <= pattern_len - 1) {
        if text[t_i] == pattern[p_i] {
            if result_idx == 0 {
                result_idx = t_i;
            }
            t_i = t_i + 1;
            p_i = p_i + 1;
            if p_i >= pattern_len {
                return Some(result_idx);
            }
        } else {
            if p_i == 0 {
                p_i = 0;
            } else {
                p_i = failure_function[p_i - 1];
            }
            t_i = t_i + 1;
            result_idx = 0;
        }
    }
    None
}

fn kmp(text: &str, pattern: &str) -> Option<usize> {
    let pattern: Vec<char> = pattern.chars().collect();
    let failure_function = find_failure_function(&pattern);
    index_of_any(text, &pattern, failure_function)
}

fn main() {
    assert_eq!(
        kmp(
            "A parabolic (or paraboloid or paraboloidal) reflector (or dish or mirror)",
            "reflector"
        ).unwrap(),
        44
    );
}
