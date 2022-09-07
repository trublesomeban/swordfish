pub fn stdlib() -> (Vec<String>, Vec<String>) {
	(
	    vec![
		" self ".to_string(),
		" here ".to_string(),
		"do ".to_string()
	    ],
	    vec![
		" as @s ".to_string(),
		" at @s ".to_string(),
		"execute ".to_string()
	    ],
	)
    }