def add_time(start, duration, day = "None"):
  # print(add_time("11:06 PM", "2:02"))
  # start = "11:06 PM"
  # duration = "24:02"
  # day = "monday"
  
  # parse
  s = start.replace(":", " ").split()
  start_time = int(s[0])*60 + int(s[1]) + 12*60 * (s[2] == "PM")
  d = duration.split(":")
  duration_time = int(d[0])*60 + int(d[1])

  # add minutes
  end_time = start_time + duration_time 

  # 24 hour system (from the minutes added)
  n = end_time // (24*60)
  h = end_time % (24*60) // 60
  m = end_time % (24*60) % 60
  c = ["AM","PM"]

  # 24 to 12 hour system conversion
  if h == 0:
    h = 12
    c_i = 0 
  elif (h > 0 and h < 12):
    c_i = 0
  elif h == 12:
    h = 12
    c_i = 1
  elif (h > 12 and h < 24):
    h -= 12 
    c_i = 1


  # days later
  day_later = ''
  if (n == 1):
    day_later = " (next day)"
  elif (n > 1):
    day_later = " (" + str(n) + " days later)"

  # weekday
  weekday = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]
  weekday_str = ''
  weekday_i = index_of(day.lower(),[x.lower() for x in weekday])
  if (weekday_i > 0):
    weekday_str = ", " + weekday[(weekday_i+n)%len(weekday)]

  # join sub-problems
  ret = str(h) + ":" + str(m).rjust(2,"0") \
  + " " + c[c_i] \
  + weekday_str \
  + day_later
  return ret

# auxiliary function
def index_of(val, in_list):
  try:
    return in_list.index(val)
  except ValueError:
    return -1 