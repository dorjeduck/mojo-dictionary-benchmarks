from string_dict import Dict as StringDict
from sys import argv
from time import perf_counter_ns

alias NUM = 1_000_000

fn write_json(file_path:String,program:String,time_sec:Float64,sum:Int) raises:
    var file = open(file_path, "w")
    var out_str = '{' + '"program": ' + '"' + program + '", ' + '"time_sec": ' + String(time_sec) + ', "sum": ' + String(sum) + '}'
    file.write(out_str)
    file.close()

fn append_csv(file_path:String, time_sec:Float64) raises:
    try:
        var file_r = open(file_path, "r")
        contents = file_r.read()
        file_r.close()
    except:
        contents = ""

    # Append elapsed time as a new line
    contents += String(time_sec) + "\n"

    var file_w = open(file_path, "w")
    file_w.write(contents)
    file_w.close()


fn main() raises:

    var keys = List[String](capacity = NUM)
    for i in range(NUM):
        keys[i] = "k"+String(i)

    var start = perf_counter_ns()

    var dic =  StringDict[Int]()

    for i in range(NUM):
        dic.put(keys[i], i % 7)
    for i in range(0, NUM, 2):
        dic.put(keys[i], dic.get(keys[i],0) * 2)

    var sum = 0
    for i in range(NUM):
        sum += dic.get(keys[i],-1)

    var elapsed = (perf_counter_ns() - start) / 1e9

    if len(argv()) >= 2 and argv()[1] == '--nightly':
        write_json('./results/compact_dict_nightly-mojo.json','compact_dict.mojo (nightly)',elapsed,sum)
        append_csv('./results/compact_dict_nightly-mojo.csv', elapsed)
    else:
        write_json('./results/compact_dict-mojo.json','compact_dict.mojo',elapsed,sum)
        append_csv('./results/compact_dict-mojo.csv', elapsed)

    print("Sum: " + String(sum) + ", elapsed sec: " + String(elapsed))
