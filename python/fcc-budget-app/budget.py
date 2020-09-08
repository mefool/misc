''' main.py
food = budget.Category("Food")
food.deposit(1000, "initial deposit")
food.withdraw(10.15, "groceries")
food.withdraw(15.89, "restaurant and more food for dessert")
print(food.get_balance())
clothing = budget.Category("Clothing")
food.transfer(50, clothing)
clothing.withdraw(25.55)
clothing.withdraw(100)
auto = budget.Category("Auto")
auto.deposit(1000, "initial deposit")
auto.withdraw(15)
'''


class Category:
  def __init__(self, name):
    self.name = name
    self.ledger = []

  def deposit(self, amount, description = ""):
    self.ledger.append({"amount": amount,
                        "description": description })

  def withdraw(self, amount, description = ""):
    if self.check_funds(amount):
      self.ledger.append({"amount": amount*(-1), 
                          "description": description })
      return True
    return False

  def transfer(self, amount, cat):
    cat_name = cat.name
    if self.check_funds(amount):
      description = "Transfer to " + cat_name
      self.ledger.append({"amount": amount*(-1), "description": description})
      cat_description = "Transfer from " + self.name
      cat.deposit(amount, cat_description)
      return True
    return False

  def get_balance(self):
    total = 0
    for led in self.ledger:
      total += led["amount"]
    return total

  def get_spent(self):
    total = 0
    for led in self.ledger:
      if (led["amount"] < 0):
        total -= led["amount"]
    return total

  def check_funds(self, amount):
    total = 0
    for led in self.ledger:
      total += led["amount"]
    if amount <= total:
      return True
    return False

  def __str__(self):
    title = self.name.center(30,'*') + '\n'
    items = ""
    total = 0
    for item in self.ledger:
      items += '{:23.23}'.format(item["description"]) + '{:7.2f}'.format(item["amount"]) + '\n'
      total += item["amount"]
    return title + items + "Total: " + str(total)
'''
'''

def create_spend_chart(categories):
  total = sum(cat.get_spent() for cat in categories)
  pc_list = [(cat.get_spent() / total) // 0.01 for cat in categories]

  title = "Percentage spent by category" + "\n"
  
  chart = ""
  for pc in range(100, -10, -10):
    chart += '{:3}'.format(pc) + '|'
    for pc_l in pc_list:
      if pc_l >= pc:
        chart += " o "
      else:
        chart += "   "
    chart += " \n"
  
  sep = ' '*4 + '-'*(len(pc_list)*3) + "-\n"
  
  max_len = max(len(cat.name) for cat in categories)
  names = ""
  for i in range(max_len):
    names += ' '*4
    for cat in categories:
      if i < len(cat.name):
        names += ' ' + cat.name[i] + ' '
      else:
        names += ' '*3
    names += " \n"

  ret = title + chart + sep + names
  return ret.rstrip() + "  "