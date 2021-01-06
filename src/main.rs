

fn main() {
    let mut location = vec![false; 10];
    //let mut instructions = vec![String::from("");10];
    let instructions = vec!['a','c','b'];
    let mut instructions_iter = instructions.iter().enumerate();
   
    
    while let Some((l,c)) = instructions_iter.next() {
    match c {
        'a'=> {location[l] = true;},
        'b'=> {location[l] = true;},
        'c'=> {location[l] = true;},
        _=> println!("not a character"),
    }
    }

    

    println!("{:?}", location);
    // println!("{:?}", instructions);


//I need to put each of these instructions into a list instruction list-. Each time I use an instruction, I first need to note down the position which the instruction is in and compare it to another list, location list,  that tells me whether the position has already been used.
// The  location list should be the size of the instruction list. Then each time I use an instruction, I flip the corresponding position in the location list from False to True. This means I can compare the instruction position to the same position in the second vector. If the position in the second vector is true then I can know it has already been used.


// Create the location list and set all values to false. -- done
// Figure out how to set a value to True. -- done
// Figure out how to loop through an iterator and match each char to one of the instructions and to then find the position of the instruction and take that data - done

}