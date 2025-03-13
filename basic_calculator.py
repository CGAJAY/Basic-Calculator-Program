# simple calculator program

# Define the calculator function
def calculator():
    # Get user inputs
    num1= float(input("Enter first number: ")) # convert input to float
    num2= float(input("Enter second number: ")) # convert input to float
    operation= input("Enter operation (+, -, *, /): ") # get operation

    # print('first number:', num1)
    # print('second number:', num2)
    # print('operation:', operation)  

    # Perform the operation
    if operation == '+':
        result = num1 + num2
        #f-string to format the output into a string 
        print(f"{num1} + {num2} = {result}") 
    elif operation == '-':
        result = num1 - num2
        print(f"{num1} - {num2} = {result}")
    elif operation == '*':
        result = num1 * num2
        print(f"{num1} * {num2} = {result}")
    elif operation == '/':
        if num2 != 0: # check if num2 is not zero
            result = num1 / num2 
            print(f"{num1} / {num2} = {result}")   
        else: 
            print("Division by zero is not allowed")
    else:
        print("Error: Invalid operation")

# Call the calculator function
calculator()