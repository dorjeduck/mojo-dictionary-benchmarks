from collections import Dict
from sys import argv
from time import perf_counter_ns
 
alias NUM = 1_000_000

fn write_json(file_path:String,program:String,time_sec:Float64,sum:Int) raises:
    var file = open(file_path, "w")
    var out_str = '{' + '"program": ' + '"' + program + '", ' + '"time_sec": ' + String(time_sec) + ', "sum": ' + String(sum) + '}'
    file.write(out_str)
    file.close()


fn main() raises:

    var keys = List[String](capacity = NUM)
    for i in range(NUM):
        keys[i] = "k"+String(i)

    var start = perf_counter_ns()

    var dic = Dict[String,Int]()
    
    for i in range(NUM):
        dic[keys[i]] = i % 7
    for i in range(0,NUM,2):
        dic[keys[i]] *= 2
    var sum_val = 0
    for i in range(NUM):
        sum_val += dic[keys[i]]
  
    var elapsed = (perf_counter_ns()-start)/1e9
    
    if len(argv()) >= 2 and argv()[1] == '--nightly':
        write_json('./results/stdlib_dict_nightly-mojo.json','stdlib_dict.mojo (nightly)',elapsed,sum_val)
    else:    
        write_json('./results/stdlib_dict-mojo.json','stdlib_dict.mojo',elapsed,sum_val)
