using Plots
include("PlusMinusSequence.jl")
using .PlusMinusSequence

num_plots = 4
plot_size_x = 600
plot_size_y = 500

main_title_size = 12
sub_title_size = 8

# Create plot layout with 1 row and 2 columns
layout = @layout [a; b; c; d]

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

plot3 = plot(
	s,
	title = "First $(length(s)) values; slope ≈ $((s[end] - s[1]) / big_n)",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

differences = [s[i+1] - s[i] for i in 1:length(s)-1]

plot4 = plot(differences,
	title = "First $(length(differences)) differences",
	xlabel = "nth difference",
	ylabel = "Difference",
	labelfontsize = 8,
	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

plot(plot1, plot2, plot3, plot4,
	layout = layout,
	size = (plot_size_x, plot_size_y * 4),
	plot_title = "Plus-Minus Sequence",
	titlefont = (main_title_size, "Helvetica", :blue),
)
