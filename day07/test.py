import networkx as nx

with open("input.txt") as input:
	graph = nx.DiGraph()
	for line in input:
		v = line.split(" ")
		graph.add_edge(v[1], v[7])

	print('Solution A: ' + ''.join(nx.lexicographical_topological_sort(graph)))