To run a single test case:
cargo run -- --code-path [elf file] > actual_reg.txt

To confirm register dump for a single test case (only works on single threaded code):
python3 reg_checker.py ref actual_reg.txt

To run a group of testcases:
python3 autograde.py {test dir}

To run a sweep (parameters and test case are in python file):
python3 sweep.py