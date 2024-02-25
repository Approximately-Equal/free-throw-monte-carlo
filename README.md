# Free Throw Monte Carlo

## The problem statement

Suppose that some people get together to shoot free throws until they can get some F consequtive free throws. The question is how many free throws would it take on average to get those F consequtive shots. 

### Parameters

This problem can be generalized / simplified where there is some probability 0 < p < 1 that a free-throw is made, and each free throw is independent and identically distributed from the bernoulli distribution. 

### The theoretical approach

 This problem can (presumably) be found exactly, under the given framework of course, using theoretical models, but I have yet to calculate this to a level that I am satisfied with. Thus the code that I've written

## Monte Carlo

The monte carlo is a method of getting around exact formulas by just simulating directly and using the law of large numbers to get a good approximation. I've went further than that and made a simulated sampling distribution. 

For this particular application, monte carlo is a good method for just sanity checking whether my thoeretical results are actually close to the truth. 

## How to use

just run cargo run --release, then copy the generated file output.txt into data in the python file and visualize histogram. It's not complex, and in fact is messy because it doesn't matter

## Results

Turns out, for F = 40, p = 0.9, the mean is around 660-670, and the distribution is like 90% between 630, 710 for 10_000 simulations of 1000 samples of shooting until F consequtive shots are made.
