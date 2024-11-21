include("PlusMinusSequence.jl")
using Statistics
using Plots
using Plots: px

# Generate the plus-minus sequence
S = generate_plus_minus_sequence(1_000_000)

# Subsets of the sequence
S100 = S[1:100]
Se4 = S[1:10_000]

# Slope average over Se4
slope_avg = mean([Se4[i+1] - Se4[i] for i in 1:length(Se4)-1])

# Constant line for Se4
constant_line_data = [slope_avg * i for i in 1:length(Se4)]

# Differences between consecutive elements of S
differences = [S[i+1] - S[i] for i in 1:1000]
differences2 = [S[i+1] - S[i] for i in 1:length(S)-1]

window_size = 1000
diff_moving_avg = [mean(differences2[i:i+window_size]) for i in 1:length(differences2)-window_size]

sub_title_size = 18

plot1 = scatter(
	S100,
 	marker = :circle,
 	title = "First 100 values",
 	xlabel = "Sequence index",
 	ylabel = "Sequence value",
 	labelfontsize = 8,
 	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

plot2 = plot(
	Se4,	
	title = "First $(length(Se4)) values",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
)

plot3 = plot(
	Se4,
	title = "First $(length(Se4)) values; avg slope ≈ $(round(slope_avg, digits=2))",
	xlabel = "Sequence index",
	ylabel = "Sequence value",
	labelfontsize = 8,
	legend = false,
	titlefont = (sub_title_size, "Helvetica", :blue),
	linewidth = 3,
	opacity = 0.5,
	label = "Plus-Minus Sequence",
)
plot!(constant_line_data, label = "y = $(round(slope_avg, digits = 2))x", color = :red)

plot4 = plot(diff_moving_avg,
	title = "Moving average over first $(length(diff_moving_avg)) differences (window size = $(window_size))",
	xlabel = "nth difference",
	ylabel = "Difference",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
	legend =  false,
)

plot5 = plot(differences,
	title = "First $(length(differences)) differences",
	xlabel = "nth difference",
	ylabel = "Difference",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
	label = "Difference",
	legend = false,
)

plot6 = plot(differences2,
	title = "First $(length(differences2)) differences",
	xlabel = "nth difference",
	ylabel = "Difference",
	labelfontsize = 8,
	titlefont = (sub_title_size, "Helvetica", :blue),
	label = false,
)
hline!([12], label="12", color = :red, linewidth = 2, linestyle = :dash)

num_plots = 5
plot_size_x = 600
plot_size_y = 500

main_title_size = 12

layout = @layout [a; b; c; d; e; f]

plot(plot1, plot2, plot3, plot4, plot5, plot6,
	layout = layout,
	size = (plot_size_x, plot_size_y * 4),
	plot_title = "Plus-Minus Sequence",
	titlefont = (main_title_size, "Helvetica", :blue),
	left_margin = 100px,
	right_margin = 100px,
	bottom_margin = 25px,
)
