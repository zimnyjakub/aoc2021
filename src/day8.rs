use std::fs;

pub fn day8() {

    let string = fs::read_to_string("SEVEN.txt").unwrap();
    let lines= string.lines();


    //  0:      1:      2:      3:      4:
    //  aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    //  ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    //  gggg    ....    gggg    gggg    ....
    //
    //   5:      6:      7:      8:      9:
    //  aaaa    aaaa    aaaa    aaaa    aaaa
    // b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    //  dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    //  gggg    gggg    ....    gggg    gggg

   //  let mut signal_patterns = Vec::new();
   //  let mut output_value = Vec::new();
   // for line in lines {
   //     let parts: Vec<&str> = line.split('|').collect();
   //     signal_patterns.push(
   //         parts[0]
   //             .split(' ')
   //             .map(|it| it.trim())
   //             .filter(|it| !it.is_empty())
   //             .collect()
   //     );
   //
   // }
   //
   //  // let signal_patterns: Vec<&str> =
   //
   //  let output_value: Vec<&str> = notes[1]
   //      .split(' ')
   //      .map(|it| it.trim())
   //      .filter(|it| !it.is_empty())
   //      .collect();
   //
   //  println!("{:?} | {:?}", signal_patterns, output_value);


}