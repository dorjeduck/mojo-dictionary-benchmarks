from string_dict import Dict as StringDict
from sys import argv
from time import now

alias NUM = 1_000_000

fn write_json(file_path:String,program:String,time_sec:Float64,sum:Int) raises:
    var file = open(file_path, "w")
    var out = '{' + '"program": ' + '"' + program + '", ' + '"time_sec": ' + str(time_sec) + ', "sum": ' + str(sum) + '}'
    file.write(out)
    file.close()

fn main() raises:

    var start = now()
    
    var dic =  StringDict[Int]()

    for i in range(NUM):
        dic.put(str(i*2),i%7)
    for i in range(0,NUM,2):
        var val = dic.get(str(i*2),0)
        dic.put(str(i*2),val*2)   

    var sum = 0
    for i in range(NUM):
        sum += dic.get(str(i*2),-1)

    var elapsed = (now()-start)/1e9 
 
    if len(argv()) >= 2 and argv()[1] == '--nightly':
        write_json('./results/compact_dict_nightly-mojo.json','compact_dict.mojo (nightly)',elapsed,sum)
    else:    
        write_json('./results/compact_dict-mojo.json','compact_dict.mojo',elapsed,sum)