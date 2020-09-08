def arithmetic_arranger(problems,showResults = False):
  problem = []
  i = 0;
  # parse
  for div in problems:
    problem.append(div.split(' '));
    if (not problem[i][0].isdigit() or not problem[i][2].isdigit()):
      return "Error: Numbers must only contain digits."

    # calculation (can be done later)
    if showResults == True:
      if problem[i][1] == '+':
        problem[i].append(str(int(problem[i][0]) + int(problem[i][2])))
      elif problem[i][1] == '-':
        problem[i].append(str(int(problem[i][0]) - int(problem[i][2])))
    
    if (problem[i][1] != '+' and problem[i][1] != '-'):
        return "Error: Operator must be '+' or '-'."
    i+=1
    if (i > 5):
      return "Error: Too many problems."
    
  
  arranged_problems = [[],[],[]]
  if (showResults == True):
    arranged_problems.append([])

  # formating
  for prob in problem:
    # auxiliary
    ln = max([len(prob[0]),len(prob[2])])
    if ln > 4:
      return "Error: Numbers cannot be more than four digits."
    # space formating
    a = prob[0].rjust(ln + 2,' ')
    b = prob[2].rjust(ln + 1,' ')
    b = prob[1][0] + b;
    if (showResults == True):
      result = prob[3].rjust(ln + 2,' ')

    #
    arranged_problems[0].append(a);
    arranged_problems[1].append(b);
    arranged_problems[2].append('-' * (ln+2))
    if (showResults == True): 
      arranged_problems[3].append(result);

  for i in range(len(arranged_problems)):
    arranged_problems[i] = "    ".join(arranged_problems[i])
 
  #return arranged_problems
  return "\n".join(arranged_problems)