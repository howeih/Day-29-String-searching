#Day 29: String searching
<br>
In computer science, the Knuth–Morris–Pratt string-searching algorithm (or KMP algorithm) searches for occurrences of a "word" W within a main "text string" S by employing the observation that when a mismatch occurs, the word itself embodies sufficient information to determine where the next match could begin, thus bypassing re-examination of previously matched characters.
<br>
The algorithm was conceived in 1970 by Donald Knuth and Vaughan Pratt, and independently by James H. Morris. This was the first linear-time algorithm for string matching. The three published it jointly in 1977.[1] Independently, in 1969, Matiyasevich discovered a similar algorithm, coded by a two-dimensional Turing machine, while studying a string-pattern-matching recognition problem.
<br>
run

```

fn main() {
    assert_eq!(
        kmp(
            "A parabolic (or paraboloid or paraboloidal) reflector (or dish or mirror)",
            "reflector"
        ).unwrap(),
        44
    );
}
```