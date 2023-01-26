// INTRODUCTION
// Arrow functions allow us to write shorter and cleaner function syntax as compared to regular functions.

let x = function(x, y) {
    return x * y;
}

let x = (x, y) => x * y;


// GENERAL SYNTAX
LET myFunction = (arg1, arg2, arg3, ... argN) => {
    statement(s)
}

// If the function has only one statement, and the statement returns a value, you can remove the brackets and the return keyword:
let myFunction = (arg1, arg2, ...argN) => expression


// EXAMPLE 1: ARROW FUNCTION WITH 
// NO ARGUMENT

//If a funtion doesn't take any argument, the you should use empty parentheses. For example,

let myFun = () => console.log('Hello');
myFun(); //Hello 


//EXAMPLE 2: Arrow Funtion with One Argument 

let myFun = x => console.log(x);
myFun('Hello'); //Hello 
//
//
//this with Arrow Function 
//      inside a regular funtion, this ketword refers to the funtion where it is called.
//      However, this is not associated with arrow functions. Arrow function does not have its own this. So whenever you call this, it refers to its parents scope. For example 
function Student() {
    this.name = 'Ram',
    this.age = 29,
    this.sayName = function (){
        console.log(this.age);
        let innerFunc = () => {
            console.log(this.age);
        }

        innerFunc();
    }
}

const x = new Student();
x.sayName();

// Output : 29
//Here, the innerFunc() functions is defined using the arrow function. And inside the arrow function. theis refers to the parents scope. Hence. this.age gibes 29
//
