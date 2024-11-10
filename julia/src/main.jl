using Plots
using Statistics
using StatsBase
include("PlusMinusSequence.jl")
using .PlusMinusSequence

# s100 = generate_plus_minus_sequence(2120)

# scatter(
# 	s100,
# 	marker = :circle,
# 	title = "Plus-Minus Sequence up to $(length(s3)) values",
# 	xlabel = "Sequence index",
# 	ylabel = "Sequence value",
# 	legend = false,
# )

s1m = generate_plus_minus_sequence(100000)
s1m = filter(x -> x % 2 != 0, s1m)


# plot(
# 	s1m,
# 	title = "Plus-Minus Sequence up to $(length(s1m)) values",
# 	xlabel = "Sequence index",
# 	ylabel = "Sequence value",
# 	legend = false,
# )

# Calculate differences between consecutive entries
differences = diff(s1m)


# # Find the most common difference value
# most_common_diff = mode(differences)
# println("Most common difference value: ", most_common_diff)




# Plot the differences
plot(
	differences,
	title = "Differences in Plus-Minus Sequence up to $(length(s1m))",
	xlabel = "Index",
	ylabel = "Difference",
	legend = false,
)
