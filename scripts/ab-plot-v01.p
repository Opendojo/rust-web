# output as png image
set terminal png

# save file to "hello.png"
set output "doc/v01-hello.png"

#nicer aspect ratio for image size
set size 1,1

# y-axis grid
set grid y

#x-axis label
set xlabel "request"

#y-axis label
set ylabel "response time (ms)"

#plot data from "out.data" using column 9 with smooth sbezier lines
plot "doc/v01-hello-world.tsv" using 9 smooth sbezier with lines title "mono-threaded version 01 - world" \
  ,"doc/v01-hello-universe.tsv" using 9 smooth sbezier with lines title "mono-threaded version 01 - universe"