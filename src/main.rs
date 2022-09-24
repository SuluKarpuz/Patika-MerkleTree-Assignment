// Import Crates
use std::fs;
use sha2::{Sha256, Digest};
fn merkle_root(filename: String) -> String {
    // Read Input Data from txt file
    let file = fs::read_to_string(&filename).expect("Can't read the file");
 
    // Create vector of strings for leaves
    let mut file_vec: Vec<String> = Vec::new();
    for line in file.lines(){
        file_vec.push(line.to_string());
 
    };
    //getting the tree power
    let  a = file_vec[0].parse().unwrap();
    file_vec.remove(0);
    // Hash inputs and append to vector
    let mut hex_vec: Vec<String> = Vec::new();
 
    // Then Write an algorithm that calculates the ROOT
    for x in 0..a{
        hex_vec  = file_vec.iter().map(|x: &String| hash_input(x)).collect();
        file_vec = create_next_level(hex_vec);
 
    }
 
    // Return the root hash as a String
    hash_input(&file_vec[0])

}

//create_next_level function
fn create_next_level(current_level: Vec::<String>) -> Vec::<String> {
    let mut next:Vec<String> =Vec::new();
    let mut str = String::from("");
    for (index, value) in current_level.iter().enumerate() {
        if index % 2 == 0 {
            str = value.clone();
        }
        else {
            str.push_str(&value);
            next.push(str.clone());
        }
    }
    next

}


// function for hashing
fn hash_input(a: &str) -> String {
    let mut hasher = Sha256::new();
    let input = a;
    hasher.update(input);
    let hash = hasher.finalize();
    let hex = hex::encode(&hash);
 
    return hex.to_string();
}

fn main() { 

}



// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = merkle_root("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = merkle_root("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = merkle_root("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = merkle_root("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = merkle_root("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}
