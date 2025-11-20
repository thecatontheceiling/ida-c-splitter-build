num_samples = 500

with open("output/__header.h", "r") as infile:
    lines = infile.readlines()

total_lines = len(lines)
if total_lines == 0:
    sampled_lines = []
else:
    step = max(total_lines // num_samples, 1)
    sampled_indices = [i for i in range(0, total_lines, step)]
    sampled_lines = [lines[i] for i in sampled_indices[:num_samples]]

with open("types_sample.txt", "w") as outfile:
    outfile.writelines(sampled_lines)
