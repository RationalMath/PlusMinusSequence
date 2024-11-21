function generate_plus_minus_sequence(n::Int)
	# Initialize a boolean array to represent the working sequence (numbers 2 to n)
	in_working_sequence = falses(n)
	in_working_sequence[2:n] .= true  # Set indices 2 to n as true

	# Initialize the output sequence
	output_sequence = Int[]

	# Loop until the working sequence is empty
	while any(in_working_sequence)
		# Find and remove the smallest number from the working sequence
		x = findfirst(in_working_sequence)
		in_working_sequence[x] = false

		# Add the number to the output sequence
		push!(output_sequence, x)

		# Define x+1 and x-1
		x_plus_one = x + 1
		x_minus_one = x - 1

		# Loop through the working sequence
		while true
			# Increment x by x_plus_one
			x += x_plus_one

			# Break if x exceeds n
			if x > n
				break
			end

			# Remove x from the working sequence
			in_working_sequence[x] = false

			# Increment x by x_minus_one
			x += x_minus_one

			# Break if x exceeds n
			if x > n
				break
			end

			# Remove x from the working sequence
			in_working_sequence[x] = false
		end
	end

	return output_sequence
end

