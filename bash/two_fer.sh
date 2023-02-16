
#  ~ the below code is to understand the  use of "$1" which varies  for functions and just variables
# print(){

#     echo $1
# }

# print holy
# print $1
# echo $1

#the above implementation has issues with arguments with space and also multiple arguments with space in each

# Check if the name argument is provided , -z checks if the first arg is empty or not
if [ -z "$1" ]
then
  # If the name argument is not provided, print "One for you, one for me."
  echo "One for you, one for me."
else
  # If the name argument is provided, print "One for [name], one for me."
  echo "One for $1, one for me."
fi