using Plots: px
include("PlusMinusSequence.jl")
using .PlusMinusSequence
using Statistics

num_plots = 5
plot_size_x = 600
plot_size_y = 500

main_title_size = 12
sub_title_size = 8

# Create plot layout with 1 row and 2 columns
layout = @layout [a; b; c; d; e]

plot_settings = (
)

s = generate_plus_minus_sequence(2120)

plot1 = scatter(
	s,
	marker = :circle,
	title = "First 100 values",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

s = generate_plus_minus_sequence(100000)

plot2 = plot(
	s,
	title = "First $(length(s)) values",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

big_n = 1_000_000
s = generate_plus_minus_sequence(big_n)

padding = 1

slope_avg = mean([s[i+1] - s[i] for i in padding:length(s)-1])

plot3 = plot(
	s,
	title = "First $(length(s)) values; slope ≈ $(slope_avg)",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
	linewidth = 3,
	opacity = 0.5,
	label = "Plus-Minus Sequence",
)

line_data = [slope_avg * i for i in 1:length(s)]
plot!(line_data, label = "y = $(round(slope_avg, digits = 2))x", color = :red)

differences = [s[i+1] - s[i] for i in 1:length(s)-1]

plot4 = plot(differences,
	title = "First $(length(differences)) differences",
	xlabel = "nth difference",
	ylabel = "Difference",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
	label = "Difference",
)

constant_line = [12 for i in 1:length(differences)]
plot!(constant_line, label = "y = 12", color = :red, opacity = 0.5, linewidth = 3)

window_size = 1000
diff_moving_avg = [mean(differences[i:i+window_size]) for i in 1:length(differences)-window_size]

plot5 = plot(diff_moving_avg,
	title = "Moving average over first $(length(diff_moving_avg)) differences (window size = $(window_size))",
	xlabel = "nth difference",
	ylabel = "Difference",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

plot(plot1, plot2, plot3, plot4, plot5,
	layout = layout,
	size = (plot_size_x, plot_size_y * 4),
	plot_title = "Plus-Minus Sequence",
	titlefont = (main_title_size, "Helvetica", :blue),
	left_margin = 100px,
	right_margin = 50px,
	bottom_margin = 25px,
)
