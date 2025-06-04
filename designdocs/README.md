# Design Docs
I'm a strong proponent of design docs for project organization.

This folder contains all documents pertaining to the software architecture of vitalvault. 

Any deviations from the outlines of these documents must be noted in both the source file and commit message explicitly.

---

A good design doc has the following qualities:

## Requirements
### Readability
A design doc that nobody can read and/or understand is worse than useless.


### Pictures 
Simple diagrams can greatly clarify intent.

### Accountability

#### Individual
Please write down who is the point person for each subsystem outlined in the document. This will be crucial for knowing who to talk to when something else (that maybe depends on or is required by some system) breaks.

#### Sources [src file names, blog/paper references]
Keeping track of where we found ideas is important in case we need to decide whether or not they are justfied later.

### Testing Plan
I don't care if you write tests before or after the functionality. I do care that you decide what is being tested before you write the tests. Remember: **a bad test is *even worse* than a missed test**

### Security Statement
Just think of at least 2 ways the security of the system could be compromised if naively implemented and state that the code won't do that. This is really important for when someone later tries to "optimize" strange looking code without realizing it exists to maintain the security of the platform.

---

written by thelonious cooper