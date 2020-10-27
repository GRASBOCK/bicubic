import matplotlib.pyplot as plt
from matplotlib import cm
from matplotlib.ticker import LinearLocator, FormatStrFormatter
import numpy as np
import sys

# Make data.
filename = sys.argv[1]

data = np.genfromtxt(filename, delimiter=' ')
X = data[:,0]
Y = data[:,1]
Z = data[:,2]

x_steps = 44
y_steps = 81
X = np.reshape(X, (y_steps, x_steps))
Y = np.reshape(Y, (y_steps, x_steps))
Z = np.reshape(Z, (y_steps, x_steps))
print(X)
print(Y)

# Plot the surface.
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

ax.plot_surface(X, Y, Z, cmap=cm.coolwarm, antialiased=False, alpha=0.6, zorder=0)

points = [(-2.5, -4.5, 12.4), (0.0, -4.5, 1.45), (1.5, -4.5, 1.33), (-2.5, 3.2, 13.4), (0.0, 3.2, 13.2), (1.5, 3.2, 6.)]
for px, py, pz in points:
	ax.scatter(px, py, pz, marker='.', color='green',zorder=10)

ax.set_xlabel('X')
ax.set_ylabel('Y')
ax.set_zlabel('Z')

plt.title("fx, fy, fyx not 0")

plt.show()