cargo run -- --code-path [elf file] > actual_reg.txt

python3 reg_checker.py ref actual_reg.txt

python3 autograde.py {test dir}