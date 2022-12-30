# ownership of return values
The ownership of a variable follows the same pattern every time: 
assigning a value to another variable moves it. 
When a variable that includes data on the heap goes out of scope, 
the value will be cleaned up by drop UNLESS ownership of the data has been moved to another variable.

taking ownership and then returning ownership with every function is a bit tedious. 
What if we want to let a function use a value but not take ownership? 
It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, 
in addition to any data resulting from the body of the function that we might want to return as well.

REFERENCE ==> let a function use a value but not take ownership.
