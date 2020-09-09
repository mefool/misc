import numpy as np
''' return
{
  'mean': [axis1, axis2, flattened],
  'variance': [axis1, axis2, flattened],
  'standard deviation': [axis1, axis2, flattened],
  'max': [axis1, axis2, flattened],
  'min': [axis1, axis2, flattened],
  'sum': [axis1, axis2, flattened]
}
'''
def calculate(list):
  if (len(list) != 9):
    raise ValueError("List must contain nine numbers.")

  a = np.resize(list, [3,3])
  ret = {}
  ret['mean'] = [np.mean(a, axis=0).tolist(),\
                 np.mean(a, axis=1).tolist(),\
                 np.mean(a).tolist()]
  ret['variance'] = [np.var(a, axis=0).tolist(),\
                     np.var(a, axis=1).tolist(),\
                     np.var(a).tolist()]
  ret['standard deviation'] = [np.std(a, axis=0).tolist(),\
                               np.std(a, axis=1).tolist(),\
                               np.std(a).tolist()]
  ret['max'] = [np.max(a, axis=0).tolist(),\
                np.max(a, axis=1).tolist(),\
                np.max(a).tolist()]
  ret['min'] = [np.min(a, axis=0).tolist(),\
                np.min(a, axis=1).tolist(),\
                np.min(a).tolist()]
  ret['sum'] = [np.sum(a, axis=0).tolist(),\
                np.sum(a, axis=1).tolist(),\
                np.sum(a).tolist()]

  return ret
