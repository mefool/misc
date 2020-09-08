import copy
import random
from collections import Counter as count
# Consider using the modules imported above.

class Hat:
  # Multiple function arguments: https://realpython.com/python-kwargs-and-args/ (unpack dictionary)
  def __init__(self, **kwargs):
    contents = []
    for key, value in kwargs.items():
      for i in range(value):
        contents += key.split()
    self.contents = contents

  # extra sanity check
  def __str__(self):
    sep = ", "
    return sep.join(self.contents)

  def draw(self, num):
    balls_drawn = []
    balls_left = self.contents
    if (num >= len(self.contents)):
      return balls_left

    for i in range(num):
      ball = random.choice(balls_left)
      balls_left.remove(ball)
      balls_drawn.append(ball)

    return balls_drawn

def experiment(hat, expected_balls, num_balls_drawn, num_experiments):
  ex_balls = []
  for key, value in expected_balls.items():
    for i in range(value):
      ex_balls += key.split()
  
  #num_experiments = 6
  cnt = 0;
  for n in range(num_experiments):
    trial = copy.deepcopy(hat)
    draw = trial.draw(num_balls_drawn)
    draw.sort()
    if listContains(ex_balls,draw):
      cnt += 1

  return cnt / num_experiments

def listContains(l1, l2):
  list1 = count(l1)
  list2 = count(l2)

  return list1 & list2 == list1 # bitwise &
