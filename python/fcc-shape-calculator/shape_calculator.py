class Rectangle:
  def __init__(self, width, height):
    self.w = width
    self.h = height
  
  def __str__(self):
    return(("Rectangle(width={}, height={})").format(self.w, self.h))

  def set_width(self, w):
    self.w = w

  def set_height(self, h):
    self.h = h

  def get_area(self):
    return (self.w * self.h)

  def get_perimeter(self): 
    return (2 * self.w + 2 * self.h)

  def get_diagonal(self):
    return ((self.w ** 2 + self.h ** 2) ** .5)

  def get_picture(self):
    if self.w > 50 or self.h > 50:
      return("Too big for picture.")
    else:
      return( ( ("*" * self.w)+ "\n" ) * self.h)

  def get_amount_inside(self,shape):
    return self.get_area() // shape.get_area()

class Square(Rectangle):
  def __init__(self, side):
    Rectangle.w = side
    Rectangle.h = side

  def set_side(self, value):
    Rectangle.set_width(self, value)
    Rectangle.set_height(self, value)

  def __str__(self):
    return(("Square(side={})").format(self.w) )