import networkx as nx
import numpy as np
import matplotlib.pyplot as plt

matrix = []
data = open("data17.txt").read().split("\n")
for i in range(len(data)):
    temp = []
    for j in range(len(data[i])):
        temp.append(int(data[i][j]))
    matrix.append(temp)


class ConstrainedPathFinder:
    def __init__(self, graph, max_consecutive_steps):
        self.graph = graph
        self.max_consecutive_steps = max_consecutive_steps

    def find_shortest_path(self, start, end):
        # Use your pathfinding algorithm to find the initial shortest path
        shortest_path = nx.shortest_path(self.graph, source=start, target=end)

        # Adjust for consecutive steps
        adjusted_path = [shortest_path[0]]
        consecutive_steps = 1

        for i in range(1, len(shortest_path)):
            if consecutive_steps < self.max_consecutive_steps:
                # Add the current node to the adjusted path
                adjusted_path.append(shortest_path[i])
                consecutive_steps += 1
            else:
                # Skip the current node to limit consecutive steps
                consecutive_steps = 1  # Reset consecutive steps
                # Add the next node to the adjusted path
                adjusted_path.append(shortest_path[i])

        return adjusted_path



matrix = np.array(matrix)

graph = nx.DiGraph(matrix)

max_steps = 3

graph.add_node('start', pos=(0,0))
graph.add_node('end', pos=(len(matrix)-1, len(matrix[0])-1))

graph.add_edge(0,0)
graph.add_edge(len(matrix)-1, len(matrix[0])-1)

path_finder = ConstrainedPathFinder(graph, max_steps)
result_path = path_finder.find_shortest_path('start', 'end')

print(nx.shortest_path(graph, source='start', target='end'))

