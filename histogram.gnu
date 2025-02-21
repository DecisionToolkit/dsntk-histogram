set terminal pngcairo size 1200,500 font "Helvetica,12"
set output 'histogram.png'

set style data histograms
set style fill solid border -1
set boxwidth 2.5

set xlabel "Percentage of benchmarks"
set ylabel "microseconds"
set xtics rotate by -80
set ytics
set title "DecisionToolkit v0.1.0"

plot "data.txt" using 2:xtic(1) title "Execution time"
