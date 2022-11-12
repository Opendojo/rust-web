ab -n 5000 -c 100 -g doc/v01-hello-world.tsv http://127.0.0.1:7878/hello-world.html
ab -n 5000 -c 100 -g doc/v01-hello-universe.tsv http://127.0.0.1:7878/hello-universe.html
gnuplot scripts/ab-plot-v01.p
