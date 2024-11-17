include("PlusMinusSequence.jl")
using .PlusMinusSequence: generate_plus_minus_sequence
using Statistics
using Plots
using Plots: px	

num_plots = 5
plot_size_x = 600
plot_size_y = 500

main_title_size = 12
sub_title_size = 8

# Create plot layout with 1 row and 2 columns
layout = @layout [a; b; c; d; e]

s100 = generate_plus_minus_sequence(2120)

plot1 = scatter(
	s100,
	marker = :circle,
	title = "First 100 values",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

s0 = generate_plus_minus_sequence(100000)

plot2 = plot(
	s0,
	title = "First $(length(s0)) values",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

big_n = 1_000_000
s1 = generate_plus_minus_sequence(big_n)

padding = 1

slope_avg = mean([s1[i+1] - s1[i] for i in padding:length(s1)-1])

plot3 = plot(
	s1,
	title = "First $(length(s1)) values; avg slope ≈ $(round(slope_avg, digits=2))",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
	linewidth = 3,
	opacity = 0.5,
	label = "Plus-Minus Sequence",
)

line_data = [slope_avg * i for i in 1:length(s1)]
plot!(line_data, label = "y = $(round(slope_avg, digits = 2))x", color = :red)

s3 = generate_plus_minus_sequence(10000)
differences = [s3[i+1] - s3[i] for i in 1:length(s3)-1]

window_size = 1000
diff_moving_avg = [mean(differences[i:i+window_size]) for i in 1:length(differences)-window_size]

plot4 = plot(diff_moving_avg,
	title = "Moving average over first $(length(diff_moving_avg)) differences (window size = $(window_size))",
	xlabel = "nth difference",
	ylabel = "Difference",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
)



plot5 = plot(differences,
	title = "First $(length(differences)) differences",
	xlabel = "nth difference",
	ylabel = "Difference",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
	label = "Difference",
)

constant_line = [12 for i in 1:length(differences)]
plot!(constant_line, label = "y = 12", color = :red, opacity = 0.5, linewidth = 3)

plot(plot1, plot2, plot3, plot4, plot5,
	layout = layout,
	size = (plot_size_x, plot_size_y * 4),
	plot_title = "Plus-Minus Sequence",
	titlefont = (main_title_size, "Helvetica", :blue),
	left_margin = 100px,
	right_margin = 50px,
	bottom_margin = 25px,
)
