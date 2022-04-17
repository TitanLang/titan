# The Basics
In this tutorial we will teach you some more of the basics<br>
and this is where things get slightly more complicated.<br><br>

## Variables
Variables can store data with a certain name you give the<br>
variable. Here is an example.
```
var pi = 3.14159
```
Here we create a variable called pi and we give it the value<br>
3.14159 and now if we want use it anywhere else we simple just do
```
var pi = 3.14159
pi + pi
```
Now here what we do is we add pi+pi and we alread know the pi is<br>
equal to 3.14159 so we can think of it as 3.14159+3.14159 which gives<br>
the result 6.28318.<br>
If we want to give a variable a specific type we do this
**NOT IMPLEMENTED YET BELOW**
```
var test: i8 = 123
```
Now i8 means 8-bit integer and each type of int has it's own limit<br>
there are also unsigned ints and they are positive numbers only and<br>
no negative number. There many types of int such as 
```
i8, i16, i32, i64
u8, u16, u32, u64
```
<br><br>

## Functions
Functions are fairly simple to understand. If you want to make one<br>
you use the keyword **func** like this
``` 
func test() 
```
Now when creating a function you have to make it before it is referenced<br>
like so
```
func test()
  print("Hello World")
end

func main()
  test()
end
```
All functions must be closed with an *end*.
