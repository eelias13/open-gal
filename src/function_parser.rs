use super::*;
use std::collections::HashMap;
use std::usize;

struct Node {
    left: Option<*mut Node>,
    right: Option<*mut Node>,
    value: Option<*mut bool>,
    operator: Option<BoolFunc>,
}

impl Node {
    pub fn build_tree(
        func: Vec<BoolFunc>,
        lookup: &HashMap<BoolFunc, *mut bool>,
        const_bool: &Vec<*mut bool>,
    ) -> *mut Self {
        let bundle = Node::split(&func);

        // is binery operator (and, or, xor)
        if let (Some(right), Some(left)) = (bundle.right.clone(), bundle.left.clone()) {
            let node = Box::new(Node {
                left: Some(Node::build_tree(left.clone(), lookup, const_bool)),
                right: Some(Node::build_tree(right.clone(), lookup, const_bool)),
                value: None,
                operator: Some(bundle.center.clone()),
            });

            return Box::into_raw(node);
        }

        // is unery operator (not)
        if let Some(left) = bundle.left.clone() {
            let node = Box::new(Node {
                left: Some(Node::build_tree(left.clone(), lookup, const_bool)),
                right: None,
                value: None,
                operator: Some(bundle.center.clone()),
            });

            return Box::into_raw(node);
        }

        // is leaf node
        let value;
        if bundle.center == BoolFunc::Zero {
            value = Some(const_bool[0]);
        } else if bundle.center == BoolFunc::One {
            value = Some(const_bool[1]);
        } else {
            value = Some(*lookup.get(&bundle.center).unwrap());
        }

        let node = Box::new(Node {
            left: None,
            right: None,
            value,
            operator: None,
        });
        Box::into_raw(node)
    }

    fn split_not(func: &Vec<BoolFunc>) -> Bundle {
        let mut left = Vec::new();
        for i in 1..func.len() {
            left.push(func[i].clone());
        }

        Bundle {
            left: Some(left),
            center: func[0].clone(),
            right: None,
        }
    }

    fn split_operator(func: &Vec<BoolFunc>) -> Bundle {
        let index = Node::split_index(func);

        // if it is 0 it must be not or an error
        if index == 0 {
            if func[0].clone() == BoolFunc::Not {
                return Node::split_not(func);
            }
            unreachable!();
        }

        let mut left = Vec::new();
        let mut right = Vec::new();
        let center = func[index].clone();
        for i in 0..index {
            left.push(func[i].clone());
        }
        for i in (index + 1)..func.len() {
            right.push(func[i].clone());
        }

        Bundle {
            left: Some(left),
            right: Some(right),
            center,
        }
    }

    // split_index: gives the index of the operator by which the expression must be split
    // Exampel: "(a|b)&c" the function would return 5 witch is the index of the char '&'
    fn split_index(func: &Vec<BoolFunc>) -> usize {
        let mut operator_index = 0;
        let mut operator_score = 0xff;
        let mut parentheses = 0;

        for (i, t) in func.iter().enumerate() {
            if parentheses != 0 {
                if t == &BoolFunc::Close {
                    parentheses -= 1;
                }
                if t == &BoolFunc::Open {
                    parentheses += 1;
                }
                continue;
            }
            if t == &BoolFunc::Open {
                parentheses += 1;
            }

            if Node::precedence_of(t.clone()) < operator_score {
                operator_score = Node::precedence_of(t.clone());
                operator_index = i;
                if operator_score == 0 {
                    break;
                }
            }
        }

        return operator_index;
    }

    fn precedence_of(bool_func: BoolFunc) -> usize {
        match bool_func {
            BoolFunc::Or => 0,
            BoolFunc::Xor => 1,
            BoolFunc::And => 2,
            BoolFunc::Not => 3,
            _ => 0xff,
        }
    }

    // this is a very imortant function to buid the tree. It splits the function in to 3 parts Example: a & b | (c | d) -> a & b, |, (c | d)
    // this works with any arbitrarily complicated function
    fn split(func: &Vec<BoolFunc>) -> Bundle {
        // is leavnode
        if func.len() == 1 {
            return Bundle {
                left: None,
                right: None,
                center: func[0].clone(),
            };
        }

        // is_parentheses: Example (a & b) returns true (a) & (b) return false
        if Node::is_parentheses(func) {
            // split_parentheses: removes the first and last char
            return Node::split_parentheses(func);
        }

        return Node::split_operator(func);
    }

    // remove fist and last item and calls split
    fn split_parentheses(func: &Vec<BoolFunc>) -> Bundle {
        let mut result = Vec::new();
        for i in 1..(func.len() - 1) {
            result.push(func[i].clone());
        }
        return Node::split(&result);
    }

    fn is_parentheses(func: &Vec<BoolFunc>) -> bool {
        if func[0] != BoolFunc::Open {
            return false;
        }
        if func.last() != Some(&BoolFunc::Close) {
            return false;
        }

        let mut counter = 1;
        for i in 1..(func.len() - 1) {
            if counter == 0 {
                return false;
            }
            if func[i] == BoolFunc::Open {
                counter += 1;
            }
            if func[i] == BoolFunc::Close {
                counter -= 1;
            }
        }
        true
    }

    pub fn eval(node_ptr: *mut Node) -> bool {
        // can do unsafe because node_ptr != null
        unsafe {
            // if levenode then return value from lookuptable
            if let Some(value) = (*node_ptr).value {
                return *value;
            }
            // walks down the tree depth first (left) and evaluates it
            let res = match (*node_ptr).operator {
                Some(BoolFunc::And) => {
                    if let (Some(left), Some(right)) = ((*node_ptr).left, (*node_ptr).right) {
                        Node::eval(left) & Node::eval(right)
                    } else {
                        // TODO make error
                        unreachable!()
                    }
                }
                Some(BoolFunc::Xor) => {
                    if let (Some(left), Some(right)) = ((*node_ptr).left, (*node_ptr).right) {
                        Node::eval(left) ^ Node::eval(right)
                    } else {
                        // TODO make error
                        unreachable!()
                    }
                }
                Some(BoolFunc::Or) => {
                    if let (Some(left), Some(right)) = ((*node_ptr).left, (*node_ptr).right) {
                        Node::eval(left) | Node::eval(right)
                    } else {
                        // TODO make error
                        unreachable!()
                    }
                }
                Some(BoolFunc::Not) => {
                    if let Some(left) = (*node_ptr).left {
                        !Node::eval(left)
                    } else {
                        // TODO make error
                        unreachable!()
                    }
                }
                _ => {
                    // TODO make error
                    unreachable!()
                }
            };
            return res;
        }
    }
}

struct Bundle {
    left: Option<Vec<BoolFunc>>,
    right: Option<Vec<BoolFunc>>,
    center: BoolFunc,
}

fn init_lookup_values(
    func: &Vec<BoolFunc>,
    lookup: &mut HashMap<BoolFunc, *mut bool>,
    values: &mut Vec<bool>,
) {
    let mut vars = Vec::<String>::new();

    for f in func {
        match f.clone() {
            BoolFunc::Var { name } => {
                let mut in_var = false;
                for s in vars.clone() {
                    if s == name {
                        in_var = true;
                        break;
                    }
                }
                if !in_var {
                    values.push(false);
                    unsafe {
                        let ptr = values.as_mut_ptr().add(values.len() - 1);
                        lookup.entry(f.clone()).or_insert(ptr.clone());
                    }
                } else {
                    vars.push(name);
                }
            }
            _ => (),
        }
    }
}

fn update_values(values: &mut Vec<bool>) -> bool {
    for i in (0..values.len()).rev() {
        values[i] = !values[i];
        if values[i].clone() == true {
            return true;
        }
    }
    return false;
}

pub fn get_names(func: Vec<BoolFunc>) -> Vec<String> {
    let mut vars = Vec::new();
    for f in func {
        match f.clone() {
            BoolFunc::Var { name } => {
                let mut in_var = false;
                for s in vars.clone() {
                    if s == name {
                        in_var = true;
                        break;
                    }
                }
                if !in_var {
                    vars.push(name);
                }
            }
            _ => (),
        }
    }
    vars
}

pub fn parse(func: Vec<BoolFunc>) -> Vec<bool> {
    let mut values = Vec::<bool>::new();
    let mut lookup = HashMap::<BoolFunc, *mut bool>::new();
    init_lookup_values(&func, &mut lookup, &mut values);

    let mut temp = vec![false, true];
    let const_bool = unsafe { vec![temp.as_mut_ptr().add(0), temp.as_mut_ptr().add(1)] };

    let tree = Node::build_tree(func.clone(), &lookup, &const_bool);
    let mut result = Vec::new();

    result.push(Node::eval(tree));

    if let Some(ptr) = lookup.get(&BoolFunc::Var {
        name: "a".to_string(),
    }) {
        unsafe {
            println!("{:?}", ptr.read_volatile());
        }
    }
    while update_values(&mut values) {
        result.push(Node::eval(tree));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_names() {
        let input = vec![
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Var {
                name: "c".to_string(),
            },
            BoolFunc::Var {
                name: "c".to_string(),
            },
            BoolFunc::Var {
                name: "d".to_string(),
            },
        ];

        assert_eq!(get_names(input), vec!["a", "b", "c", "d"]);
    }
    #[test]
    fn test_eval() {
        let func = vec![BoolFunc::Var {
            name: "a".to_string(),
        }];
        let mut values = Vec::<bool>::new();
        let mut lookup = HashMap::<BoolFunc, *mut bool>::new();
        init_lookup_values(&func, &mut lookup, &mut values);
        let mut temp = vec![false, true];
        let const_bool = unsafe { vec![temp.as_mut_ptr().add(0), temp.as_mut_ptr().add(1)] };
        let tree = Node::build_tree(func.clone(), &lookup, &const_bool);
        assert_eq!(Node::eval(tree), false);
        if let Some(ptr) = lookup.get(&BoolFunc::Var {
            name: "a".to_string(),
        }) {
            unsafe {
                assert_eq!(format!("{:?}", ptr), format!("{:?}", values.as_ptr()));
                assert_eq!(ptr.read_volatile(), false);
            }
        }
        assert_eq!(update_values(&mut values), true);
        assert_eq!(Node::eval(tree), true);
        if let Some(ptr) = lookup.get(&BoolFunc::Var {
            name: "a".to_string(),
        }) {
            unsafe {
                assert_eq!(format!("{:?}", ptr), format!("{:?}", values.as_ptr()));
                assert_eq!(ptr.read_volatile(), true);
            }
        }
        assert_eq!(update_values(&mut values), false);
    }

    #[test]
    fn test_single() {
        let output = vec![false, true];
        let input = parse(vec![BoolFunc::Var {
            name: "a".to_string(),
        }]);

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at {}", i);
        }
    }

    #[test]
    fn test_lookup_values_2() {
        let func = vec![
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Var {
                name: "b".to_string(),
            },
        ];
        let mut values = Vec::<bool>::new();
        let mut lookup = HashMap::<BoolFunc, *mut bool>::new();
        init_lookup_values(&func, &mut lookup, &mut values);

        assert_eq!(values, vec![false, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![false, true]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, true]);
        assert_eq!(update_values(&mut values), false);
    }

    #[test]
    fn test_lookup_values_3() {
        let func = vec![
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Var {
                name: "c".to_string(),
            },
        ];
        let mut values = Vec::<bool>::new();
        let mut lookup = HashMap::<BoolFunc, *mut bool>::new();
        init_lookup_values(&func, &mut lookup, &mut values);

        assert_eq!(values, vec![false, false, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![false, false, true]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![false, true, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![false, true, true]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, false, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, false, true]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, true, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, true, true]);
        assert_eq!(update_values(&mut values), false);
    }

    #[test]
    fn test_const_false() {
        let output = parse(vec![BoolFunc::Zero, BoolFunc::And, BoolFunc::One]);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], false);
    }

    #[test]
    fn test_const_true() {
        let output = parse(vec![BoolFunc::One, BoolFunc::And, BoolFunc::One]);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], true);
    }

    #[test]
    fn and() {
        let output = vec![false, false, false, true];
        let input = parse(vec![
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::And,
            BoolFunc::Var {
                name: "b".to_string(),
            },
        ]);

        assert_eq!(input, output);
    }

    #[test]
    fn xor() {
        let output = vec![false, true, true, false];
        let input = parse(vec![
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Xor,
            BoolFunc::Var {
                name: "b".to_string(),
            },
        ]);

        assert_eq!(input, output);
    }

    #[test]
    fn or() {
        let output = vec![false, true, true, true];
        let input = parse(vec![
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Or,
            BoolFunc::Var {
                name: "b".to_string(),
            },
        ]);

        assert_eq!(input, output);
    }

    #[test]
    fn not() {
        let output = vec![true, false];
        let input = parse(vec![
            BoolFunc::Not,
            BoolFunc::Var {
                name: "a".to_string(),
            },
        ]);

        assert_eq!(input, output);
    }

    #[test]
    fn identaty() {
        let output = vec![false, true];
        let input = parse(vec![BoolFunc::Var {
            name: "a".to_string(),
        }]);

        assert_eq!(input, output);
    }

    #[test]
    fn pares_complex_1() {
        // (a|b)&!c -> 0010 1010
        let output = vec![false, false, true, false, true, false, true, false];
        let input = parse(vec![
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Or,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Close,
            BoolFunc::And,
            BoolFunc::Not,
            BoolFunc::Var {
                name: "c".to_string(),
            },
        ]);
        assert_eq!(input, output);
    }

    #[test]
    fn pares_complex_2() {
        //  (a&b&!c)  -> 0000 0010
        let output = vec![false, false, false, false, false, false, true, false];
        let input = parse(vec![
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::And,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::And,
            BoolFunc::Not,
            BoolFunc::Var {
                name: "c".to_string(),
            },
            BoolFunc::Close,
        ]);
        assert_eq!(input, output);
    }

    #[test]
    fn pares_complex_3() {
        // !((a|b)&(c|!d)) -> 1111 0100 0100 0100
        let output = vec![
            true, true, true, true, false, true, false, false, false, true, false, false, false,
            true, false, false,
        ];
        let input = parse(vec![
            BoolFunc::Not,
            BoolFunc::Open,
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Or,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Close,
            BoolFunc::And,
            BoolFunc::Open,
            BoolFunc::Var {
                name: "c".to_string(),
            },
            BoolFunc::Or,
            BoolFunc::Not,
            BoolFunc::Var {
                name: "d".to_string(),
            },
            BoolFunc::Close,
            BoolFunc::Close,
        ]);
        assert_eq!(input, output);
    }

    #[test]
    fn is_parentheses_true() {
        let input = vec![
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::And,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Close,
        ];
        assert_eq!(Node::is_parentheses(&input), true);
    }

    #[test]
    fn is_parentheses_false() {
        let input = vec![
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Close,
            BoolFunc::And,
            BoolFunc::Open,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Close,
        ];
        assert_eq!(Node::is_parentheses(&input), false);
    }

    #[test]
    fn test_split_index() {
        // (a | b) & c
        let input = vec![
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Or,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Close,
            BoolFunc::And,
            BoolFunc::Var {
                name: "c".to_string(),
            },
        ];

        assert_eq!(Node::split_index(&input), 5);
    }

    #[test]
    fn test_split_not() {
        // !(a & b)
        let input = vec![
            BoolFunc::Not,
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::And,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Close,
        ];

        let bundle = Node::split(&input);
        assert_eq!(bundle.center, BoolFunc::Not);
        assert_eq!(
            bundle.left,
            Some(vec![
                BoolFunc::Open,
                BoolFunc::Var {
                    name: "a".to_string(),
                },
                BoolFunc::And,
                BoolFunc::Var {
                    name: "b".to_string(),
                },
                BoolFunc::Close,
            ])
        );
        assert_eq!(bundle.right, None);
    }

    // this is a very imortant function to buid the tree. It splits the function in to 3 parts Example: a & b | (c | d) -> a & b, |, (c | d)
    // this works with any arbitrarily complicated function
    // fn split

    #[test]
    fn test_split_operator() {
        let input = vec![
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Close,
            BoolFunc::And,
            BoolFunc::Open,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Close,
        ];

        let bundle = Node::split(&input);
        assert_eq!(bundle.center, BoolFunc::And);
        assert_eq!(
            bundle.left,
            Some(vec![
                BoolFunc::Open,
                BoolFunc::Var {
                    name: "a".to_string(),
                },
                BoolFunc::Close,
            ])
        );
        assert_eq!(
            bundle.right,
            Some(vec![
                BoolFunc::Open,
                BoolFunc::Var {
                    name: "b".to_string(),
                },
                BoolFunc::Close,
            ])
        );
    }

    #[test]
    fn test_split_parentheses() {
        let bundle = Node::split(&vec![
            BoolFunc::Open,
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::And,
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Close,
        ]);

        assert_eq!(bundle.center, BoolFunc::And);
        assert_eq!(
            bundle.left,
            Some(vec![BoolFunc::Var {
                name: "a".to_string(),
            },])
        );

        assert_eq!(
            bundle.right,
            Some(vec![BoolFunc::Var {
                name: "b".to_string(),
            },])
        );
    }
}
