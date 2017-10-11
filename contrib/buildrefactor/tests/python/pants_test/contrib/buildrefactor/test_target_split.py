# TODO test target_split

# Target split should be able to split a BUILD file into multiple target directories
# Perhaps it would make one new target per directory

# Should probably ask Yi for clarification on the idea

# Perhaps an split-factor option

# as an option
# register('split-factor')

# then the goal would be ./pants target-split <path_to_build_file> --split-factor=2

# --split-factor=2 would split in half
# --split-factor=3 would split by 3
# --split-factor=4 would split by 4