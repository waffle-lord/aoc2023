use std::collections::HashMap;

pub fn get_map(input: &Vec<String>) -> HashMap<String, (String,String)> {

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in input.iter() {

        if line.contains("=") {
            let data: Vec<&str> = line.split("=").collect();
            let location = data[0].trim();
            let left_right_options = data[1];
            let left_right_options: Vec<&str> = left_right_options
                .trim()
                .strip_prefix("(").unwrap()
                .strip_suffix(")").unwrap()
                .split(',').collect();

            map.insert(location.to_string(), 
                       (
                           left_right_options[0].trim().to_string(), 
                           left_right_options[1].trim().to_string()
                       ));
        }
    }

    map
}
















