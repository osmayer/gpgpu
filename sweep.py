import os
import re
import csv

testcase = "instruction_tests/add.elf"
dest_file = "actual_reg.txt"
cycles_file = "total_cycles.csv"
cycles_no_dispatch = "cycles_no_dispatch.csv"
warp_occupancy_used = "warp_occupancy_used.csv"
warp_occupancy_avail = "warp_occupancy_avail.csv"
thread_occupancy_used = "thread_occupancy_used.csv"
thread_occupancy_avail = "thread_occupancy_avail.csv"
memory_requests = "memory_requests.csv"
wake_cycles = "wake_cycles.csv"

# num_blocks = [1, 4, 16, 32, 64]
# threads_per_warp = [1, 4, 16, 32, 64]
# warps_per_block = [1, 4, 16, 32, 64]
# memory_delay = [1, 64, 128]
# functional_units = [1, 4, 8, 64]
# schedulers = [0, 1, 2]

num_blocks = [1, 2]
threads_per_warp = [1, 2]
warps_per_block = [1, 2]
memory_delay = [1, 2]
functional_units = [1, 2]
schedulers = [0, 1]


def get_values(string):
    pattern = fr"{string}(\d+)" # Matches 'Price:' followed by a decimal number
    with open(dest_file, 'r') as file:
        content = file.read()
        match = re.search(pattern, content)
        if match:
            return match.group(1)
        

def parse_file():
    cycles = get_values("Total Cycles Elapsed:")

if __name__ == '__main__':
    # Specify the path to the file you want to modify
    overall_cycles_array = []
    cycles_no_dispatch_array = []
    warp_occupancy_used_array = []
    warp_occupancy_avail_array = []
    thread_occupancy_used_array = []
    thread_occupancy_avail_array = []
    memory_requests_array = []
    wake_cycles_array = []

    for scheduler in schedulers:
        scheduler_cycles_array = [];
        scheduler_cycles_no_dispatch_array = []
        scheduler_warp_occupancy_used_array = []
        scheduler_warp_occupancy_avail_array = []
        scheduler_thread_occupancy_used_array = []
        scheduler_thread_occupancy_avail_array = []
        scheduler_memory_requests_array = [];
        scheduler_wake_cycles_array = []
        for blocks in num_blocks:
            for warps in warps_per_block:
                for threads in threads_per_warp:
                    for delay in memory_delay:
                        for units in functional_units:
                            print(f"working on n{blocks} w{warps} t{threads} m{delay} f{units} s{scheduler}")
                            os.system(f"cargo run --release -- --code-path {testcase} -n {blocks} -w {warps} -t {threads} -m {delay} -f {units} -s {scheduler} > {dest_file}")
                            scheduler_cycles_array.append(get_values("Total Cycles Elapsed: "))
                            scheduler_cycles_no_dispatch_array.append(get_values("Cycles without any instructions dispatched: "))
                            scheduler_warp_occupancy_used_array.append(get_values("Warp occupancy rate slots used: "))
                            scheduler_warp_occupancy_avail_array.append(get_values("Warp occupancy rate slots available: "))
                            scheduler_thread_occupancy_used_array.append(get_values("Thread Occupancy Rate slots used: "))
                            scheduler_thread_occupancy_avail_array.append(get_values("Thread Occupancy Rate slots available: "))
                            scheduler_memory_requests_array.append(get_values("Total Memory Requests: "))
                            scheduler_wake_cycles_array.append(get_values("Total Cycles with Wake: "))

        overall_cycles_array.append(scheduler_cycles_array)
        cycles_no_dispatch_array.append(scheduler_cycles_no_dispatch_array)
        warp_occupancy_used_array.append(scheduler_warp_occupancy_used_array)
        warp_occupancy_avail_array.append(scheduler_warp_occupancy_avail_array)
        thread_occupancy_used_array.append(scheduler_warp_occupancy_used_array)
        thread_occupancy_avail_array.append(scheduler_warp_occupancy_avail_array)

        memory_requests_array.append(scheduler_memory_requests_array)
        wake_cycles_array.append(scheduler_wake_cycles_array)
    
    with open(cycles_file, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(overall_cycles_array)
    with open(cycles_no_dispatch, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(cycles_no_dispatch_array)
    with open(warp_occupancy_used, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(warp_occupancy_used_array)
    with open(warp_occupancy_avail, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(warp_occupancy_avail_array)
    with open(thread_occupancy_used, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(thread_occupancy_used_array)
    with open(thread_occupancy_avail, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(thread_occupancy_avail_array)
    with open(memory_requests, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(memory_requests_array)
    with open(wake_cycles, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(wake_cycles_array)

