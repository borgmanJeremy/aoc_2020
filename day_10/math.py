import math

minus_sum = 0
for n in range(3,14):
    print(2**n)
    minus_sum += 2**n
print("sum: ",minus_sum)

n = 15 
sum = 0
for r in range(0,n+1):
    res = math.factorial(n) / (math.factorial(r) * math.factorial((n-r)))
    sum +=res 
    print(res)

print(sum)
print(sum-minus_sum)
#
#
#
#n = 4 
#sum = 0
#for r in range(0,n):
#    res = math.factorial(n) / (math.factorial(r) * math.factorial((n-r)))
#    sum +=res 
#    #print(res)
#
#print(sum)