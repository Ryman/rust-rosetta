// Solution for http://rosettacode.org/wiki/Execute_a_Markov_algorithm

// Individual markov rule
struct MarkovRule {
    pattern: ~str,
    replacement: ~str,
    stop: bool,
}

impl MarkovRule {
    fn new(pattern: ~str, replacement: ~str, stop: bool) -> MarkovRule {
        MarkovRule{pattern: pattern, replacement: replacement, stop: stop,}
    }
}

// The complete markov algorithm
struct MarkovAlgorithm {
    rules: Vec<MarkovRule>,
}

impl MarkovAlgorithm {
    // Parse an algorithm description to build a markov algorithm
    pub fn from_str(s: &str) -> Result<MarkovAlgorithm, ~str> {
        let mut rules: Vec<MarkovRule> = vec!();
        for line in
            s.lines().map(|l|
                              l.trim()).filter( // Ignore whitespace before and after
                                               |l|
                                                   l.char_len() > 0 &&
                                                       l.char_at(0) != '#')
            { // Ignore comments

             // check for -> (must be preceded by whitespace)
             // invalid ruleset if absent
             // whitespace rules mean there's 2 possible variations: " ->" and "\t->"

            let arrow_pos =
                line.find_str(" ->").or_else(|| line.find_str("\t->"));
            match arrow_pos {
                None => {
                    // Ruleset is invalid
                    return Err(format!("Invalid rule \"{}\"" , line));
                },
                Some(arrow) => {
                    // extract pattern (trim trailing whitespace)
                    let pattern = line.slice_to(arrow).trim_right();

                    // get the string after the arrow
                    // this adds 3 to skip the arrow itself
                    let line_end = line.slice_from(arrow + 3).trim_left();

                    // check for . (stop)
                    let stop =
                        (line_end.char_len() > 0) &&
                            (line_end.char_at(0) == '.');

                    // extract replacement
                    let replacement =
                        if stop { line_end.slice_from(1) } else { line_end };

                    // add to rules
                    let new_rule =
                        MarkovRule::new(pattern.to_owned(),
                                        replacement.to_owned(), stop);
                    rules.push(new_rule);
                }
            }
        }
        let rule_set = MarkovAlgorithm{rules: rules,};
        Ok(rule_set)
    }

    // Transform a text string by applying the markov algorithm
    pub fn apply(&self, input: &str) -> ~str {

        // get a writable version of the input to work with
        let mut state = input.into_owned();

        // Don't allow input to be used after this
        drop(input);

        // loop while operations are possible
        loop  {
            // find the first rule that is applicable
            // (pattern string is in state)
            let mut rule_iterator = self.rules.iter();
            let possible_rule =
                rule_iterator.find(|rule| {
                                   state.find_str(rule.pattern).is_some() });

            match possible_rule {

                // stop if no rule found
                None => {
                    break ;
                },
                Some(rule) => {
                    // replace the first instance (only) of the pattern
                    // Note: cannot use str::replace as that replaces all instances

                    // unwrap is safe here as the code for finding a rule
                    // already established that the pattern is present
                    let pos = state.find_str(rule.pattern).unwrap();
                    let width = rule.pattern.len();

                    // string parts
                    let left = state.slice_to(pos).to_owned();
                    let right = state.slice_from(pos + width).to_owned();

                    // construct new string
                    state = left + rule.replacement + right;

                    // stop if required
                    if rule.stop { break ; }
                }
            }
        }

        state
    }
}

// A Rosetta Code sample
struct RCSample {
    ruleset: ~str,
    input: ~str,
    expected_result: ~str,
}

// Sample markow algorithms from rosetta code
// The extra whitespaces are trimmed when MarkovAlgorithm::from_str is called
fn get_samples() -> [RCSample, ..5] {
    [RCSample{ruleset:
                  "# This rules file is extracted from Wikipedia:
                # http://en.wikipedia.org/wiki/Markov_Algorithm
                A -> apple
                B -> bag
                S -> shop
                T -> the
                the shop -> my brother
                a never used -> .terminating rule".to_owned(),
              input: "I bought a B of As from T S.".to_owned(),
              expected_result:
                  "I bought a bag of apples from my brother.".to_owned(),},
     RCSample{ruleset:
                  "# Slightly modified from the rules on Wikipedia
                A -> apple
                B -> bag
                S -> .shop
                T -> the
                the shop -> my brother
                a never used -> .terminating rule".to_owned(),
              input: "I bought a B of As from T S.".to_owned(),
              expected_result:
                  "I bought a bag of apples from T shop.".to_owned(),},
     RCSample{ruleset:
                  "# BNF Syntax testing rules
                A -> apple
                WWWW -> with
                Bgage -> ->.*
                B -> bag
                ->.* -> money
                W -> WW
                S -> .shop
                T -> the
                the shop -> my brother
                a never used -> .terminating rule".to_owned(),
              input: "I bought a B of As W my Bgage from T S.".to_owned(),
              expected_result:
                  "I bought a bag of apples with my money from T shop.".to_owned(),},
     RCSample{ruleset:
                  "### Unary Multiplication Engine, for testing Markov Algorithm implementations
                ### By Donal Fellows.
                # Unary addition engine
                _+1 -> _1+
                1+1 -> 11+
                # Pass for converting from the splitting of multiplication into ordinary
                # addition
                1! -> !1
                ,! -> !+
                _! -> _
                # Unary multiplication by duplicating left side, right side times
                1*1 -> x,@y
                1x -> xX
                X, -> 1,1
                X1 -> 1X
                _x -> _X
                ,x -> ,X
                y1 -> 1y
                y_ -> _
                # Next phase of applying
                1@1 -> x,@y
                1@_ -> @_
                ,@_ -> !_
                ++ -> +
                # Termination cleanup for addition
                _1 -> 1
                1+_ -> 1
                _+_ -> ".to_owned(),
              input: "_1111*11111_".to_owned(),
              expected_result: "11111111111111111111".to_owned(),},
     RCSample{ruleset:
                  "# Turing machine: three-state busy beaver
                #
                # state A, symbol 0 => write 1, move right, new state B
                A0 -> 1B
                # state A, symbol 1 => write 1, move left, new state C
                0A1 -> C01
                1A1 -> C11
                # state B, symbol 0 => write 1, move left, new state A
                0B0 -> A01
                1B0 -> A11
                # state B, symbol 1 => write 1, move right, new state B
                B1 -> 1B
                # state C, symbol 0 => write 1, move left, new state B
                0C0 -> B01
                1C0 -> B11
                # state C, symbol 1 => write 1, move left, halt
                0C1 -> H01
                1C1 -> H11".to_owned(),
              input: "000000A000000".to_owned(),
              expected_result: "00011H1111000".to_owned(),}]
}

#[cfg(not(test))]
fn main() {
    for (index, sample) in get_samples().iter().enumerate() {
        match MarkovAlgorithm::from_str(sample.ruleset) {
            Ok(algorithm) => {
                println!("Sample {}" , ( index + 1 ));
                println!("Output: {}" , algorithm . apply ( sample . input ));
            },
            Err(message) => println!("{}" , message)
        }
    }
}

#[test]
fn test_samples() {
    for sample in get_samples().iter() {
        match MarkovAlgorithm::from_str(sample.ruleset) {
            Ok(algorithm) =>
            assert!(sample . expected_result == algorithm . apply
                    ( sample . input )),
            Err(message) => fail!("{}" , message)
        }
    }
}
