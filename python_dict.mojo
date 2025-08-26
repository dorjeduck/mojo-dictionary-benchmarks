from sys import argv
from python import Python
from time import perf_counter_ns

alias NUM = 1_000_000

fn write_json(file_path:String, program:String, time_sec:Float64, sum:Int) raises:
    var file = open(file_path, "w")
    var out_str = '{' + '"program": ' + '"' + program + '", ' + '"time_sec": ' + String(time_sec) + ', "sum": ' + String(sum) + '}'
    file.write(out_str)
    file.close()

fn append_csv(file_path:String, time_sec:Float64) raises:
    # Try to read existing file
    try:
        var file_r = open(file_path, "r")
        contents = file_r.read()
        file_r.close()
    except:
        # File does not exist yet, start empty
        contents = ""

    # Append new line (always add newline)
    contents += String(time_sec) + "\n"

    # Write back
    var file_w = open(file_path, "w")
    file_w.write(contents)
    file_w.close()

fn main() raises:

    var keys = List[String](capacity = NUM)
    for i in range(NUM):
        keys[i] = String(i)

    var start = perf_counter_ns()

    var dic = Python.dict()

    for i in range(NUM):
        dic[keys[i]] = i % 7
    for i in range(0, NUM, 2):
        dic[keys[i]] *= 2

    var sum_val: Int = 0
    for i in range(NUM):
        sum_val += Int(dic[keys[i]])

    var elapsed = (perf_counter_ns() - start) / 1e9

    var json_file:String
    var csv_file:String
    if len(argv()) >= 2 and argv()[1] == '--nightly':
        json_file = './results/python_dict_nightly-mojo.json'
        csv_file = './results/python_dict_nightly-mojo.csv'
        write_json(json_file, 'python_dict.mojo (nightly)', elapsed, sum_val)
    else:
        json_file = './results/python_dict-mojo.json'
        csv_file = './results/python_dict-mojo.csv'
        write_json(json_file, 'python_dict.mojo', elapsed, sum_val)

    # Append elapsed time to CSV
    append_csv(csv_file, elapsed)

    print("Sum: " + String(sum_val) + ", elapsed sec: " + String(elapsed))
