pub mod tomasulo_sim;
fn main() {
    // test instruction
    let input1:[&str;6]=[
        "LD F6 34+ R2",
        "LD F2 45+ R3",
        "MULTD 0 F2 F4",
        "SUBD F8 F6 F2",
        "DIVD F10 F0 F6",
        "ADDD F6 F8 F2"
    ];

    let input2:[&str;8]=[
        "LD F2 0 R2",
        "LD F4 0 R3",
        "DIVD F0 F4 F2",
        "MULTD F6 F0 F2",
        "ADDD F0 F4 F2",
        "SD F6 0 R3",
        "MULTD F6 F0 F2",
        "SD F6 0 R1"
    ];
    
}
