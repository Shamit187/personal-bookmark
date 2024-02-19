# all additive partitions of 5

# 1 + 1 + 1 + 2
# 1 + 1 + 2 + 1
# 1 + 2 + 1 + 1
# 2 + 1 + 1 + 1
# 1 + 1 + 3
# 1 + 3 + 1
# 3 + 1 + 1
# 1 + 2 + 2
# 2 + 1 + 2
# 2 + 2 + 1
# 1 + 4
# 4 + 1
# 2 + 3
# 3 + 2

partitions = [
    [1, 1, 1, 2],
    [1, 1, 2, 1],
    [1, 2, 1, 1],
    [2, 1, 1, 1],
    [1, 1, 3],
    [1, 3, 1],
    [3, 1, 1],
    [1, 2, 2],
    [2, 1, 2],
    [2, 2, 1],
    [1, 4],
    [4, 1],
    [2, 3],
    [3, 2]
]

total_items = 16
select_partitions = []

current_items = 0
import random
while current_items < total_items:
    partition = random.choice(partitions)
    current_items += len(partition)
    select_partitions.append(partition)

print(select_partitions)

# Output:
# [[2, 2, 1], [1, 3, 1], [1, 3, 1], [2, 1, 2], [1, 1, 3], [2, 1, 2]]