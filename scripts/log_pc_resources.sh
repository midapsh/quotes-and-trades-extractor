#!/bin/bash
echo "date | CPU(%) | RAM(%) | Swap(%) | HDD(%)" >> cpu_ram_logging.log;
while true;
do
    mdate=`date +'%d/%m/%Y %H:%M:%S'`
    mcpu="`LC_ALL=C top -bn1 | grep "Cpu(s)" | sed "s/., *\([0-9.]\)%* id.*/\1/" | awk '{print 100 - $1}'`%"
    mmem=`free -m | awk '/Mem:/ { printf("%3.1f%%", $3/$2*100) }'`
    mswap=`free -m | awk '/Swap:/ { printf("%3.1f%%", $3/$2*100) }'`
    mhdd=`df -h / | awk '/\// {print $(NF-1)}'`
    echo "$mdate | $mcpu | $mmem | $mswap | $mhdd" >> cpu_ram_logging.log;
    sleep 1;
done